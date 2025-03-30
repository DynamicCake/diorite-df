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

use super::{parsed::TreeFile, FileResolution, Project, ProjectFile, ProjectFiles, ProjectResources};
use lasso::{RodeoResolver, Spur, ThreadedRodeo};
use logos::{Lexer, Logos};
use rustc_hash::FxHasher;
use tokio::{
    fs::{read_to_string, File},
    io::{self, AsyncReadExt},
};

pub struct NoProjectFiles;

impl ProjectFiles for NoProjectFiles {}

impl Project<NoProjectFiles> {
    // The reason why this was seperated from parsing is for testing reasons
    pub async fn create_project(
        rodeo: ThreadedRodeo,
        files: Vec<ProjectFile<RawFile>>,
        actiondump_path: Box<Path>,
    ) -> Result<Project<RawProjectFiles>, ProjectCreationError> {
        // Start the action dump reading early
        let actiondump = Self::get_actiondump(actiondump_path);

        // Hash files
        let mut hasher = FxHasher::default();
        for file in &files {
            file.hash.hash(&mut hasher);
        }
        let hash = hasher.finish();

        let mut root: Option<Spur> = None;
        for file in &files {
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
        }
        let root = if let Some(it) = root {
            it
        } else {
            return Err(ProjectCreationError::NoFilesInputed);
        };

        let actiondump = actiondump.await.map_err(ProjectCreationError::ActionDump)?;
        let resources = ProjectResources::new(root, Arc::new(actiondump));

        Ok(Project::<RawProjectFiles> {
            resources: Arc::new(resources),
            files: RawProjectFiles { rodeo, files },
            hash,
        })
    }

    async fn get_actiondump(path: Box<Path>) -> Result<ActionDump, ActionDumpReadError> {
        // intentionally don't use serde_json::from_reader
        let dump = match read_to_string(&path).await {
            Ok(t) => t,
            Err(e) => return Err(ActionDumpReadError::Io(path, e)),
        };
        // I *could* make this a match statement, but I wont
        let dump = serde_json::from_str(&dump).map_err(|e| ActionDumpReadError::Parse(path, e))?;
        Ok(dump)
    }
}

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

impl ProjectFile<RawFile> {
    // This is technically safe, but marking as unsafe because it breaks the contract
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

    // TODO: move?
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

// None
#[derive(Debug, PartialEq)]
pub struct RawFile {
    pub root: Spur,
}
impl FileResolution for RawFile {}

#[derive(Debug)]
pub struct RawProjectFiles {
    pub files: Vec<ProjectFile<RawFile>>,
    pub rodeo: ThreadedRodeo,
}
impl ProjectFiles for RawProjectFiles {}
