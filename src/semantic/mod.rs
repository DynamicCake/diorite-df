//! Crates a new parse tree using the [crate::ast] module

use std::sync::Arc;

use crate::common::prelude::*;
use crate::parser::ParsedFile;
use crate::project::{ParsedProjectFiles, ProjectFile, TreeFile};
use crate::{dump::ActionDump, error::semantic::SemanticError};

pub struct Analyzer<'a> {
    dump: &'a ActionDump,
    program: ProjectFile<TreeFile>,
    starters: StarterSet,
    errors: Vec<SemanticError>,
}

pub struct AnalysisResult {
    errors: Vec<SemanticError>,
    program: ProjectFile<TreeFile>,
    starters: StarterSet,
}

impl<'a> Analyzer<'a> {
    pub fn verify(program: ProjectFile<TreeFile>, dump: &'a ActionDump) -> AnalysisResult {
        Self::new(program, dump).resolve_self()
    }
    fn new(program: ProjectFile<TreeFile>, dump: &'a ActionDump) -> Self {
        Self {
            dump,
            program,
            starters: StarterSet::new(),
            errors: Vec::new(),
        }
    }

    fn resolve_self(self) -> AnalysisResult {
        AnalysisResult {
            errors: self.errors,
            starters: self.starters,
            program: todo!(),
        }
    }
}
