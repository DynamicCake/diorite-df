use crate::{common::prelude::*, dump::ActionDump};

use std::sync::Arc;

use lasso::Spur;

pub mod analyzed;
pub mod parsed;
pub mod raw;

/// Immutable resources used for the project not including the source
#[derive(Debug)]
pub struct ProjectResources {
    project_root: Spur,
    actiondump: Arc<ActionDump>,
}

/// A trait to indicate if a struct has state-specific project data
/// ProjectFiles typically contain a Vec with FileResoltion implemented
pub trait ProjectFiles {}

#[derive(Debug)]
pub struct Project<T: ProjectFiles> {
    pub resources: Arc<ProjectResources>,
    /// The files that the project has
    pub files: T,
    /// A hash comprised of the project metadata and all the source files
    pub hash: u64,
}

impl ProjectResources {
    pub fn new(project_root: Spur, actiondump: Arc<ActionDump>) -> Self {
        Self {
            project_root,
            actiondump,
        }
    }

    pub fn project_root(&self) -> Spur {
        self.project_root
    }

    pub fn actiondump(&self) -> &ActionDump {
        &self.actiondump
    }
}

/// A trait to indicate if a struct is resolution data for a file
pub trait FileResolution {}

#[derive(Debug, PartialEq)]
pub struct ProjectFile<S: FileResolution> {
    pub src: Spur,
    /// Relative path of the file from the project root
    pub path: Spur,
    pub hash: u64,
    pub resolution: S,
}
