use logos::Span;

use self::top::TopLevel;

pub mod statement;
pub mod top;

pub struct Spanned<T> {
    data: T,
    span: Span,
}

impl<T> Spanned<T> {
    pub fn new(data: T, span: Span) -> Self {
        Self { data, span }
    }
    pub fn empty(span: Span) -> Spanned<()> {
        Spanned { data: (), span }
    }
}

pub struct Program<'src> {
    top_statements: Vec<TopLevel<'src>>,
}

impl<'src> Program<'src> {
    pub fn new(top_statements: Vec<TopLevel<'src>>) -> Self {
        Self { top_statements }
    }
}

pub struct Parameters<T> {
    items: Vec<Parameter<T>>,
}

pub struct Parameter<T> {
    comma: Spanned<()>,
    data: T,
}

pub struct StringLiteral<'src> {
    inner: &'src str,
}

pub struct NumberLiteral<'src> {
    inner: &'src str,
}

pub struct Iden<'src> {
    name: &'src str,
}
