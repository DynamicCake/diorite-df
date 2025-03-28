//! Provides structs to create a parse tree to be used [crate::parser]

pub mod prelude {
    pub use crate::common::prelude::*;
    pub use crate::tree::{recovery::*, statement::*, top::*, TreeRoot};
}

use self::top::TreeTopLevel;

pub mod recovery;
pub mod statement;
pub mod top;

#[derive(Debug, PartialEq)]
pub struct TreeRoot {
    pub top_statements: Vec<TreeTopLevel>,
}

impl TreeRoot {
    pub fn new(top_statements: Vec<TreeTopLevel>) -> Self {
        Self { top_statements }
    }
}
