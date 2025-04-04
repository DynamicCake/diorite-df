use super::*;
use crate::common::span::CalcSpan;
use crate::error::syntax::{ParseResult, UnexpectedEOF, UnexpectedToken};
use crate::tree::recovery::StatementRecovery;
use crate::{lexer::Token, tree::statement::TreeStatement};

pub mod common;
pub mod kind;

use super::Parser;

impl<'lex> Parser<'lex> {
    pub fn statements(&mut self, in_else: bool) -> ParseResult<Vec<TreeStatement>> {
        let mut statements: Vec<TreeStatement> = Vec::new();
        let mut errors = Vec::new();

        loop {
            let which_one = if in_else {
                self.peek_expect(
                    &Token::IF_BODY_LOOP,
                    Some("statement declaration, end or else"),
                )
            } else {
                self.peek_expect(&Token::STATEMENT_LOOP, Some("statement declaration or end"))
            };
            match which_one {
                Ok(it) => {
                    // Tokens are guaranteed to be end or statement
                    match it.data {
                        Token::End | Token::Else => {
                            break;
                        }
                        _ => {
                            let ParseResult {
                                data,
                                mut error,
                                at_eof,
                            } = self.statement();
                            errors.append(&mut error);
                            match data {
                                Ok(it) => {
                                    statements.push(it);
                                }
                                Err(_err) => statements.push(TreeStatement::Recovery),
                            };
                            // Because it is in a loop, a break will happen if at_eof is some
                            if let Some(at_eof) = at_eof {
                                return ParseResult::new(statements, errors, Some(at_eof));
                            }
                        }
                    };
                }
                Err(err) => {
                    match err {
                        AdvanceUnexpected::Token(err) => {
                            errors.push(err);
                        }
                        AdvanceUnexpected::Eof(err) => {
                            return ParseResult::new(statements, errors, Some(Box::new(err)))
                        }
                    }

                    statements.push(TreeStatement::Recovery);
                    if let Some(at_eof) = self.statement_recovery() {
                        return ParseResult::new(statements, errors, Some(at_eof));
                    }
                }
            }
        }

        ParseResult::new(statements, errors, None)
    }

    fn statement(
        &mut self,
    ) -> ParseResult<Result<TreeStatement, StatementRecovery>, Vec<UnexpectedToken>> {
        let decl_token = match self.peek_expect(&Token::STATEMENT, Some("statements")) {
            Ok(it) => it.data.to_owned().spanned(it.span),
            Err(err) => match err {
                AdvanceUnexpected::Token(err) => {
                    return ParseResult::new(Err(StatementRecovery), vec![err], None)
                }
                AdvanceUnexpected::Eof(err) => {
                    return ParseResult::new(
                        Err(StatementRecovery),
                        Vec::new(),
                        Some(Box::new(err)),
                    )
                }
            },
        };

        // I am aware that this will become a nightmare when adding new tokens or features... Too bad!
        // TODO create statement functions
        match decl_token.data {
            Token::PlayerAction
            | Token::EntityAction
            | Token::GameAction
            | Token::Control
            | Token::CallFunction
            | Token::CallProcess
            | Token::Select
            | Token::SetVar => {
                let ParseResult {
                    data,
                    error,
                    at_eof,
                } = self.regular_statement();

                match data {
                    Ok(it) => {
                        let span = it.calc_span();
                        ParseResult::new(
                            Ok(TreeStatement::Simple(Spanned::new(it, span))),
                            error,
                            at_eof,
                        )
                    }
                    Err(err) => ParseResult::new(Err(err), error, at_eof),
                }
            }
            Token::IfPlayer | Token::IfEntity | Token::IfGame | Token::IfVar => {
                let ParseResult {
                    data,
                    error,
                    at_eof,
                } = self.if_statement();

                match data {
                    Ok(it) => {
                        let span = it.calculate_span();
                        ParseResult::new(
                            Ok(TreeStatement::If(Spanned::new(it, span))),
                            error,
                            at_eof,
                        )
                    }
                    Err(err) => ParseResult::new(Err(err), error, at_eof),
                }
            }
            Token::Repeat => {
                let ParseResult {
                    data,
                    error,
                    at_eof,
                } = self.repeat();

                match data {
                    Ok(it) => {
                        let span = it.calculate_span();
                        ParseResult::new(
                            Ok(TreeStatement::Repeat(Spanned::new(it, span))),
                            error,
                            at_eof,
                        )
                    }
                    Err(err) => ParseResult::new(Err(err), error, at_eof),
                }
            }
            _ => {
                let at_eof = self.statement_recovery();
                ParseResult::new(Err(StatementRecovery), Vec::new(), at_eof)
            }
        }

        // self.consume(expected, None);
        // CompilerResult::new(Ok(Spanned::new(, 1..2)), Vec::new(), None)
    }

    pub fn statement_recovery(&mut self) -> Option<Box<UnexpectedEOF>> {
        loop {
            match self.peek() {
                Ok(tok) => match tok.data {
                    Token::PlayerAction
                    | Token::EntityAction
                    | Token::GameAction
                    | Token::Control
                    | Token::CallFunction
                    | Token::CallProcess
                    | Token::Select
                    | Token::SetVar
                    | Token::IfPlayer
                    | Token::IfEntity
                    | Token::IfGame
                    | Token::IfVar
                    | Token::Repeat
                    | Token::End => {
                        break;
                    }

                    _ => {
                        self.advance().expect("Peek succeeded before");
                    }
                },
                Err(err) => return Some(Box::new(err)),
            };
        }
        None
    }
}
