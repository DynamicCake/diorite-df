use crate::{
    ast::AstRoot,
    common::prelude::*,
    dump::ActionDump,
    error::syntax::{LexerError, UnexpectedEOF, UnexpectedToken},
    parser::Parser,
    tree::TreeRoot,
};

use std::{
    hash::{Hash, Hasher},
    path::Path,
    sync::Arc,
};

use lasso::{RodeoResolver, Spur, ThreadedRodeo};
use logos::{Lexer, Logos};
use rustc_hash::FxHasher;
use tokio::{
    fs::{read_to_string, File},
    io::{self, AsyncReadExt},
};

#[derive(thiserror::Error, Debug)]
pub enum ProjectFileResolveError {
    /// The path: Spur cannot be found in the rodeo
    #[error("Cannot resolve spur")]
    CannotResolve,
    /// The path is relative, input absolute paths
    #[error("Path is relative")]
    Relative,
    /// The file specificed in the project could not be found
    #[error("Path is not in project")]
    NotInProject,
}

/// Immutable resources used for the project not including the source
#[derive(Debug)]
pub struct ProjectResources {
    pub rodeo: RodeoResolver,
    pub project_root: Spur,
    pub actiondump: ActionDump,
}

#[derive(Debug)]
pub struct Project<T: ProjectFiles> {
    pub resources: Arc<ProjectResources>,
    /// The files that the project has
    pub files: T,
    /// A hash comprised of the project metadata and all the source files
    pub hash: u64,
}

impl Project<ParsedProjectFiles> {
    pub async fn create_project(
        rodeo: ThreadedRodeo,
        files: Vec<ProjectFile<RawFile>>,
        actiondump_path: Box<Path>,
    ) -> Result<Project<ParsedProjectFiles>, ProjectCreationError> {
        // Start the action dump reading early
        let actiondump = tokio::spawn(Self::get_actiondump(actiondump_path));

        // Hash files
        let mut hasher = FxHasher::default();
        for file in &files {
            file.hash.hash(&mut hasher);
        }
        let hash = hasher.finish();

        let mut handles = Vec::new();
        let rodeo = Arc::new(rodeo);
        let mut root: Option<Spur> = None;
        for file in files {
            if let Some(root) = root {
                if root != file.resolution.root {
                    return Err(ProjectCreationError::RootsDoNotMatch {
                        root,
                        file: file.resolution.root,
                    });
                }
            } else {
                root = Some(file.resolution.root);
            }
            let rodeo = rodeo.clone();
            let handle = tokio::spawn(async move {
                let src = rodeo.resolve(&file.src);
                let lexer = Token::lexer_with_extras(src, rodeo.clone());
                let tree = Parser::parse(lexer, file.path);
                (tree, file)
            });
            handles.push(handle);
        }
        let root = if let Some(it) = root {
            it
        } else {
            return Err(ProjectCreationError::NoFilesInputed);
        };

        // Async clojures are unstable
        let mut trees = Vec::new();
        let mut lex_errs = Vec::new();
        let mut parse_errs = Vec::new();
        let mut eof_errs = Vec::new();
        for handle in handles {
            let (mut tree, file) = handle.await.expect("Thread failed to execute");
            lex_errs.append(&mut tree.lex_errs);
            parse_errs.append(&mut tree.parse_errs);
            if let Some(eof) = tree.at_eof {
                eof_errs.push(*eof);
            }
            let file = file.to_parsed(tree.root);
            trees.push(file);
        }

        let rodeo = Arc::try_unwrap(rodeo).expect("The Arced ThreadedRodeo has escaped this scope");
        let actiondump = actiondump
            .await
            .expect("Task cannot panic or be canceled")
            .map_err(ProjectCreationError::ActionDump)?;

        let resources = ProjectResources::new(rodeo.into_resolver(), root, actiondump);
        let files = ParsedProjectFiles::new(trees, lex_errs, parse_errs, eof_errs);

        Ok(Self {
            resources: Arc::new(resources),
            files,
            hash,
        })
    }

