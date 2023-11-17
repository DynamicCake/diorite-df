use chumsky::span::SimpleSpan;

pub mod statement;
pub mod top;

pub struct Spanned<T> {
    span: SimpleSpan,
    data: T,
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
