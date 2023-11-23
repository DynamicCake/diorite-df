use super::error::*;
use super::top::*;
use super::*;
use crate::ast::recovery::StatementRecovery;
use crate::ast::recovery::TopLevelRecovery;
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
    pub fn statements(&mut self) -> CompilerResult<'src, Statements<'src>> {
        let expected = {
            let mut expected = Token::STATEMENT.to_vec();
            expected.push(Token::End);
            ExpectedTokens::new(expected)
        };

        let mut statements = Vec::new();
        let mut errors = Vec::new();

        loop {
            match self.peek_expect(&expected, Some("Expected statement declaration or end")) {
                Ok(it) => {
                    // Tokens are guaranteed to be end or statement
                    if let Token::End = it.data {
                        break;
                    } else {
                        let mut result = self.statement();
                        errors.append(&mut result.error);
                        if let Some(it) = result.data {
                            statements.push(it);
                        };
                    }
                }
                Err(err) => {
                    errors.push(err);
                    let mut result = self.statement_recovery(Vec::new());
                    errors.append(&mut result.error);
                    if let Some(it) = result.data {
                        statements.push(Statement::Recovery(it));
                    }
                }
            }
        }

        CompilerResult::new(Some(Statements::new(statements)), errors);
        todo!();
    }

    fn statement_recovery(
        &mut self,
        mut tokens: Vec<Spanned<Token<'src>>>,
    ) -> CompilerResult<'src, StatementRecovery<'src>> {
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
                        CompilerError::Unexpected {
                            expected: _,
                            received: _,
                            expected_name: _,
                        } => panic!("self.next() cannot return CompilerError::Unexpected"),
                        CompilerError::UnexpectedEOF {
                            expected: _,
                            expected_name: _,
                        } => {
                            return CompilerResult::single_err(
                                Some(StatementRecovery::new(tokens)),
                                err,
                            );
                        }
                        CompilerError::LexerError(span) => {
                            tokens.push(Token::Invalid.spanned(span.span.clone()));
                        }
                    }
                    errors.push(err);
                }
            };
        }
        CompilerResult::new(Some(StatementRecovery::new(tokens)), errors)
    }

    fn statement(&mut self) -> CompilerResult<'src, Statement<'src>> {
        let token = match self.peek() {
            Ok(it) => {
                let span = it.span;
                let data = it.data.to_owned();
                data.spanned(span)
            }
            Err(err) => return CompilerResult::single_err(None, err),
        };

        // I am aware that this will become a nightmare when adding new tokens or features... Too bad!

        match token.data {
            Token::PlayerAction
            | Token::EntityAction
            | Token::GameAction
            | Token::Control
            | Token::CallFunction
            | Token::CallProcess
            | Token::Select
            | Token::SetVar => {
                self.regular_statement();
                todo!()
            }
            Token::IfPlayer | Token::IfEntity | Token::IfGame | Token::IfVar => {
                self.if_statement();
                todo!()
            }
            _ => {
                return CompilerResult::single_err(
                    None,
                    CompilerError::Unexpected {
                        expected: ExpectedTokens::new(Token::STATEMENT.to_vec()),
                        received: token,
                        expected_name: None,
                    },
                );
            }
        };

        // self.consume(expected, None);
        todo!()
    }

    fn regular_statement(&mut self) -> CompilerResult<'src, SimpleStatement<'src>> {
        let type_tok = match self
            .next_expect(&ExpectedTokens::new(Token::SIMPLE_STATEMENT.to_vec()), None)
        {
            Ok(it) => Spanned::new(
                ActionType::from_token(it.data).expect("A non action token managed to sneak in"),
                it.span,
            ),
            Err(err) => return CompilerResult::single_err(None, err),
        };

        let action = match self.next_expect(&Token::Iden("").into(), None) {
            Ok(it) => {}
            Err(err) => {}
        };

        let mut errors = Vec::new();
        CompilerResult::new(
            Some(SimpleStatement {
                type_tok,
                action: todo!(),
                selection: todo!(),
                tags: todo!(),
                params: todo!(),
            }),
            errors,
        )
    }

    fn if_statement(&mut self) -> CompilerResult<'src, IfStatement<'src>> {
        const EXPECTED: [Token<'_>; 4] = [
            Token::IfPlayer,
            Token::IfEntity,
            Token::IfGame,
            Token::IfVar,
        ];
        todo!()
    }
}
