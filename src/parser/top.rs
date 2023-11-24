use logos::Span;

use crate::ast::{
    recovery::{Recovery, StatementRecovery, TopLevelRecovery},
    statement::{ActionType, IfStatement, SimpleStatement, Statement, Statements},
    top::{Event, EventType, FuncDef, ProcDef},
    Iden,
};

use super::error::*;
use super::stmt::*;
use super::*;

impl<'src> Parser<'src> {
    /// It is guaranteed that the next token will be a top level declaration token
    pub(super) fn top_level(&mut self) -> CompilerResult<'src, TopLevel<'src>> {
        let token = match self.peek_expect(
            &ExpectedTokens::new(Token::TOP_LEVEL.to_vec()),
            Some("top level decleration token"),
        ) {
            Ok(it) => Spanned::new(it.data.to_owned(), it.span),
            Err(err) => {
                let CompilerResult { data, mut error } = self.top_recovery(Vec::new());
                error.push(err);
                return CompilerResult::new(TopLevel::Recovery(data), error);
            }
        };

        let top = match &token.data {
            Token::PlayerEvent | Token::EntityEvent => {
                let CompilerResult { data, error } = self.event();
                let data = match data {
                    Ok(it) => TopLevel::Event(it),
                    Err(err) => TopLevel::Recovery(err),
                };
                CompilerResult::new(data, error)
            }
            Token::ProcDef => {
                let def = self.process();
                todo!()
            }
            Token::FuncDef => {
                let def = self.function();
                todo!()
            }
            it => self
                .top_recovery(Vec::new())
                .map_inner(|i| TopLevel::Recovery(i)),
        };

        top
    }

    fn process(&mut self) -> CompilerResult<'src, ProcDef<'src>> {
        todo!()
    }

    fn function(&mut self) -> CompilerResult<'src, FuncDef<'src>> {
        todo!()
    }

    /// Represents an event delceration
    /// `pevent Join (statements) end`
    /// If the compiler result data is None, then it can be treated as malformed
    fn event(&mut self) -> CompilerResult<'src, Result<Event<'src>, TopLevelRecovery<'src>>> {
        let definition = self.next_assert(
            &ExpectedTokens::new(vec![Token::PlayerEvent, Token::PlayerEvent]),
            Some("event token"),
        );

        let type_tok = match definition.data {
            Token::PlayerEvent => EventType::Player,
            Token::EntityEvent => EventType::Entity,
            it => panic!(
                "Expected PlayerEvent or EntityEvent token, received {:?}",
                it
            ),
        };

        let name = match self.next_expect(&Token::Iden("").into(), Some("event name")) {
            Ok(it) => {
                let span = it.span;
                let data = match it.data {
                    Token::Iden(it) => it,
                    it => panic!("Expected Iden received {:?}", it),
                };
                Spanned::new(Iden::new(data), span)
            }
            Err(err) => {
                let CompilerResult { data, mut error } = self.top_recovery(vec![definition]);
                error.push(err);
                return CompilerResult::new(Err(data), error);
            }
        };

        let CompilerResult {
            data: stmts,
            error: mut errors,
        } = self.statements();

        let end = match self.next_expect(&Token::End.into(), None) {
            Ok(it) => it.to_empty(),
            Err(err) => {
                errors.push(err);
                // TODO HERE
                return CompilerResult::new(Err(), errors);
            }
        };

        let range = {
            match stmts.first() {
                Some(first) => {
                    let last = stmts.last().expect("If the first exists, the last exists");

                    last.span.start..last.span.end
                }
                None => {
                    // Get a bit desperate here
                    name.span.end..end.span.start
                }
            }
        };

        let event = Event::new(
            Spanned::new(type_tok, definition.span),
            name,
            Spanned::new(Statements::new(stmts), range),
            end,
        );

        CompilerResult::new(Ok(event), errors)
    }

    /// Looks for event, proc, func tokens
    /// This function will never error
    fn top_recovery(
        &mut self,
        mut tokens: Vec<Spanned<Token<'src>>>,
    ) -> CompilerResult<'src, TopLevelRecovery<'src>> {
        let mut errors = Vec::new();
        loop {
            match self.peek() {
                Ok(tok) => match tok.data {
                    Token::PlayerEvent | Token::EntityEvent | Token::FuncDef | Token::ProcDef => {
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
                            return CompilerResult::single_err(TopLevelRecovery::new(tokens), err);
                        }
                        CompilerError::LexerError(span) => {
                            tokens.push(Token::Invalid.spanned(span.span.clone()));
                        }
                    }
                    errors.push(err);
                }
            };
        }
        CompilerResult::new(TopLevelRecovery::new(tokens), errors)
    }

    fn john(
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
                            errors.push(err);
                            return CompilerResult::new(StatementRecovery::new(tokens), errors);
                        }
                        CompilerError::LexerError(span) => {
                            tokens.push(Token::Invalid.spanned(span.span.clone()));
                        }
                    }
                    errors.push(err);
                }
            };
        }
        CompilerResult::new(StatementRecovery::new(tokens), errors)
    }
}

