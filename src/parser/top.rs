use crate::ast::{
    statement::{ActionType, IfStatement, SimpleStatement, Statement},
    top::{Event, EventType, FuncDef, ProcDef},
    Iden,
};

use super::error::*;
use super::stmt::*;
use super::*;

impl<'src> Parser<'src> {
    pub(super) fn top_level(&mut self) -> CompilerResult<'src, TopLevel<'src>> {
        let expected = ExpectedTokens::new(vec![
            Token::FuncDef,
            Token::ProcDef,
            Token::PlayerEvent,
            Token::EntityEvent,
        ]);

        let token = match self.peek_expect(&expected, None) {
            Ok(it) => it,
            Err(err) => {
                return CompilerResult::single_err(None, err);
            }
        };

        match token.data {
            Token::PlayerEvent | Token::EntityEvent => {
                let def = self.event();
            }
            Token::ProcDef => {
                let def = self.process();
            }
            Token::FuncDef => {}
            it => {
                return CompilerResult::new(
                    None,
                    vec![CompilerError::Unexpected {
                        expected,
                        received: it.spanned(token.span),
                        message: Some("Top level statements must be declarations".to_string()),
                    }],
                )
            }
        }

        todo!()
    }

    fn process(&mut self) -> CompilerResult<'src, ProcDef<'src>> {
        todo!()
    }

    fn function(&mut self) -> CompilerResult<'src, FuncDef<'src>> {
        todo!()
    }

    fn event(&mut self) -> CompilerResult<'src, Event<'src>> {
        let next = self.next_expect(&ExpectedTokens::new(vec![Token::PlayerAction, Token::EntityAction]), "");
        let definition = match next {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        };

        let type_tok = match definition.data {
            Token::PlayerEvent => EventType::Player,
            Token::EntityEvent => EventType::Entity,
            it => panic!(
                "Expected PlayerEvent or EntityEvent token, received {:?}",
                it
            ),
        };

        let name = self.next_expect(
            &Token::Iden("").into(),
            Some("Expected event name after declaration"),
        );
        let name = match name {
            Ok(it) => {
                let span = it.span;
                let data = match it.data {
                    Token::Iden(it) => it,
                    it => panic!("Expected Iden received {:?}", it),
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
