use std::marker::PhantomData;

use logos::Span;

use crate::lexer::Token;

use self::top::TopLevel;

pub mod recovery;
pub mod statement;
pub mod top;

pub trait AstNode<'src, T: NonTerminal<'src> + Terminal<'src>> {}

pub trait NonTerminal<'src> {
    fn collect_tokens(self) -> Vec<Spanned<Token<'src>>>;
}

pub trait Terminal<'src> {
    fn to_token(self) -> Token<'src>;
}

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
pub struct Parameters<'src, T>  {
    pub items: Vec<Parameter<'src, T>>,
}

impl<'src, T> NonTerminal<'src> for Parameters<'src, T> where T: Terminal<'src> {

}

impl<'src, T> NonTerminal<'src> for Parameters<'src, T> where T: NonTerminal<'src> {
    fn collect_tokens(self) -> Vec<Spanned<Token<'src>>> {
        let mut out = Vec::new();
        self.items.into_iter().for_each(|it| {
            out.append(&mut it.collect_tokens());
        });
        out
    }
}

#[derive(Debug)]
pub struct Parameter<'src, T>
where
    T: NonTerminal<'src> + Terminal<'src>,
{
    pub data: T,
    pub comma: Spanned<()>,
    phantom: PhantomData<&'src T>,
}

impl<'src, T> NonTerminal<'src> for Parameter<'src, T>
where
    T: NonTerminal<'src> + Terminal<'src>,
{
    fn collect_tokens(self) -> Vec<Spanned<Token<'src>>> {
        let out = Vec::new();
    }
}

impl<'src, T> Parameter<'src, T>
where
    T: NonTerminal<'src> + Terminal<'src>,
{
    fn new(data: T, comma: Spanned<()>) -> Self {
        Self {
            data,
            comma,
            phantom: PhantomData,
        }
    }
}

struct Spooky<T> {
    phantom: PhantomData<T>
}

struct SpookRef<'src, T> {
    phantom_ref: &'src PhantomData<T>
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

impl<'src> Terminal<'src> for StringLiteral<'src> {
    fn to_token(self) -> Token<'src> {
        Token::Iden(self.inner)
    }
}

// NumberLiteral

#[derive(Debug)]
pub struct NumberLiteral<'src> {
    inner: &'src str,
}

impl<'src> Terminal<'src> for NumberLiteral<'src> {
    fn to_token(self) -> Token<'src> {
        Token::Iden(self.inner)
    }
}

// Iden

#[derive(Debug)]
pub struct Iden<'src> {
    pub name: &'src str,
}

impl<'src> Terminal<'src> for Iden<'src> {
    fn to_token(self) -> Token<'src> {
        Token::Iden(self.name)
    }
}

impl<'src> Iden<'src> {
    pub fn new(name: &'src str) -> Self {
        Self { name }
    }
}

