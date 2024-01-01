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
    /// The main token iterator
    /// It is not advised to use this in functions called by `parse(&mut self)`
    toks: Peekable<SpannedIter<'src, Token<'src>>>,
    source: &'src str,
}

impl<'src> Parser<'src> {
    pub fn new(lexer: Lexer<'src, Token<'src>>) -> Self {
        Self {
            source: lexer.source(),
            toks: lexer.spanned().peekable(),
        }
    }

    pub fn parse(&mut self) -> CompilerResult<Program<'src>, Vec<CompilerError<'src>>> {
        let mut stmts = Vec::new();
        let mut errors = Vec::new();
        loop {
            if let Err(err) = self.peek() {
                match err {
                    TokAdvanceError::UnexpectedEOF(_err) => break,
                    TokAdvanceError::Lexer(_err) => break,
                };
            };
            let CompilerResult { data, mut error } = self.top_level();
            errors.append(&mut error);
            stmts.push(data);
        }
        CompilerResult::new(Program::new(stmts), errors)
    }

    /// Used when you know what next token you expect
    /// ```diroite
    /// paction Name ()
    ///    HERE ^^^^
    /// ```
    /// If you are 100% sure during compiler time use next_assert
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
                    Err(CompilerError::Unexpected(UnexpectedToken {
                        expected: expected.clone(),
                        received: token.spanned(span),
                        expected_name: expected_name.map(|str| str.to_owned()),
                    }))
                };
            } else {
                Err(CompilerError::LexerError(LexerError::new(
                    Spanned::<()>::empty(span),
                )))
            }
        } else {
            Err(CompilerError::UnexpectedEOF(UnexpectedEOF {
                expected: Some(expected.clone()),
                expected_name: None,
            }))
        }
    }

    pub fn peek_expect(
        &mut self,
        expected: &ExpectedTokens<'src>,
        msg: Option<&str>,
    ) -> Result<Spanned<&Token<'src>>, CompilerError<'src>> {
        if let Some((token, span)) = self.toks.peek() {
            if let Ok(token) = token {
                // This is to bypass the unused warning
                let _match_expected = &expected.expected;
                return if matches!(&token, _match_expected) {
                    Ok(Spanned::new(token, span.clone()))
                } else {
                    Err(CompilerError::Unexpected(UnexpectedToken {
                        expected: expected.clone(),
                        received: token.clone().spanned(span.clone()),
                        expected_name: msg.map(|str| str.to_owned()),
                    }))
                };
            } else {
                Err(CompilerError::LexerError(LexerError::new(
                    Spanned::<()>::empty(span.clone()),
                )))
            }
        } else {
            Err(CompilerError::UnexpectedEOF(UnexpectedEOF {
                expected: None,
                expected_name: None,
            }))
        }
    }

    pub fn peek(&mut self) -> Result<Spanned<&Token<'src>>, TokAdvanceError<'src>> {
        if let Some(it) = self.toks.peek() {
            let (token, span) = it;
            if let Ok(token) = token {
                let spanned = Spanned::new(token, span.clone());
                Ok(spanned)
            } else {
                Err(TokAdvanceError::Lexer(LexerError::new(
                    Spanned::<()>::empty(span.clone()),
                )))
            }
        } else {
            Err(TokAdvanceError::UnexpectedEOF(UnexpectedEOF {
                expected: None,
                expected_name: None,
            }))
        }
    }

    /// Advances to the iterator to the next token
    /// Great for use in recovery functions
    /// Returns a `Some(Token::Invalid)` if there is a lexer error
    pub fn next(&mut self) -> Result<Spanned<Token<'src>>, UnexpectedEOF<'src>> {
        if let Some(it) = self.toks.next() {
            let (token, span) = it;
            Ok(if let Ok(token) = token {
                token.spanned(span)
            } else {
                Token::Invalid.spanned(span)
            })
        } else {
            Err(UnexpectedEOF {
                expected: None,
                expected_name: None,
            })
        }
    }

    /// Only use if you are sure at compile time that this cannot fail
    pub fn next_assert(
        &mut self,
        expected: &ExpectedTokens<'src>,
        expected_name: Option<&str>,
    ) -> Spanned<Token<'src>> {
        if let Some(it) = self.toks.next() {
            let (token, span) = it;
            if let Ok(token) = token {
                let _match_expected = &expected.expected;
                return if matches!(&token, _match_expected) {
                    token.spanned(span)
                } else {
                    panic!(
                        "{:?}",
                        CompilerError::Unexpected(UnexpectedToken {
                            expected: expected.clone(),
                            received: token.spanned(span),
                            expected_name: expected_name.map(|str| str.to_owned()),
                        })
                    )
                };
            } else {
                panic!(
                    "{:#?}",
                    CompilerError::LexerError(LexerError::new(Spanned::<()>::empty(span)))
                )
            }
        } else {
            panic!(
                "{:#?}",
                CompilerError::UnexpectedEOF(UnexpectedEOF {
                    expected: Some(expected.clone()),
                    expected_name: None,
                })
            )
        }
    }
}

#[derive(Debug)]
pub enum TokAdvanceError<'src> {
    UnexpectedEOF(UnexpectedEOF<'src>),
    Lexer(LexerError),
}

#[derive(Debug)]
pub struct RecoveryError<'src> {
    lexer_errors: Vec<LexerError>,
    unexpected_eof: Option<UnexpectedEOF<'src>>,
}

impl<'src> RecoveryError<'src> {
    pub fn new(lexer_errors: Vec<LexerError>, unexpected_eof: Option<UnexpectedEOF<'src>>) -> Self {
        Self {
            lexer_errors,
            unexpected_eof,
        }
    }
}
