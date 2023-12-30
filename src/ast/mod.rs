use std::marker::PhantomData;

use crate::lexer::Token;

use self::top::TopLevel;

pub mod recovery;
pub mod statement;
pub mod top;

pub type Span = core::ops::Range<usize>;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub data: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(data: T, span: Span) -> Self {
        Self { data, span }
    }
    pub fn empty(span: Span) -> Spanned<()> {
        Spanned { data: (), span }
    }
    pub fn to_empty(self) -> Spanned<()> {
        Self::empty(self.span)
    }
    pub fn map_inner<R, F>(self, f: F) -> Spanned<R>
    where
        F: FnOnce(T) -> R,
    {
        let Self { data, span } = self;
        let res = f(data);
        Spanned::<R>::new(res, span)
    }
}

#[derive(Debug)]
pub struct Program<'src> {
    top_statements: Vec<TopLevel<'src>>,
}

impl<'src> Program<'src> {
    pub fn new(top_statements: Vec<TopLevel<'src>>) -> Self {
        Self { top_statements }
    }
}

#[derive(Debug)]
pub struct Parameters<'src, T> {
    pub items: Vec<Parameter<'src, T>>,
}

#[derive(Debug)]
pub struct Parameter<'src, T> {
    pub data: T,
    pub comma: Spanned<()>,
    phantom: PhantomData<&'src T>,
}

impl<'src, T> Parameter<'src, T> {
    fn new(data: T, comma: Spanned<()>) -> Self {
        Self {
            data,
            comma,
            phantom: PhantomData,
        }
    }
}

// StringLiteral

#[derive(Debug)]
pub struct StringLiteral<'src> {
    inner: &'src str,
}

impl<'src> StringLiteral<'src> {
    pub fn new(inner: &'src str) -> Self {
        Self { inner }
    }
}

// NumberLiteral

#[derive(Debug)]
pub struct NumberLiteral<'src> {
    inner: &'src str,
}

// Iden

#[derive(Debug)]
pub struct Iden<'src> {
    pub name: &'src str,
}

impl<'src> Iden<'src> {
    pub fn new(name: &'src str) -> Self {
        Self { name }
    }
}
