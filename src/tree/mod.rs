//! Provides structs to create a parse tree
//! Not to be confused with

use lasso::Spur;

use crate::span::{Span, SpanEnd, SpanSize, SpanStart, TryCalcSpan, TrySpanEnd, TrySpanStart};

use self::top::TopLevel;

pub mod recovery;
pub mod statement;
pub mod top;


#[derive(Debug)]
pub struct Program {
    pub top_statements: Vec<TopLevel>,
}

impl Program {
    pub fn new(top_statements: Vec<TopLevel>) -> Self {
        Self { top_statements }
    }
}

#[derive(Debug)]
pub struct Parameters<T> {
    pub items: Vec<T>,
}

impl<T> TryCalcSpan for Parameters<T>
where
    T: SpanStart,
    T: SpanEnd,
{
    fn try_calculate_span(&self) -> Option<Span> {
        match self.items.first() {
            Some(first) => {
                let first = first.start();
                let last = self
                    .items
                    .last()
                    .expect("If first exists, last exists")
                    .end();
                Some(first..last)
            }
            None => None,
        }
    }
}

impl<T> TrySpanStart for Parameters<T>
where
    T: TrySpanStart,
{
    fn try_start(&self) -> Option<SpanSize> {
        for span in self.items.iter() {
            let span = span.try_start();
            if let Some(span) = span {
                return Some(span);
            }
        }
        None
    }
}

impl<T> TrySpanEnd for Parameters<T>
where
    T: TrySpanEnd,
{
    fn try_end(&self) -> Option<SpanSize> {
        for span in self.items.iter().rev() {
            let span = span.try_end();
            if let Some(span) = span {
                return Some(span);
            }
        }
        None
    }
}

impl<T> TryCalcSpan for Parameters<Parameters<T>>
where
    T: TrySpanStart,
    T: TrySpanEnd,
{
    fn try_calculate_span(&self) -> Option<Span> {
        match self.items.first() {
            Some(first) => {
                let first = first.try_start()?;
                let last = self
                    .items
                    .last()
                    .expect("If first exists, last exists")
                    .try_end()?;
                Some(first..last)
            }
            None => None,
        }
    }
}

impl<T> Parameters<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }
}

// StringLiteral

#[derive(Debug)]
pub struct StringLiteral {
    inner: Spur,
}

impl StringLiteral {
    pub fn new(inner: Spur) -> Self {
        Self { inner }
    }
}

// NumberLiteral

#[derive(Debug)]
pub struct NumberLiteral {
    inner: Spur,
}

impl NumberLiteral {
    pub fn new(inner: Spur) -> Self {
        Self { inner }
    }
}

// Iden

#[derive(Debug)]
pub struct Iden {
    pub name: Spur,
}

impl Iden {
    pub fn new(name: Spur) -> Self {
        Self { name }
    }
}
