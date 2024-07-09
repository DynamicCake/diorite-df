//! Provides structs to create an ast to be used in [crate::semantic]

use lasso::Spur;

use crate::ast::prelude::*;

pub mod prelude {
    pub use crate::ast::Ast;
    pub use crate::ast::{statement::*, top::*};
    pub use crate::common::prelude::*;
}

use self::top::TopLevel;

pub mod statement;
pub mod top;

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub top_statements: Vec<TopLevel>,
}

impl Ast {
    pub fn new(top_statements: Vec<TopLevel>) -> Self {
        Self { top_statements }
    }
}

