use std::{fmt::Display, path::Path, sync::Arc};

use lasso::Spur;

use crate::{common::span::{Referenced, Spanned}, lexer::Token};

/// Represents a parse result that every parsing function should return
/// All parsing functions are called by [Parser::parse](crate::parser::Parser::parse)
#[derive(Debug, PartialEq)]
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

    /// Maps a [ParseResult<T>] to [ParseResult<R>] by applying a function to the data field
    pub fn map_data<R, F>(self, f: F) -> ParseResult<R, E>
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
        Self {
            data,
            error: Vec::new(),
            at_eof: None,
        }
    }
}

/// Created when an unexpected token is encountered
#[derive(Debug, PartialEq)]
pub struct UnexpectedToken {
    pub expected: ExpectedTokens,
    pub received: Spanned<Token>,
    pub expected_name: Option<Arc<str>>,
    pub file: Spur,
}

impl UnexpectedToken {
    pub fn new(
        expected: ExpectedTokens,
        received: Spanned<Token>,
        expected_name: Option<Arc<str>>,
        file: Spur,
    ) -> Self {
        Self {
            expected,
            received,
            expected_name,
            file,
        }
    }

    /// Returns a more friendly error message
    pub fn expected_print(&self) -> String {
        if let Some(it) = &self.expected_name {
            format!("{} ({})", it, self.expected.to_string())
        } else {
            self.expected.to_string()
        }
    }
}

/// Created when an unexpected end of file
#[derive(Debug, PartialEq)]
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

/// Created when an invalid token is encountered
#[derive(Debug, PartialEq)]
pub struct LexerError {
    pub token: Referenced<()>,
}

impl LexerError {
    pub fn new(token: Referenced<()>) -> Self {
        Self { token }
    }
}

/// An immutable list of tokens
#[derive(Debug, PartialEq, Clone)]
pub struct ExpectedTokens {
    pub expected: Arc<[Token]>,
}

impl ExpectedTokens {
    pub fn new(expected: Arc<[Token]>) -> Self {
        Self { expected }
    }
}

impl Display for ExpectedTokens {
    // HACK: Make this less scuffed, I am too lazy
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.expected.iter();
        let first = if let Some(it) = iter.next() {
            format!("{}", it.expected_print())
        } else {
            write!(f, "[]").unwrap();
            return Ok(());
        };
        let later: String = iter
            .map(|tok| ", ".to_string() + &tok.expected_print())
            .collect();
        write!(f, "[{}{}]", first, later).unwrap();

        Ok(())
    }
}
