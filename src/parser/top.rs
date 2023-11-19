use crate::ast::{
    statement::{ActionType, SimpleStatement, Statement, IfStatement},
    top::{Event, EventType},
    Iden,
};

use super::*;
use super::error::*;
use super::stmt::*;

impl<'src> Parser<'src> {
    pub(super) fn top_level(&mut self) -> CompilerResult<'src, TopLevel<'src>> {
        let expected = ExpectedTokens::new(vec![
            Token::FuncDef,
            Token::ProcDef,
            Token::PlayerEvent,
            Token::EntityEvent,
        ]);

        let token = match self.next_expect(&expected, None) {
            Ok(it) => it,
            Err(err) => {
                return CompilerResult::single_err(None, err);
            }
        };

        match token.data {
            Token::PlayerEvent | Token::EntityEvent => {
                let def = self.event(token);
            }
            Token::ProcDef => todo!(),
            Token::FuncDef => todo!(),
            it => {
                return CompilerResult::new(
                    None,
                    vec![CompilerError::Unexpected {
                        expected: expected,
                        received: it.spanned(token.span),
                        message: Some("Top level statements must be declerations".to_string()),
                    }],
                )
            }
        }

        todo!()
    }

    fn event(&mut self, definition: Spanned<Token<'src>>) -> CompilerResult<'src, TopLevel<'src>> {
        let type_tok = match definition.data {
            Token::PlayerEvent => EventType::Player,
            Token::EntityEvent => EventType::Entity,
            it => panic!(
                "Expected PlayerEvent or EntityEvent token, recieved {:?}",
                it
            ),
        };

        let name = self.next_expect(
            &Token::Iden("").into(),
            Some("Expected event name after decleration"),
        );
        let name = match name {
            Ok(it) => {
                let span = it.span;
                let data = match it.data {
                    Token::Iden(it) => it,
                    it => panic!("Expected Iden recieved {:?}", it),
                };
                Spanned::new(Iden::new(data), span)
            }
            Err(err) => return CompilerResult::single_err(None, err),
        };

        
        // TODO Continue writing here
        // Notes: Make sure to have multiple errors at a time possible
        let statements = self.statements();


        let event = TopLevel::Event(Event::new(
            Spanned::new(type_tok, definition.span),
            name,
            todo!(),
            todo!(),
        ));

        todo!()
    }

}
