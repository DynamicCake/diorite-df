use std::{path::Path, sync::Arc};

use lasso::RodeoResolver;
use path_clean::clean;

use crate::common::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ProjectFile(Spur);
impl ProjectFile {
    /// Creates a new project file
    /// See [ProjectFileResolveError] for ways this function can fail
    pub fn new(
        path: Spur,
        resolver: Arc<ProjectResolver>,
    ) -> Result<Self, ProjectFileResolveError> {
        let name = resolver
            .rodeo
            .try_resolve(&path)
            .ok_or_else(|| ProjectFileResolveError::CannotResolve)?;
        let resolved_path = Path::new(name);

        if !resolved_path.is_relative() {
            return Err(ProjectFileResolveError::NotRelative);
        }

        let full = clean(resolver.path.join(resolved_path));
        if full.strip_prefix(resolved_path).is_err() {
            return Err(ProjectFileResolveError::NotInProject);
        }
        Ok(Self(path))
    }

    pub fn path(self) -> Spur {
        self.0
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ProjectFileResolveError {
    /// The path: Spur cannot be found in the rodeo
    #[error("Cannot resolve spur")]
    CannotResolve,
    /// The path is absolute
    #[error("Path is not relative")]
    NotRelative,
    /// The file specificed in the project could not be found
    #[error("Path is not in project")]
    NotInProject,
}


pub struct ProjectResolver {
    /// This field implements Sized
    pub rodeo: RodeoResolver,
    pub path: Box<Path>,
}

impl ProjectResolver {
    pub fn new(rodeo: RodeoResolver, path: Box<Path>) -> Self {
        Self { rodeo, path }
    }
}

