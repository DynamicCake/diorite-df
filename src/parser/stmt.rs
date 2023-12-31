use std::ops::Range;

use super::error::*;
use super::top::*;
use super::*;
use crate::ast::recovery::Recovery;
use crate::ast::recovery::StatementRecovery;
use crate::ast::recovery::TopLevelRecovery;
use crate::ast::Iden;
use crate::ast::Parameters;
use crate::{
    ast::{
        statement::{ActionType, IfStatement, SimpleStatement, Statement, Statements},
        Spanned,
    },
    lexer::Token,
    parser::error::ExpectedTokens,
};

use super::{error::CompilerResult, Parser};

impl<'src> Parser<'src> {
    pub fn statements(
        &mut self,
    ) -> CompilerResult<'src, Vec<Spanned<Statement<'src>>>, Vec<UnexpectedToken<'src>>> {
        let mut statements = Vec::new();
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
                            Err(err) => {
                                let span = err
                                    .calc_span()
                                    .expect("statement recovery shouldn't be empty");
                                statements.push(Spanned::new(Statement::Recovery(err), span))
                            }
                        };
                        // Because it is in a loop, a break will happen if at_eof is some
                        if let Some(at_eof) = at_eof {
                            return CompilerResult::new_with_eof(statements, errors, Some(at_eof));
                        }
                    }
                }
                Err(err) => {
                    match err {
                        AdvanceUnexpected::Token(err) => {
                            errors.push(err);
                        }
                        AdvanceUnexpected::Eof(err) => {
                            return CompilerResult::new_with_eof(
                                statements,
                                errors,
                                Some(Box::new(err)),
                            )
                        }
                    }
                    let CompilerResult {
                        data,
                        error: _,
                        at_eof,
                    } = self.statement_recovery(Vec::new());

                    // If the tokens are empty, there is no reason to push them to the output as no
                    // processing is going to be done on them
                    if data.tokens.is_empty() {
                        break;
                    } else {
                        let span = data.calc_span().expect("data is non empty");
                        statements.push(Spanned::new(Statement::Recovery(data), span));
                    }
                    if let Some(at_eof) = at_eof {
                        return CompilerResult::new_with_eof(statements, errors, Some(at_eof));
                    }
                }
            }
        }

        CompilerResult::new(statements, errors)
    }

    fn statement_recovery(
        &mut self,
        mut tokens: Vec<Spanned<Token<'src>>>,
    ) -> CompilerResult<'src, StatementRecovery<'src>, ()> {
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
                        let a = self.advance().expect("Peek succeeded before");
                        tokens.push(Spanned::new(a.data, a.span));
                    }
                },
                Err(err) => {
                    return CompilerResult::new_with_eof(
                        StatementRecovery::new(tokens),
                        (),
                        Some(Box::new(err)),
                    );
                }
            };
        }
        CompilerResult::new(StatementRecovery::new(tokens), ())
    }

    fn statement(
        &mut self,
    ) -> CompilerResult<
        'src,
        Result<Spanned<Statement<'src>>, StatementRecovery<'src>>,
        Vec<UnexpectedToken<'src>>,
    > {
        let decl_token = match self.peek_expect(&Token::STATEMENT, Some("statements")) {
            Ok(it) => it.data.to_owned().spanned(it.span),
            Err(err) => match err {
                AdvanceUnexpected::Token(err) => {
                    return CompilerResult::new(Err(StatementRecovery::empty()), vec![err])
                }
                AdvanceUnexpected::Eof(err) => {
                    return CompilerResult::new_with_eof(
                        Err(StatementRecovery::empty()),
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
                let res = self.regular_statement();
            }
            Token::IfPlayer | Token::IfEntity | Token::IfGame | Token::IfVar => {
                self.if_statement();
                todo!()
            }
            _ => {
                let CompilerResult {
                    data,
                    error: _,
                    at_eof,
                } = self.statement_recovery(Vec::new());
                return CompilerResult::new_with_eof(Err(data), Vec::new(), at_eof);
            }
        };

        // self.consume(expected, None);
        todo!()
    }

    fn regular_statement(
        &mut self,
    ) -> CompilerResult<
        'src,
        Result<SimpleStatement<'src>, StatementRecovery<'src>>,
        Vec<UnexpectedToken<'src>>,
    > {
        let type_tok = self.next_assert(&Token::SIMPLE_STATEMENT, Some("simple statement"));

        let action = match self.next_expect(&[Token::Iden("")], None) {
            Ok(it) => it.map_inner(|i| match i {
                Token::Iden(data) => Iden::new(data),
                it => panic!("{:?} must be an Iden", it),
            }),
            Err(err) => return match err {
                AdvanceUnexpected::Token(err) => {
                    let CompilerResult {
                        data,
                        error: _,
                        at_eof,
                    } = self.statement_recovery(vec![type_tok]);
                    CompilerResult::new_with_eof(Err(data), vec![err], at_eof)
                }
                AdvanceUnexpected::Eof(err) => {
                    CompilerResult::new_with_eof(Err(StatementRecovery::new(vec![type_tok])), Vec::new(), Some(Box::new(err)))
                }
            },
        };

        let type_tok = type_tok.map_inner(|inner| {
            ActionType::from_token(inner).expect("A non action token managed to sneak in")
        });

        CompilerResult::new(
            Ok(SimpleStatement {
                type_tok,
                action,
                selection: None,
                tags: None,
                params: Spanned::new(Parameters { items: Vec::new() }, Range { start: 0, end: 0 }),
            }),
            Vec::new(),
        )
    }

    fn if_statement(&mut self) -> CompilerResult<'src, IfStatement<'src>, UnexpectedToken<'src>> {
        const EXPECTED: [Token<'_>; 4] = [
            Token::IfPlayer,
            Token::IfEntity,
            Token::IfGame,
            Token::IfVar,
        ];
        todo!()
    }
}
