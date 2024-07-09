//! Provides structs to create a parse tree to be used [crate::parser]

pub mod prelude {
    pub use crate::tree::Program;
    pub use crate::tree::{recovery::*, statement::*, top::*};
    pub use crate::common::prelude::*;
}

use lasso::Spur;

use crate::common::span::{
    CalcSpan, MaybeSpan, Span, SpanEnd, SpanSize, SpanStart, Spanned, TryCalcSpan, TrySpanEnd,
    TrySpanStart,
};

use self::top::TopLevel;

pub mod recovery;
pub mod statement;
pub mod top;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub top_statements: Vec<TopLevel>,
}

impl Program {
    pub fn new(top_statements: Vec<TopLevel>) -> Self {
        Self { top_statements }
    }
}
