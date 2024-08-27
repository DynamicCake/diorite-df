//! Moule for the code generation step
use std::sync::Arc;

use crate::{common::prelude::*, dump::ActionDump, project::{AnalyzedFile, ProjectFile}};

mod block;
mod data;
mod test;

pub struct CodeGenerator<'a> {
    pub dump: &'a ActionDump,
    pub program: &'a ProjectFile<AnalyzedFile>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(dump: &'a ActionDump, program: &'a ProjectFile<AnalyzedFile>) -> Self {
        Self { dump, program }
    }

    pub fn generate(&self) -> String {
        "".to_string()
    }
}
