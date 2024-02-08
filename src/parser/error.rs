use core::fmt;
use std::{fmt::Display, marker::PhantomData, sync::Arc};

use crate::{
    ast::{recovery::Recovery, Spanned},
    lexer::Token,
};

#[derive(Debug)]
pub struct CompilerResult<'src, T, E = Vec<UnexpectedToken<'src>>> {
    pub data: T,
    pub error: E,
    /// Show if EOF has been reached
    /// Is `Some` when an unexpected EOF has been reached
    /// and `None` when everything is going good
    /// Premature optimization go brrrr
    pub at_eof: Option<Box<UnexpectedEOF<'src>>>,
}

impl<'src, T, E> CompilerResult<'src, T, E> {
    pub fn new(data: T, error: E, at_eof: Option<Box<UnexpectedEOF<'src>>>) -> Self {
        Self {
            data,
            error,
            at_eof,
        }
    }

    pub fn map_inner<R, F>(self, f: F) -> CompilerResult<'src, R, E>
    where
        F: FnOnce(T) -> R,
    {
        let Self {
            data,
            error,
            at_eof,
        } = self;
        let res = f(data);
        CompilerResult::<R, E>::new(res, error, at_eof)
    }
}

#[derive(Debug)]
pub struct UnexpectedToken<'src> {
    pub expected: ExpectedTokens<'src>,
    pub received: Spanned<Token<'src>>,
    pub expected_name: Option<String>,
}

impl<'src> UnexpectedToken<'src> {
    pub fn new(
        expected: ExpectedTokens<'src>,
        received: Spanned<Token<'src>>,
        expected_name: Option<String>,
    ) -> Self {
        Self {
            expected,
            received,
            expected_name,
        }
    }
}

#[derive(Debug)]
pub struct UnexpectedEOF<'src> {
    pub expected: Option<ExpectedTokens<'src>>,
    pub expected_name: Option<String>,
}

impl<'src> UnexpectedEOF<'src> {
    pub fn new(expected: Option<ExpectedTokens<'src>>, expected_name: Option<String>) -> Self {
        Self {
            expected,
            expected_name,
        }
    }
}

#[derive(Debug)]
pub struct LexerError {
    pub token: Spanned<()>,
}

impl LexerError {
    pub fn new(token: Spanned<()>) -> Self {
        Self { token }
    }
}

#[derive(Debug, Clone)]
pub struct ExpectedTokens<'src> {
    pub expected: Arc<[Token<'src>]>,
}

impl<'src> ExpectedTokens<'src> {
    pub fn new(expected: Arc<[Token<'src>]>) -> Self {
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
        let later: String = iter
            .map(|tok| ", ".to_string() + &format!("{:#?}", tok))
            .collect();
        write!(f, "[{}{}]", first, later).unwrap();

        Ok(())
    }
}
