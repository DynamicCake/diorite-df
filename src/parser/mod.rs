use std::iter::{Enumerate, Peekable};
use std::rc::Rc;

use logos::{Lexer, SpannedIter};

use crate::ast::{Program, Spanned};
use crate::{ast::top::TopLevel, lexer::Token};

use self::error::*;
use self::top::*;

pub mod error;
pub mod top;

pub struct Parser<'src> {
    toks: Peekable<SpannedIter<'src, Token<'src>>>,
}

impl<'src> Parser<'src> {
    fn new(lexer: Lexer<'src, Token<'src>>) -> Self {
        Self {
            toks: lexer.spanned().peekable(),
        }
    }

    fn parse(&mut self) -> CompilerResult<'src, Program<'src>> {
        let mut stmts = Vec::new();
        let mut errors = Vec::new();
        while let Some(_it) = self.toks.peek() {
            let mut top = self.top_level();
            errors.append(&mut top.error);
            stmts.push(top.data);
        }
        todo!()
        // CompilerResult::new(Program::new(stmts), errors)
    }

    fn consume(
        &mut self,
        expect_tok: Token<'src>,
        msg: &str,
    ) -> Result<&Token<'src>, CompilerError<'src>> {
        let (token, span) = match self.toks.peek() {
            Some(it) => it,
            None => todo!(), // return Err(CompilerError::UnexpectedEOF(expect_tok)),
        };

        let token = match token {
            Ok(it) => it,
            Err(_) => {
                return Err(CompilerError::LexerError(Spanned::<()>::empty(
                    span.clone(),
                )));
            }
        };

        if token == &expect_tok {
            Ok(token)
        } else {
            todo!()
            /*
            Err(CompilerError::Unexpected {
                expected: expect_tok,
                received: token.to_owned().spanned(span.clone()),
            }) */
        }
    }

    pub fn next<T>(&mut self, expected: ExpectedTokens<'src>, data: T) -> Result<Token<'src>, CompilerError<'src>>{

        let (token, span) = if let Some(it) = self.toks.next() {
            it
        } else {
            return Err(CompilerResult::new(data, vec![CompilerError::UnexpectedEOF(expected)]));
        };

        let token = if let Ok(it) = token {
            it
        } else {
            return Err(CompilerResult::new(data, vec![CompilerError::LexerError(Spanned::<()>::empty(span))]));
        };
        Ok(token)
    }
}
