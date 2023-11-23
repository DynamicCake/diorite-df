use std::iter::{Enumerate, Peekable};
use std::rc::Rc;

use logos::{Lexer, SpannedIter};

use crate::ast::{Program, Spanned};
use crate::{ast::top::TopLevel, lexer::Token};

use self::error::*;
use self::top::*;

pub mod error;
pub mod stmt;
pub mod top;

pub struct Parser<'src> {
    toks: Peekable<SpannedIter<'src, Token<'src>>>,
}

impl<'src> Parser<'src> {
    pub fn new(lexer: Lexer<'src, Token<'src>>) -> Self {
        Self {
            toks: lexer.spanned().peekable(),
        }
    }

    pub fn parse(&mut self) -> CompilerResult<'src, Program<'src>> {
        let mut stmts = Vec::new();
        let mut errors = Vec::new();
        loop {
            if let Err(err) = self.peek() {
                match err {
                    CompilerError::UnexpectedEOF {
                        expected: _,
                        expected_name: _,
                    } => break,
                    CompilerError::LexerError(_) => break,
                    it => panic!("self.peek cannot return error variant {:?}", it),
                };
            };
            let mut top = self.top_level();
            errors.append(&mut top.error);
            if let Some(data) = top.data {
                stmts.push(data)
            };
        }
        CompilerResult::new(Some(Program::new(stmts)), errors)
    }

    pub fn next_expect(
        &mut self,
        expected: &ExpectedTokens<'src>,
        expected_name: Option<&str>,
    ) -> Result<Spanned<Token<'src>>, CompilerError<'src>> {
        if let Some(it) = self.toks.next() {
            let (token, span) = it;
            if let Ok(token) = token {
                let _match_expected = &expected.expected;
                return if matches!(&token, _match_expected) {
                    Ok(token.spanned(span))
                } else {
                    Err(CompilerError::Unexpected {
                        expected: expected.clone(),
                        received: token.spanned(span),
                        expected_name: expected_name.map(|str| str.to_owned()),
                    })
                };
            } else {
                Err(CompilerError::LexerError(Spanned::<()>::empty(span)))
            }
        } else {
            Err(CompilerError::UnexpectedEOF {
                expected: Some(expected.clone()),
                expected_name: None,
            })
        }
    }

    pub fn peek_expect(
        &mut self,
        expected: &ExpectedTokens<'src>,
        msg: Option<&str>,
    ) -> Result<Spanned<&Token<'src>>, CompilerError<'src>> {
        let token = if let Some(it) = self.toks.peek() {
            let (token, span) = it;
            if let Ok(token) = token {
                let _match_expected = &expected.expected;
                return if matches!(&token, _match_expected) {
                    Ok(Spanned::new(token, span.clone()))
                } else {
                    Err(CompilerError::Unexpected {
                        expected: expected.clone(),
                        received: token.clone().spanned(span.clone()),
                        expected_name: msg.map(|str| str.to_owned()),
                    })
                };
            } else {
                Err(CompilerError::LexerError(Spanned::<()>::empty(
                    span.clone(),
                )))
            }
        } else {
            Err(CompilerError::UnexpectedEOF {
                expected: None,
                expected_name: None,
            })
        };
        token
    }

    pub fn peek(&mut self) -> Result<Spanned<&Token<'src>>, CompilerError<'src>> {
        if let Some(it) = self.toks.peek() {
            let (token, span) = it;
            if let Ok(token) = token {
                let spanned = Spanned::new(token, span.clone());
                Ok(spanned)
            } else {
                Err(CompilerError::LexerError(Spanned::<()>::empty(
                    span.clone(),
                )))
            }
        } else {
            Err(CompilerError::UnexpectedEOF {
                expected: None,
                expected_name: None,
            })
        }
    }

    pub fn next(&mut self) -> Result<Spanned<Token<'src>>, CompilerError<'src>> {
        if let Some(it) = self.toks.next() {
            let (token, span) = it;
            if let Ok(token) = token {
                let spanned = token.spanned(span);
                Ok(spanned)
            } else {
                Err(CompilerError::LexerError(Spanned::<()>::empty(span)))
            }
        } else {
            Err(CompilerError::UnexpectedEOF {
                expected: None,
                expected_name: None,
            })
        }
    }
}
