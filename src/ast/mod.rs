//! Provides structs to create a parse tree to be used [crate::parser]

pub mod prelude {
    pub use crate::common::prelude::*;
    pub use crate::ast::{statement::*, top::*};
}

use std::marker::PhantomData;

use lasso::Spur;

use crate::{common::span::{
    CalcSpan, MaybeSpan, Span, SpanEnd, SpanSize, SpanStart, Spanned, TryCalcSpan, TrySpanEnd,
    TrySpanStart,
}, semantic::Analyzer};

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


