use std::{
    hash::{DefaultHasher, SipHasher},
    marker::PhantomData,
};

use crate::{
    common::prelude::*,
    dump::ActionDump,
    semantic::{AnalysisResult, Analyzer},
    tree::top::TopLevel,
};

use std::{
    hash::{BuildHasher, Hash, Hasher, RandomState},
    mem::transmute,
    path::Path,
    sync::Arc,
};

use lasso::{RodeoResolver, Spur};
use path_clean::clean;

use crate::common::prelude::*;

/*
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ProjectFile(Spur);
impl ProjectFile {
    /// Creates a new project file
    /// See [ProjectFileResolveError] for ways this function can fail
    pub fn new(
        path: Spur,
        resolver: Arc<ProjectResources>,
    ) -> Result<Self, ProjectFileResolveError> {
        let name = resolver
            .rodeo
            .try_resolve(&path)
            .ok_or_else(|| ProjectFileResolveError::CannotResolve)?;
        let resolved_path = Path::new(name);

        if resolved_path.is_relative() {
            return Err(ProjectFileResolveError::Relative);
        }

        let full = clean(resolver.project_root.join(resolved_path));
        if !full.starts_with(resolved_path) {
            return Err(ProjectFileResolveError::NotInProject);
        }
        Ok(Self(path))
    }

    pub fn path(self) -> Spur {
        self.0
    }
}
*/

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
pub struct ProjectResources {
    rodeo: RodeoResolver,
    project_root: Box<Path>,
}

pub struct Project {
    resources: ProjectResources,
    projects: Vec<Program>,
    hash: u64,
}

impl Project<UncheckedProgram> {
    pub fn new(resources: ProjectResources, programs: Vec<Program<UncheckedProgram>>) -> Self {
        let hash = {
            // Not so random now huh
            // This will be changed when I find a suitable hashing algorithm
            let s: RandomState = unsafe {
                let nuh_uh: (u64, u64) = (420, 69);
                transmute(nuh_uh)
            };
            let mut hasher = s.build_hasher();
            for program in &programs {
                program.hash.hash(&mut hasher);
            }
            hasher.finish()
        };
        Self {
            resources,
            projects: programs,
            hash,
        }
    }
}

impl ProjectResources {
    pub fn new(rodeo: RodeoResolver, path: Box<Path>) -> Self {
        Self {
            rodeo,
            project_root: path,
        }
    }

    pub fn rodeo(&self) -> &RodeoResolver {
        &self.rodeo
    }

    pub fn project_root(&self) -> &Path {
        &self.project_root
    }
}

pub struct ProjectFile {
    pub src: Spur,
    /// Relative path of the file from the project root
    pub file: Spur,
}

impl ProjectFile {
    pub fn new(src: Spur, file: Spur) -> Self {
        Self { src, file }
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub top_statements: Vec<TopLevel>,
}

/// Used for the type state pattern with [Program]
pub trait ProgramState {}

/// A program state with source code only and file info
#[derive(Debug, PartialEq)]
pub struct SourceProgram;
impl ProgramState for SourceProgram {}

/// A program state with a parse tree
#[derive(Debug, PartialEq)]
pub struct UncheckedProgram {}
impl ProgramState for UncheckedProgram {}

/// This means the program has been checked for a single thread
/// There still hasn't been starter conflict verification
#[derive(Debug, PartialEq)]
pub struct CheckedAloneProgram;
impl ProgramState for CheckedAloneProgram {}

#[derive(Debug, PartialEq)]
pub struct CheckedProgram;
impl ProgramState for CheckedProgram {}

// Files

pub trait FileState {}

#[derive(Debug, PartialEq)]
pub struct RawFile;
impl FileState for RawFile {}

#[derive(Debug, PartialEq)]
pub struct ParsedFile {
    program: Program,
}
impl FileState for ParsedFile {}

#[derive(Debug, PartialEq)]
pub struct AnalyzedFile {
    program: Program,
}

impl FileState for AnalyzedFile {}
