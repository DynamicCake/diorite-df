use std::iter::Peekable;
use std::sync::Arc;

use lasso::ThreadedRodeo;
use logos::{Lexer, SpannedIter};

use crate::span::{SpanSize, Spanned};
use crate::tree::Program;
use crate::{lexer::Token, tree::top::TopLevel};

use self::error::*;

pub mod error;
pub mod helper;
pub mod stmt;
pub mod top;

pub struct Parser<'lex> {
    /// The main token iterator
    /// It is not advised to use this in functions called by `parse(&mut self)`
    toks: Peekable<SpannedIter<'lex, Token>>,
    // source: &'src str,
    /// Whenever an invalid token is replaced with `Token::Invalid`, a lexer error gets added
    lex_errs: Vec<LexerError>,
    file: Arc<str>,
}

mod ext {
    macro_rules! adv_stmt {
        ($params:expr, $func:expr) => {
            match $func {
                Ok(it) => it,
                Err(err) => return helper::recover_statement($params, err),
            }
        };
    }

    macro_rules! adv_top {
        ($params:expr, $func:expr) => {
            match $func {
                Ok(it) => it,
                Err(err) => return helper::recover_top_level($params, err),
            }
        };
    }

    macro_rules! ret_err {
        ($expr:expr) => {
            match $expr {
                Ok(it) => it,
                Err(err) => return err,
            }
        };
    }

    macro_rules! should_return {
        ($expr:expr) => {
            match helper::should_return_func($expr) {
                Ok(it) => it,
                Err(err) => return err,
            }
        };
    }

    macro_rules! should_return_top {
        ($expr:expr) => {
            match helper::should_return_top_func($expr) {
                Ok(it) => it,
                Err(err) => return err,
            }
        };
    }

    pub(crate) use adv_stmt;
    pub(crate) use adv_top;
    pub(crate) use should_return;
    pub(crate) use should_return_top;
}

impl<'lex> Parser<'lex> {
    pub fn new(lexer: Lexer<'lex, Token>, file: Arc<str>) -> Self {
        Self {
            // source: lexer.source(),
            toks: lexer.spanned().peekable(),
            lex_errs: Vec::new(),
            file,
        }
    }

    pub fn parse(&mut self) -> ParseResult<Program, Vec<UnexpectedToken>> {
        let mut stmts = Vec::new();
        let mut errors = Vec::new();
        loop {
            // err is intentionally ignored to allow an empty file
            if let Err(_err) = self.peek() {
                break;
            }

            let ParseResult {
                data,
                mut error,
                at_eof,
            } = self.top_level();
            errors.append(&mut error);
            stmts.push(data);

            if let Some(at_eof) = at_eof {
                return ParseResult::new(Program::new(stmts), errors, Some(at_eof));
            }
        }
        ParseResult::new(Program::new(stmts), errors, None)
    }

    /// Only use if you are sure at compile time that this cannot fail
    pub fn next_assert(&mut self, expected: &[Token]) -> Spanned<Token> {
        if let Some(it) = self.toks.next() {
            let (token, span) = it;
            if let Ok(token) = token {
                let span = (span.start as SpanSize)..(span.end as SpanSize);
                return if expected.contains(&token) {
                    token.spanned(span)
                } else {
                    panic!(
                        "Unexpected Error: {:#?}",
                        UnexpectedToken {
                            expected: ExpectedTokens::new(expected.into()),
                            received: token.spanned(span),
                            expected_name: None,
                            file: self.file.clone(),
                        }
                    )
                };
            } else {
                let span = (span.start as SpanSize)..(span.end as SpanSize);
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
        expected: &[Token],
        expected_name: Option<&str>,
    ) -> Result<Spanned<Token>, AdvanceUnexpected> {
        if let Some(it) = self.toks.next() {
            let (token, span) = it;
            let span = (span.start as SpanSize)..(span.end as SpanSize);
            if let Ok(token) = token {
                return if expected.contains(&token) {
                    Ok(token.spanned(span))
                } else {
                    Err(AdvanceUnexpected::Token(UnexpectedToken {
                        expected: ExpectedTokens::new(expected.into()),
                        received: token.spanned(span),
                        expected_name: expected_name.map(|str| str.into()),
                        file: self.file.clone(),
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
        expected: &[Token],
        msg: Option<&str>,
    ) -> Result<Spanned<&Token>, AdvanceUnexpected> {
        if let Some((token, span)) = self.toks.peek() {
            if let Ok(token) = token {
                let span = (span.start as SpanSize)..(span.end as SpanSize);
                return if expected.contains(token) {
                    Ok(Spanned::new(token, span.clone()))
                } else {
                    Err(AdvanceUnexpected::Token(UnexpectedToken {
                        expected: ExpectedTokens::new(expected.into()),
                        received: token.clone().spanned(span.clone()),
                        expected_name: msg.map(|str| str.into()),
                        file: self.file.clone(),
                    }))
                };
            } else {
                let span = (span.start as SpanSize)..(span.end as SpanSize);
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
    pub fn peek(&mut self) -> Result<Spanned<&Token>, UnexpectedEOF> {
        if let Some(it) = self.toks.peek() {
            let (token, span) = it;
            if let Ok(token) = token {
                let span = (span.start as SpanSize)..(span.end as SpanSize);
                let spanned = Spanned::new(token, span.clone());
                Ok(spanned)
            } else {
                let span = (span.start as SpanSize)..(span.end as SpanSize);
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
    pub fn advance(&mut self) -> Result<Spanned<Token>, UnexpectedEOF> {
        if let Some(it) = self.toks.next() {
            let (token, span) = it;
            let span = (span.start as SpanSize)..(span.end as SpanSize);
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

#[derive(Debug)]
pub enum AdvanceUnexpected {
    Token(UnexpectedToken),
    Eof(UnexpectedEOF),
}
