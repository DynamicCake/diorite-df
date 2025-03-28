//! Crates a new parse tree using the [crate::tree] module

use std::hash::{Hash, Hasher};
use std::iter::Peekable;

use lasso::Spur;
use logos::{Lexer, SpannedIter};
use rustc_hash::FxHasher;

use crate::common::prelude::*;
use crate::error::syntax::{
    ExpectedTokens, LexerError, ParseResult, UnexpectedEOF, UnexpectedToken,
};
use crate::tree::TreeRoot;
use crate::{lexer::Token, tree::top::TreeTopLevel};

pub mod helper;
pub mod stmt;
mod test;
pub mod top;

/// Converts tokens from a file to a parse tree (not an abstract syntax tree)
pub struct Parser<'lex> {
    /// The main token iterator
    /// It is not advised to use this in functions called by [parse(mut self)](Parser::parse)
    /// Use the helper methods instead
    toks: Peekable<SpannedIter<'lex, Token>>,
    /// Whenever an invalid token is replaced with [Token::Invalid](crate::lexer::Token), a lexer error gets added
    lex_errs: Vec<LexerError>,
    /// The file this parser belongs to
    path: Spur,
}

#[derive(Debug, PartialEq)]
pub struct ParsedFile {
    /// The parse tree made from the parser
    /// Unfortunately having only a parse tree is not possible as some other things need to be
    /// transformed like `loc(10, 0, -10)` which could become invalid
    pub root: TreeRoot,
    /// List of lexer errors caused by invalid tokens
    pub lex_errs: Vec<LexerError>,
    /// List of unexpected token errors
    pub parse_errs: Vec<UnexpectedToken>,
    /// Indicates weather an error happened at the end of file
    /// It was found that this is better than putting eof errors in [parse_errs](ParsedFile::parse_errs)
    /// Because:
    /// 1. there can only be one EOF error
    /// 2. It stops the parsers from attempting recovery by checking if it is some
    /// 3. [parse_errs](ParsedFile::parse_errs) no longer needs to be an Vec<enum>
    pub at_eof: Option<Box<UnexpectedEOF>>,
}

impl ParsedFile {
    pub fn is_successful(&self) -> bool {
        self.lex_errs.is_empty() && self.parse_errs.is_empty() && self.at_eof.is_none()
    }
}

impl<'lex> Parser<'lex> {
    pub fn parse(lexer: Lexer<'lex, Token>, file: Spur) -> ParsedFile {
        let src: &str = lexer.source();
        let mut hasher = FxHasher::default();
        src.hash(&mut hasher);
        let hash = hasher.finish();

        Self::new(lexer, file).parse_self()
    }
    pub fn new(lexer: Lexer<'lex, Token>, file: Spur) -> Self {
        Self {
            toks: lexer.spanned().peekable(),
            lex_errs: Vec::new(),
            path: file,
        }
    }

