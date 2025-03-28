//! Moule for the code generation step

use crate::{dump::ActionDump, project::{AnalyzedFile, ProjectFile}};

pub mod block;
pub mod data;
pub mod test;

pub struct CodeGenerator<'a> {
    pub dump: &'a ActionDump,
    pub program: &'a ProjectFile<AnalyzedFile<'a>>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(dump: &'a ActionDump, program: &'a ProjectFile<AnalyzedFile<'a>>) -> Self {
        Self { dump, program }
    }

    pub fn generate(&self) -> String {
        "".to_string()
    }
}