    async fn get_actiondump(path: Box<Path>) -> Result<ActionDump, ActionDumpReadError> {
        let dump = match read_to_string(&path).await {
            Ok(t) => t,
            Err(e) => return Err(ActionDumpReadError::Io(path, e)),
        };
        // I *could* make this a match statement, but I wont
        let dump = serde_json::from_str(&dump).map_err(|e| ActionDumpReadError::Parse(path, e))?;
        Ok(dump)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ActionDumpReadError {
    #[error("Io error when reading actiondump at {0} with error {1}")]
    Io(Box<Path>, io::Error),
    #[error("Parse error in actiondump at {0} with error {1}")]
    Parse(Box<Path>, serde_json::Error),
}

#[derive(Debug)]
pub enum ProjectCreationError {
    RootsDoNotMatch { root: Spur, file: Spur },
    NoFilesInputed,
    ActionDump(ActionDumpReadError),
}

impl ProjectResources {
    pub fn new(rodeo: RodeoResolver, project_root: Spur, actiondump: ActionDump) -> Self {
        Self {
            rodeo,
            project_root,
            actiondump,
        }
    }

    pub fn rodeo(&self) -> &RodeoResolver {
        &self.rodeo
    }

    pub fn project_root(&self) -> Spur {
        self.project_root
    }
}

#[derive(Debug)]
pub struct ProjectFile<S: FileResolution> {
    pub src: Spur,
    /// Relative path of the file from the project root
    pub path: Spur,
    pub hash: u64,
    pub resolution: S,
}

impl ProjectFile<RawFile> {
    /// # Safety
    /// Hash the file correctly.
    /// Marked unsafe because there is no guarantee that the paths are correct
    pub unsafe fn raw(src: Spur, path: Spur, hash: u64, root: Spur) -> Self {
        Self {
            src,
            path,
            hash,
            resolution: RawFile { root },
        }
    }
    pub async fn read(
        path: &Path,
        root: Spur,
        resolver: Arc<ThreadedRodeo>,
    ) -> Result<Self, ProjectFileCreationError> {
        // Open and verify
        let mut file = File::open(path)
            .await
            .map_err(ProjectFileCreationError::Io)?;

        let canonical = path.canonicalize().map_err(ProjectFileCreationError::Io)?;
        if !canonical.is_absolute() {
            return Err(ProjectFileCreationError::BaseNotAbsolute);
        }
        let path = if let Ok(it) = canonical.strip_prefix(Path::new(resolver.resolve(&root))) {
            if let Some(it) = it.to_str() {
                it
            } else {
                return Err(ProjectFileCreationError::NotUTF8);
            }
        } else {
            return Err(ProjectFileCreationError::NotInRoot);
        };

        // Read file
        let mut src = String::new();
        file.read_to_string(&mut src)
            .await
            .map_err(ProjectFileCreationError::Io)?;

        // Obtain hash
        let mut hasher = FxHasher::default();
        src.hash(&mut hasher);
        let hash = hasher.finish();

        let src = resolver.get_or_intern(src);
        let path = resolver.get_or_intern(path);
        Ok(Self {
            src,
            path,
            hash,
            resolution: RawFile { root },
        })
    }

    pub fn to_parsed(self, parsed_file: TreeRoot) -> ProjectFile<TreeFile> {
        ProjectFile {
            src: self.src,
            path: self.path,
            hash: self.hash,
            resolution: TreeFile::new(parsed_file),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectFileCreationError {
    #[error("{0}")]
    Io(io::Error),
    #[error("File path is not in project root")]
    NotInRoot,
    #[error("Project root path not absolute")]
    BaseNotAbsolute,
    #[error("File path is not utf-8")]
    NotUTF8,
}

// Files

pub trait FileResolution {}

// None
#[derive(Debug, PartialEq)]
pub struct RawFile {
    pub root: Spur,
}
impl FileResolution for RawFile {}

// Lexing
#[derive(Debug)]
pub struct TokenizedFile<'src> {
    lexer: Lexer<'src, Token>,
}
impl FileResolution for TokenizedFile<'_> {}

// Parsing
#[derive(Debug, PartialEq)]
pub struct TreeFile {
    pub root: TreeRoot,
}

/// This is named like (Parse)TreeFile because ParsedFile also refers to
/// parse tree + errors that is returned from parser
impl TreeFile {
    pub fn new(program: TreeRoot) -> Self {
        Self { root: program }
    }
}
impl FileResolution for TreeFile {}

// Semantic Checking
#[derive(Debug)]
pub struct AnalyzedFile<'resources> {
    starters: StarterSet,
    programs: CheckedProjectFiles<'resources>,
}
impl FileResolution for AnalyzedFile<'_> {}

/// Used for the type state pattern with project
pub trait ProjectFiles {}

/// A program state with a parse tree
#[derive(Debug)]
pub struct ParsedProjectFiles {
    pub lex_errs: Vec<LexerError>,
    pub parse_errs: Vec<UnexpectedToken>,
    pub eof_errs: Vec<UnexpectedEOF>,
    pub parsed: Vec<ProjectFile<TreeFile>>,
}

impl ParsedProjectFiles {
    pub fn new(
        files: Vec<ProjectFile<TreeFile>>,
        lex_errs: Vec<LexerError>,
        parse_errs: Vec<UnexpectedToken>,
        eof_errs: Vec<UnexpectedEOF>,
    ) -> Self {
        Self {
            parsed: files,
            lex_errs,
            parse_errs,
            eof_errs,
        }
    }
}
impl ProjectFiles for ParsedProjectFiles {}

#[derive(Debug, PartialEq)]
pub struct CheckedProjectFiles<'d> {
    pub programs: Vec<AstRoot<'d>>,
}
impl ProjectFiles for CheckedProjectFiles<'_> {}
