//! Provides structs to create a parse tree to be used [crate::parser]

pub mod prelude {
    pub use crate::ast::{statement::*, top::*, AstRoot};
    pub use crate::common::prelude::*;
}

use self::top::AstTopLevel;

pub mod statement;
pub mod top;

#[derive(Debug, PartialEq)]
pub struct AstRoot {
    pub top_statements: Vec<AstTopLevel>,
}

impl AstRoot {
    pub fn new(top_statements: Vec<AstTopLevel>) -> Self {
        Self { top_statements }
    }
}
