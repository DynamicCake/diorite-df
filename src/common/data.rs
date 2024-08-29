use crate::common::prelude::*;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Wrapped<T> {
    pub open: Spanned<()>,
    pub tags: MaybeSpan<Parameters<T>>,
    pub close: Spanned<()>,
}

impl<T> Wrapped<T> {
    pub fn new(open: Spanned<()>, tags: MaybeSpan<Parameters<T>>, close: Spanned<()>) -> Self {
        Self { open, tags, close }
    }
}

impl<T> CalcSpan for Wrapped<T> {
    fn calculate_span(&self) -> Span {
        self.open.span.start..self.close.span.end
    }
}

// StringLiteral

#[derive(Debug, PartialEq)]
pub struct StringLiteral {
    inner: Spur,
}

impl StringLiteral {
    pub fn new(inner: Spur) -> Self {
        Self { inner }
    }
}

// NumberLiteral

#[derive(Debug, PartialEq)]
pub struct NumberLiteral {
    inner: Spur,
}

impl NumberLiteral {
    pub fn new(inner: Spur) -> Self {
        Self { inner }
    }
}

// Iden

#[derive(Debug, PartialEq, Clone)]
pub struct Iden {
    pub name: Spur,
}

impl Iden {
    pub fn new(name: Spur) -> Self {
        Self { name }
    }
}
