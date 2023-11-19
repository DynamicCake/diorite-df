use crate::{lexer::Token, parser::error::{CompilerError, ExpectedTokens}, ast::{statement::{SimpleStatement, IfStatement, ActionType, Statement, Statements}, Spanned}};
use super::*;
use super::error::*;
use super::top::*;

use super::{Parser, error::CompilerResult};

impl<'src> Parser<'src> {

    pub fn statements(&mut self) -> CompilerResult<'src, Statements<'src>> {
        todo!()
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
        const EXPECTED: [Token<'_>; 12] = [
            Token::PlayerAction,
            Token::EntityAction,
            Token::GameAction,
            Token::Control,
            Token::CallFunction,
            Token::CallProcess,
            Token::Select,
            Token::SetVar,
            Token::IfPlayer,
            Token::IfEntity,
            Token::IfGame,
            Token::IfVar,
        ];
        match token.data {
            Token::PlayerAction
            | Token::EntityAction
            | Token::GameAction
            | Token::Control
            | Token::CallFunction
            | Token::CallProcess
            | Token::Select
            | Token::SetVar => {}
            Token::IfPlayer | Token::IfEntity | Token::IfGame | Token::IfVar => {}
            _ => {
                return CompilerResult::single_err(
                    None,
                    CompilerError::Unexpected {
                        expected: ExpectedTokens::new(EXPECTED.to_vec()),
                        received: token,
                        message: None,
                    },
                );
            }
        };

        // self.consume(expected, None);
        todo!()
    }

    fn regular_statement(&mut self) -> CompilerResult<'src, SimpleStatement<'src>> {
        const EXPECTED: [Token<'_>; 8] = [
            Token::PlayerAction,
            Token::EntityAction,
            Token::GameAction,
            Token::Control,
            Token::CallFunction,
            Token::CallProcess,
            Token::Select,
            Token::SetVar,
        ];
        let type_tok = match self.next_expect(&ExpectedTokens::new(EXPECTED.to_vec()), None) {
            Ok(it) => Spanned::new(ActionType::from_token(it.data).expect("A non "), it.span),
            Err(err) => return CompilerResult::single_err(None, err),
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
