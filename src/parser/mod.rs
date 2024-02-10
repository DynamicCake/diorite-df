use std::iter::Peekable;

use logos::{Lexer, SpannedIter};

use crate::ast::{Program, Spanned};
use crate::{ast::top::TopLevel, lexer::Token};

use self::error::*;

pub mod error;
pub mod helper;
pub mod stmt;
pub mod top;

pub struct Parser<'src> {
    /// The main token iterator
    /// It is not advised to use this in functions called by `parse(&mut self)`
    toks: Peekable<SpannedIter<'src, Token<'src>>>,
    // source: &'src str,
    /// Whenever an invalid token is replaced with `Token::Invalid`, a lexer error gets added
    lex_errs: Vec<LexerError>,
}

impl<'src> Parser<'src> {
    pub fn new(lexer: Lexer<'src, Token<'src>>) -> Self {
        Self {
            // source: lexer.source(),
            toks: lexer.spanned().peekable(),
            lex_errs: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> CompilerResult<'src, Program<'src>, Vec<UnexpectedToken<'src>>> {
        let mut stmts = Vec::new();
        let mut errors = Vec::new();
        loop {
            // err is intentionally ignored to allow an empty file
            if let Err(_err) = self.peek() {
                break;
            }

            let CompilerResult {
                data,
                mut error,
                at_eof,
            } = self.top_level();
            errors.append(&mut error);
            stmts.push(data);

            if let Some(at_eof) = at_eof {
                return CompilerResult::new(Program::new(stmts), errors, Some(at_eof));
            }
        }
        CompilerResult::new(Program::new(stmts), errors, None)
    }

    /// Only use if you are sure at compile time that this cannot fail
    pub fn next_assert(
        &mut self,
        expected: &[Token<'src>],
        expected_name: Option<&str>,
    ) -> Spanned<Token<'src>> {
        if let Some(it) = self.toks.next() {
            let (token, span) = it;
            if let Ok(token) = token {
                return if expected.contains(&token) {
                    token.spanned(span)
                } else {
                    panic!(
                        "Unexpected Error: {:#?}",
                        UnexpectedToken {
                            expected: ExpectedTokens::new(expected.into()),
                            received: token.spanned(span),
                            expected_name: expected_name.map(|str| str.to_owned()),
                        }
                    )
                };
            } else {
                panic!(
                    "Unexpected Error: {:#?}",
                    LexerError::new(Spanned::<()>::empty(span))
                )
            }
        } else {
            panic!(
                "Unexpected Error: {:#?}",
                UnexpectedEOF {
                    expected: Some(ExpectedTokens::new(expected.into())),
                    expected_name: None,
                }
            )
        }
    }

    /// Used when you know what next token you expect
    /// ```diroite
    /// paction Name ()
    ///    HERE ^^^^
    /// ```
    /// If you are 100% sure during compiler time use next_assert
    pub fn next_expect(
        &mut self,
        expected: &[Token<'src>],
        expected_name: Option<&str>,
    ) -> Result<Spanned<Token<'src>>, AdvanceUnexpected<'src>> {
        if let Some(it) = self.toks.next() {
            let (token, span) = it;
            if let Ok(token) = token {
                return if expected.contains(&token) {
                    Ok(token.spanned(span))
                } else {
                    Err(AdvanceUnexpected::Token(UnexpectedToken {
                        expected: ExpectedTokens::new(expected.into()),
                        received: token.spanned(span),
                        expected_name: expected_name.map(|str| str.to_owned()),
                    }))
                };
            } else {
                self.lex_errs
                    .push(LexerError::new(Spanned::<()>::empty(span.clone())));
                Ok(Token::Invalid.spanned(span))
            }
        } else {
            Err(AdvanceUnexpected::Eof(UnexpectedEOF {
                expected: Some(ExpectedTokens::new(expected.into())),
                expected_name: None,
            }))
        }
    }

    pub fn peek_expect(
        &mut self,
        expected: &[Token<'src>],
        msg: Option<&str>,
    ) -> Result<Spanned<&Token<'src>>, AdvanceUnexpected<'src>> {
        if let Some((token, span)) = self.toks.peek() {
            if let Ok(token) = token {
                return if expected.contains(token) {
                    Ok(Spanned::new(token, span.clone()))
                } else {
                    Err(AdvanceUnexpected::Token(UnexpectedToken {
                        expected: ExpectedTokens::new(expected.into()),
                        received: token.clone().spanned(span.clone()),
                        expected_name: msg.map(|str| str.to_owned()),
                    }))
                };
            } else {
                // This clone has minimal overhead as it is only cloning a Range<usize>
                self.lex_errs
                    .push(LexerError::new(Spanned::<()>::empty(span.clone())));
                Ok(Spanned::new(&Token::Invalid, span.clone()))
            }
        } else {
            Err(AdvanceUnexpected::Eof(UnexpectedEOF {
                expected: Some(ExpectedTokens::new(expected.into())),
                expected_name: msg.map(|it| it.to_string()),
            }))
        }
    }

    /// Returns a reference to the next() Token without advancing the iterator.
    /// On lexer error, it pushes it onto `self.lex_errs` and returns `Token::Invalid`
    pub fn peek(&mut self) -> Result<Spanned<&Token<'src>>, UnexpectedEOF<'src>> {
        if let Some(it) = self.toks.peek() {
            let (token, span) = it;
            if let Ok(token) = token {
                let spanned = Spanned::new(token, span.clone());
                Ok(spanned)
            } else {
                self.lex_errs
                    .push(LexerError::new(Spanned::<()>::empty(span.clone())));
                Ok(Spanned::new(&Token::Invalid, span.clone()))
            }
        } else {
            Err(UnexpectedEOF {
                expected: None,
                expected_name: None,
            })
        }
    }

    /// Advances to the iterator to the next token
    /// Great for use in recovery functions
    /// Returns a `Some(Token::Invalid)` if there is a lexer error
    pub fn advance(&mut self) -> Result<Spanned<Token<'src>>, UnexpectedEOF<'src>> {
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
}

pub enum AdvanceUnexpected<'src> {
    Token(UnexpectedToken<'src>),
    Eof(UnexpectedEOF<'src>),
}
