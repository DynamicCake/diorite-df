//! Provides structs to create a parse tree to be used [crate::parser]

pub mod prelude {
    pub use crate::common::prelude::*;
    pub use crate::ast::{AstRoot, statement::*, top::*};
}




use self::top::AstTopLevel;

pub mod statement;
pub mod top;

#[derive(Debug, PartialEq)]
pub struct AstRoot<'d> {
    pub top_statements: Vec<AstTopLevel<'d>>,
}

impl<'d> AstRoot<'d> {
    pub fn new(top_statements: Vec<AstTopLevel<'d>>) -> Self {
        Self { top_statements }
    }
}