    /// Consume the token iterator and output a parsed file
    fn parse_self(mut self) -> ParsedFile {
        let mut stmts = Vec::new();
        let mut errors = Vec::new();
        loop {
            // to allow an empty file
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
                return ParsedFile {
                    root: TreeRoot::new(stmts),
                    lex_errs: self.lex_errs,
                    parse_errs: errors,
                    at_eof: Some(at_eof),
                };
            }
        }
        ParsedFile {
            root: TreeRoot::new(stmts),
            lex_errs: self.lex_errs,
            parse_errs: errors,
            at_eof: None,
        }
    }

    /// Advances the iterator and if it doesn't match, panic
    /// Useful for the first iterator advancement
    pub fn next_assert(&mut self, expected: &[Token]) -> Spanned<Token> {
        if let Some(it) = self.toks.next() {
            let (token, span) = it;
            if let Ok(token) = token {
                let span = (span.start as SpanSize)..(span.end as SpanSize);
                if expected.contains(&token) {
                    token.spanned(span)
                } else {
                    panic!(
                        "Unexpected Error: {:#?}",
                        UnexpectedToken {
                            expected: ExpectedTokens::new(expected.into()),
                            received: token.spanned(span),
                            expected_name: None,
                            file: self.path
                        }
                    )
                }
            } else {
                let span = (span.start as SpanSize)..(span.end as SpanSize);
                panic!(
                    "Unexpected Error: {:#?}",
                    LexerError::new(Referenced::<()>::empty(
                        Spanned::<()>::empty(span),
                        self.path
                    ))
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
    /// If you are sure that the next token is the one you use [Self::next_assert]
    pub fn next_expect(
        &mut self,
        expected: &[Token],
        expected_name: Option<&str>,
    ) -> Result<Spanned<Token>, AdvanceUnexpected> {
        if let Some(it) = self.toks.next() {
            let (token, span) = it;
            let span = (span.start as SpanSize)..(span.end as SpanSize);
            if let Ok(token) = token {
                if expected.contains(&token) {
                    Ok(token.spanned(span))
                } else {
                    Err(AdvanceUnexpected::Token(UnexpectedToken {
                        expected: ExpectedTokens::new(expected.into()),
                        received: token.spanned(span),
                        expected_name: expected_name.map(|str| str.into()),
                        file: self.path,
                    }))
                }
            } else {
                self.lex_errs.push(LexerError::new(Referenced::<()>::empty(
                    Spanned::<()>::empty(span.clone()),
                    self.path,
                )));
                Ok(Token::Invalid.spanned(span))
            }
        } else {
            Err(AdvanceUnexpected::Eof(UnexpectedEOF {
                expected: Some(ExpectedTokens::new(expected.into())),
                expected_name: None,
            }))
        }
    }

    /// Get the next token without advancing the iterator
    pub fn peek_expect(
        &mut self,
        expected: &[Token],
        msg: Option<&str>,
    ) -> Result<Spanned<&Token>, AdvanceUnexpected> {
        if let Some((token, span)) = self.toks.peek() {
            if let Ok(token) = token {
                let span = (span.start as SpanSize)..(span.end as SpanSize);
                if expected.contains(token) {
                    Ok(Spanned::new(token, span.clone()))
                } else {
                    Err(AdvanceUnexpected::Token(UnexpectedToken {
                        expected: ExpectedTokens::new(expected.into()),
                        received: token.clone().spanned(span.clone()),
                        expected_name: msg.map(|str| str.into()),
                        file: self.path,
                    }))
                }
            } else {
                let span = (span.start as SpanSize)..(span.end as SpanSize);
                // This clone has minimal overhead as it is only cloning a Range<usize>
                self.lex_errs.push(LexerError::new(Referenced::<()>::empty(
                    Spanned::<()>::empty(span.clone()),
                    self.path,
                )));
                Ok(Spanned::new(&Token::Invalid, span.clone()))
            }
        } else {
            Err(AdvanceUnexpected::Eof(UnexpectedEOF {
                expected: Some(ExpectedTokens::new(expected.into())),
                expected_name: msg.map(|it| it.to_string()),
            }))
        }
    }

    /// Get a reference to the next token without advancing the iterator.
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
                self.lex_errs.push(LexerError::new(Referenced::<()>::empty(
                    Spanned::<()>::empty(span.clone()),
                    self.path,
                )));
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

/// Some macros to make writing the parser easier
mod ext {

    /// Pass in `self` for the first parameter and the advancement function
    /// If the function fails, it starts recovery
    macro_rules! adv_stmt {
        ($params:expr, $func:expr) => {
            match $func {
                Ok(it) => it,
                Err(err) => return helper::recover_statement($params, err),
            }
        };
    }

    /// [adv_stmt] but for top level
    macro_rules! adv_top {
        ($params:expr, $func:expr) => {
            match $func {
                Ok(it) => it,
                Err(err) => return helper::recover_top_level($params, err),
            }
        };
    }

    /// If ok, produce value
    /// If err, return from the function with the error
    /// Now phased out and there really isn't a reason to use this
    macro_rules! ret_err {
        ($expr:expr) => {
            match $expr {
                Ok(it) => it,
                Err(err) => return err,
            }
        };
    }

    /// Run a [should_return_func](crate::parser::helper::should_return_func) and then return from
    /// the function calling this if there is an error
    macro_rules! should_return {
        ($expr:expr) => {
            match helper::should_return_func($expr) {
                Ok(it) => it,
                Err(err) => return err,
            }
        };
    }

    /// [should_return] but for the top level
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
