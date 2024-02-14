use std::{fmt::Display, sync::Arc};

use crate::{ast::Spanned, lexer::{Token}};

#[derive(Debug)]
pub struct ParseResult<T, E = Vec<UnexpectedToken>> {
    pub data: T,
    pub error: E,
    /// Show if EOF has been reached
    /// Is `Some` when an unexpected EOF has been reached
    /// and `None` when everything is going good
    /// Premature optimization go brrrr
    pub at_eof: Option<Box<UnexpectedEOF>>,
}

impl<T, E> ParseResult<T, E> {
    pub fn new(data: T, error: E, at_eof: Option<Box<UnexpectedEOF>>) -> Self {
        Self {
            data,
            error,
            at_eof,
        }
    }

    pub fn map_inner<R, F>(self, f: F) -> ParseResult<R, E>
    where
        F: FnOnce(T) -> R,
    {
        let Self {
            data,
            error,
            at_eof,
        } = self;
        let res = f(data);
        ParseResult::<R, E>::new(res, error, at_eof)
    }
}

impl<T> ParseResult<T> {
    pub fn ok(data: T) -> Self {
        let error = Default::default();
        Self {
            data,
            error,
            at_eof: None,
        }
    }
}

#[derive(Debug)]
pub struct UnexpectedToken {
    pub expected: ExpectedTokens,
    pub received: Spanned<Token>,
    pub expected_name: Option<String>,
}

impl UnexpectedToken {
    pub fn new(
        expected: ExpectedTokens,
        received: Spanned<Token>,
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
pub struct UnexpectedEOF {
    pub expected: Option<ExpectedTokens>,
    pub expected_name: Option<String>,
}

impl UnexpectedEOF {
    pub fn new(expected: Option<ExpectedTokens>, expected_name: Option<String>) -> Self {
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
pub struct ExpectedTokens {
    pub expected: Arc<[Token]>,
}

impl ExpectedTokens {
    pub fn new(expected: Arc<[Token]>) -> Self {
        Self { expected }
    }
}

impl Display for ExpectedTokens {
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
