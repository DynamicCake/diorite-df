use core::fmt;
use std::fmt::Display;

use crate::{ast::Spanned, lexer::Token};

#[derive(Debug)]
pub struct CompilerResult<'src, T> {
    pub data: Option<T>,
    pub error: Vec<CompilerError<'src>>,
}

impl<'src, T> CompilerResult<'src, T> {
    pub fn new(data: Option<T>, error: Vec<CompilerError<'src>>) -> Self {
        CompilerResult { data, error }
    }
    pub fn single_err(data: Option<T>, error: CompilerError<'src>) -> Self {
        CompilerResult::new(data, vec![error])
    }
}

#[derive(Debug)]
pub enum CompilerError<'src> {
    Unexpected {
        expected: ExpectedTokens<'src>,
        received: Spanned<Token<'src>>,
        expected_name: Option<String>,
    },
    UnexpectedEOF {
        expected: Option<ExpectedTokens<'src>>,
        expected_name: Option<String>,
    },
    LexerError(Spanned<()>),
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
