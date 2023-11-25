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
    parser::error::{CompilerError, ExpectedTokens},
};

use super::{error::CompilerResult, Parser};

impl<'src> Parser<'src> {
    pub fn statements(
        &mut self,
    ) -> CompilerResult<Vec<Spanned<Statement<'src>>>, CompilerError<'src>> {
        let expected = {
            let mut expected = Token::STATEMENT.to_vec();
            expected.push(Token::End);
            ExpectedTokens::new(expected)
        };

        let mut statements: Vec<Spanned<Statement<'src>>> = Vec::new();
        let mut errors: Vec<CompilerError<'src>> = Vec::new();

        loop {
            match self.peek_expect(&expected, Some("statement declaration or end")) {
                Ok(it) => {
                    // Tokens are guaranteed to be end or statement
                    if let Token::End = it.data {
                        break;
                    } else {
                        let CompilerResult { data, mut error } = self.statement();
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
                    }
                }
                Err(err) => {
                    errors.push(err);
                    let CompilerResult { data, mut error } = self.statement_recovery(Vec::new());
                    errors.append(&mut error.into_iter().map(|it| it.into()).collect());

                    // If the tokens are empty, there is no reason to push them to the output as no
                    // processing is going to be done on them
                    if data.tokens.is_empty() {
                        break;
                    } else {
                        let span = data.calc_span().expect("data is non empty");
                        statements.push(Spanned::new(Statement::Recovery(data), span));
                    }
                }
            }
        }

        CompilerResult::new(statements, errors)
    }

    fn statement_recovery(
        &mut self,
        mut tokens: Vec<Spanned<Token<'src>>>,
    ) -> CompilerResult<StatementRecovery<'src>, TokAdvanceError<'src>> {
        let mut errors = Vec::new();
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
                        let a = self.next().expect("Peek succeeded before");
                        tokens.push(Spanned::new(a.data, a.span));
                    }
                },
                Err(err) => {
                    match &err {
                        TokAdvanceError::UnexpectedEOF(UnexpectedEOF {
                            expected: _,
                            expected_name: _,
                        }) => {
                            errors.push(err);
                            return CompilerResult::new(StatementRecovery::new(tokens), errors);
                        }
                        TokAdvanceError::Lexer(span) => {
                            tokens.push(Token::Invalid.spanned(span.token.span.clone()));
                        }
                    }
                    errors.push(err);
                }
            };
        }
        CompilerResult::new(StatementRecovery::new(tokens), errors)
    }

    fn statement(
        &mut self,
    ) -> CompilerResult<
        Result<Spanned<Statement<'src>>, StatementRecovery<'src>>,
        CompilerError<'src>,
    > {
        let decl_token = match self.peek_expect(
            &ExpectedTokens::new(Token::STATEMENT.to_vec()),
            Some("statements"),
        ) {
            Ok(it) => it.data.to_owned().spanned(it.span),
            Err(err) => return CompilerResult::single_err(Err(StatementRecovery::empty()), err),
        };

        // I am aware that this will become a nightmare when adding new tokens or features... Too bad!

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
                let CompilerResult { data, error } = self.statement_recovery(Vec::new());
                let mut errors: Vec<CompilerError<'src>> =
                    error.into_iter().map(|it| it.into()).collect();
                errors.push(CompilerError::Unexpected(UnexpectedToken {
                    expected: ExpectedTokens::new(Token::STATEMENT.to_vec()),
                    received: decl_token,
                    expected_name: None,
                }));

                return CompilerResult::new(Err(data), errors);
            }
        };

        // self.consume(expected, None);
        todo!()
    }

    fn regular_statement(
        &mut self,
    ) -> CompilerResult<Result<SimpleStatement<'src>, StatementRecovery<'src>>, CompilerError<'src>>
    {
        let type_tok =
            self.next_assert(&ExpectedTokens::new(Token::SIMPLE_STATEMENT.to_vec()), None);

        let action = match self.next_expect(&Token::Iden("").into(), None) {
            Ok(it) => it.map_inner(|i| match i {
                Token::Iden(data) => Iden::new(data),
                it => panic!("{:?} must be an Iden", it),
            }),
            Err(err) => {
                let CompilerResult { data, mut error } = self.statement_recovery(vec![type_tok]);
                let mut errors: Vec<CompilerError<'src>> =
                    error.into_iter().map(|it| it.into()).collect();
                errors.push(err);
                return CompilerResult::new(Err(data), errors);
            }
        };

        let mut errors = Vec::new();

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
            errors,
        )
    }

    fn if_statement(&mut self) -> CompilerResult<IfStatement<'src>, CompilerError<'src>> {
        const EXPECTED: [Token<'_>; 4] = [
            Token::IfPlayer,
            Token::IfEntity,
            Token::IfGame,
            Token::IfVar,
        ];
        todo!()
    }
}
