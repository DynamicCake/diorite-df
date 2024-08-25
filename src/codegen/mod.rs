//! Moule for the code generation step
use std::sync::Arc;

use crate::{common::prelude::{CheckedProgram, ProgramFile}, dump::ActionDump};

mod block;
mod data;
mod test;

pub struct CodeGenerator<'a> {
    pub dump: &'a ActionDump,
    pub program: &'a ProgramFile<CheckedProgram>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(dump: &'a ActionDump, program: &'a ProgramFile<CheckedProgram>) -> Self {
        Self { dump, program }
    }

    pub fn generate(&self) -> String {
        "".to_string()
    }
}
