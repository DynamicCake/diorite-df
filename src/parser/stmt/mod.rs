use super::*;
use crate::ast::recovery::StatementRecovery;
use crate::{
    ast::{
        statement::Statement,
        Spanned,
    },
    lexer::Token,
};

pub mod common;
pub mod kind;

use super::{error::CompilerResult, Parser};

impl<'src> Parser<'src> {
    pub fn statements(
        &mut self,
    ) -> CompilerResult<'src, Vec<Statement<'src>>, Vec<UnexpectedToken<'src>>> {
        let mut statements: Vec<Statement<'src>> = Vec::new();
        let mut errors = Vec::new();

        loop {
            match self.peek_expect(&Token::STATEMENT_LOOP, Some("statement declaration or end")) {
                Ok(it) => {
                    // Tokens are guaranteed to be end or statement
                    if let Token::End = it.data {
                        break;
                    } else {
                        let CompilerResult {
                            data,
                            mut error,
                            at_eof,
                        } = self.statement();
                        errors.append(&mut error);
                        match data {
                            Ok(it) => {
                                statements.push(it);
                            }
                            Err(_err) => statements.push(Statement::Recovery),
                        };
                        // Because it is in a loop, a break will happen if at_eof is some
                        if let Some(at_eof) = at_eof {
                            return CompilerResult::new(statements, errors, Some(at_eof));
                        }
                    }
                }
                Err(err) => {
                    match err {
                        AdvanceUnexpected::Token(err) => {
                            errors.push(err);
                        }
                        AdvanceUnexpected::Eof(err) => {
                            return CompilerResult::new(statements, errors, Some(Box::new(err)))
                        }
                    }

                    statements.push(Statement::Recovery);
                    if let Some(at_eof) = self.statement_recovery() {
                        return CompilerResult::new(statements, errors, Some(at_eof));
                    }
                }
            }
        }

        CompilerResult::new(statements, errors, None)
    }

    fn statement(
        &mut self,
    ) -> CompilerResult<'src, Result<Statement<'src>, StatementRecovery>, Vec<UnexpectedToken<'src>>>
    {
        let decl_token = match self.peek_expect(&Token::STATEMENT, Some("statements")) {
            Ok(it) => it.data.to_owned().spanned(it.span),
            Err(err) => match err {
                AdvanceUnexpected::Token(err) => {
                    return CompilerResult::new(Err(StatementRecovery), vec![err], None)
                }
                AdvanceUnexpected::Eof(err) => {
                    return CompilerResult::new(
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
                let CompilerResult {
                    data,
                    error,
                    at_eof,
                } = self.regular_statement();

                match data {
                    Ok(it) => {
                        let span = it.calc_span();
                        CompilerResult::new(
                            Ok(Statement::Simple(Spanned::new(it, span))),
                            error,
                            at_eof,
                        )
                    }
                    Err(err) => CompilerResult::new(Err(err), error, at_eof),
                }
            }
            Token::IfPlayer | Token::IfEntity | Token::IfGame | Token::IfVar => {
                self.if_statement();
                todo!()
            }
            _ => {
                let at_eof = self.statement_recovery();
                return CompilerResult::new(Err(StatementRecovery), Vec::new(), at_eof);
            }
        }

        // self.consume(expected, None);
        // CompilerResult::new(Ok(Spanned::new(, 1..2)), Vec::new(), None)
    }


    pub fn statement_recovery(&mut self) -> Option<Box<UnexpectedEOF<'src>>> {
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
