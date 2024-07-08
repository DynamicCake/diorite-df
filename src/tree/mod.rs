//! Provides structs to create a parse tree
//! Not to be confused with

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

