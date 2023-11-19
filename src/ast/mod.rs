use logos::Span;

use self::top::TopLevel;

pub mod statement;
pub mod top;

#[derive(Debug)]
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
pub struct Parameters<T> {
    items: Vec<Parameter<T>>,
}

#[derive(Debug)]
pub struct Parameter<T> {
    comma: Spanned<()>,
    data: T,
}

#[derive(Debug)]
pub struct StringLiteral<'src> {
    inner: &'src str,
}

#[derive(Debug)]
pub struct NumberLiteral<'src> {
    inner: &'src str,
}

#[derive(Debug)]
pub struct Iden<'src> {
    name: &'src str,
}

impl<'src> Iden<'src> {
    pub fn new(name: &'src str) -> Self { Self { name } }
}
