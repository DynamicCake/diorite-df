use core::fmt;
use std::{fmt::Display, rc::Rc};

use crate::{ast::Spanned, lexer::Token};

pub struct CompilerResult<'src, T> {
    pub data: T,
    pub error: Vec<CompilerError<'src>>,
}

impl<'src, T> CompilerResult<'src, T> {
    pub fn new(data: T, error: Vec<CompilerError<'src>>) -> Self {
        CompilerResult { data, error }
    }
}

pub enum CompilerError<'src> {
    Unexpected {
        expected: ExpectedTokens<'src>,
        received: Spanned<Token<'src>>,
    },
    UnexpectedEOF(ExpectedTokens<'src>),
    LexerError(Spanned<()>),
}

impl CompilerError<'_> {
    fn display() {}
}

pub struct ExpectedTokens<'src> {
    expected: Vec<Token<'src>>,
}

impl<'src> ExpectedTokens<'src> {
    pub fn new(expected: Vec<Token<'src>>) -> Self {
        Self { expected }
    }
}

impl Display for ExpectedTokens<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.expected.iter();
        let first = if let Some(it) = iter.next() {
            it.expected_print()
        } else {
            return Err(fmt::Error);
        };
        let later: String = iter
            .map(|tok| ", ".to_string() + &tok.expected_print())
            .collect();
        write!(f, "[{}{}]", first, later);

        Ok(())
    }
}
