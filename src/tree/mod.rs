//! Provides structs to create a parse tree to be used [crate::parser]

pub mod prelude {
    pub use crate::common::prelude::*;
    pub use crate::tree::{recovery::*, statement::*, top::*};
}

use std::marker::PhantomData;

use lasso::Spur;

use crate::{common::span::{
    CalcSpan, MaybeSpan, Span, SpanEnd, SpanSize, SpanStart, Spanned, TryCalcSpan, TrySpanEnd,
    TrySpanStart,
}, semantic::Analyzer};

use self::top::TopLevel;

pub mod recovery;
pub mod statement;
pub mod top;

#[derive(Debug, PartialEq)]
pub struct TreeRoot {
    pub top_statements: Vec<TopLevel>,
}

impl TreeRoot {
    pub fn new(top_statements: Vec<TopLevel>) -> Self {
        Self { top_statements }
    }
}


