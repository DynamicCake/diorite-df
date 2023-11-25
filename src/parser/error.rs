use core::fmt;
use std::{fmt::Display, marker::PhantomData};

use crate::{
    ast::{recovery::Recovery, Spanned},
    lexer::Token,
};

use super::TokAdvanceError;

#[derive(Debug)]
pub struct CompilerResult<T, E 
   // = CompilerResultDefault<'src>
    > {
    pub data: T,
    pub error: Vec<E>,
}

impl<T, E> CompilerResult<T, E> {
    pub fn new(data: T, error: Vec<E>) -> Self {
        CompilerResult { data, error }
    }

    pub fn single_err(data: T, error: E) -> Self {
        CompilerResult::new(data, vec![error])
    }

    pub fn map_inner<R, F>(self, f: F) -> CompilerResult<R, E>
    where
        F: FnOnce(T) -> R,
    {
        let Self { data, error } = self;
        let res = f(data);
        CompilerResult::<R, E>::new(res, error)
    }
}

#[derive(Debug)]
pub enum CompilerError<'src> {
    Unexpected(UnexpectedToken<'src>),
    UnexpectedEOF(UnexpectedEOF<'src>),
    LexerError(LexerError),
}

impl<'src> From<TokAdvanceError<'src>> for CompilerError<'src> {
    fn from(value: TokAdvanceError<'src>) -> Self {
        match value {
            TokAdvanceError::UnexpectedEOF(inner) => CompilerError::UnexpectedEOF(inner),
            TokAdvanceError::Lexer(inner) => CompilerError::LexerError(inner),
        }
    }
}

#[derive(Debug)]
pub struct UnexpectedToken<'src> {
    pub expected: ExpectedTokens<'src>,
    pub received: Spanned<Token<'src>>,
    pub expected_name: Option<String>,
}

impl<'src> UnexpectedToken<'src> {
    pub fn new(expected: ExpectedTokens<'src>, received: Spanned<Token<'src>>, expected_name: Option<String>) -> Self { Self { expected, received, expected_name } }
}

#[derive(Debug)]
pub struct UnexpectedEOF<'src> {
    pub expected: Option<ExpectedTokens<'src>>,
    pub expected_name: Option<String>,
}

impl<'src> UnexpectedEOF<'src> {
    pub fn new(expected: Option<ExpectedTokens<'src>>, expected_name: Option<String>) -> Self { Self { expected, expected_name } }
}

#[derive(Debug)]
pub struct LexerError {
    pub token: Spanned<()>
}

impl LexerError {
    pub fn new(token: Spanned<()>) -> Self { Self { token } }
}

#[derive(Debug, Clone)]
pub struct ExpectedTokens<'src> {
    pub expected: Vec<Token<'src>>,
}

impl<'a> From<Token<'a>> for ExpectedTokens<'a> {
    fn from(value: Token<'a>) -> Self {
        Self::new(vec![value])
    }
}

impl<'src> ExpectedTokens<'src> {
    pub fn new(expected: Vec<Token<'src>>) -> Self {
        Self { expected }
    }
}

impl Display for ExpectedTokens<'_> {
    // TODO Make this less scuffed, I am too lazy
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.expected.iter();
        let first = if let Some(it) = iter.next() {
            format!("{:?}", it)
        } else {
            write!(f, "[]").unwrap();
            return Ok(());
        };
        let later: String = iter.map(|tok| format!(", {:?}", tok)).collect();
        write!(f, "[{}{}]", first, later).unwrap();

        Ok(())
    }
}

