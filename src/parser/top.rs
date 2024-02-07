use logos::Span;

use crate::ast::{
    recovery::{Recovery, StatementRecovery, TopLevelRecovery, TopRecoveryType},
    statement::{ActionType, IfStatement, SimpleStatement, Statement, Statements},
    top::{Event, EventType, FuncDef, ProcDef},
    Iden,
};

use super::error::*;
use super::stmt::*;
use super::*;

fn isolated() {
}

impl<'src> Parser<'src> {
    /// It is guaranteed that the next token will be a top level declaration token
    pub(super) fn top_level(
        &mut self,
    ) -> CompilerResult<'src, TopLevel<'src>, Vec<UnexpectedToken<'src>>> {
        // Find first token
        let token = match self.peek_expect(&Token::TOP_LEVEL, Some("top level decleration token")) {
            Ok(it) => Spanned::new(it.data.to_owned(), it.span),
            Err(err) => {
                return match err {
                    AdvanceUnexpected::Token(err) => {
                        let CompilerResult {
                            data,
                            error: _,
                            at_eof,
                        } = self.top_recovery();
                        CompilerResult::new(
                            TopLevel::Recovery(TopLevelRecovery::new(vec![
                                TopRecoveryType::Unrecognizable(data),
                            ])),
                            vec![err],
                            at_eof,
                        )
                    }
                    AdvanceUnexpected::Eof(err) => CompilerResult::new(
                        TopLevel::Recovery(TopLevelRecovery::new(Vec::new())),
                        Vec::new(),
                        Some(Box::new(err)),
                    ),
                }
            }
        };

        let top = match &token.data {
            Token::PlayerEvent | Token::EntityEvent => {
                let CompilerResult {
                    data,
                    error,
                    at_eof,
                } = self.event();
                let data = match data {
                    Ok(it) => TopLevel::Event(it),
                    Err(err) => TopLevel::Recovery(err),
                };
                CompilerResult::new(data, error, at_eof)
            }
            Token::ProcDef => {
                let def = self.process();
                todo!()
            }
            Token::FuncDef => {
                let def = self.function();
                todo!()
            }
            it => {
                panic!(
                    "Somehow: filter {:#?} did not include {:#?}",
                    Token::TOP_LEVEL,
                    it
                );
            }
        };

        top
    }

    fn process(&mut self) -> CompilerResult<'src, ProcDef<'src>, UnexpectedToken<'src>> {
        todo!()
    }

    fn function(&mut self) -> CompilerResult<'src, FuncDef<'src>, UnexpectedToken<'src>> {
        todo!()
    }

    /// Represents an event delceration
    /// `pevent Join (statements) end`
    /// If the compiler result data is None, then it can be treated as malformed
    fn event(
        &mut self,
    ) -> CompilerResult<'src, Result<Event<'src>, TopLevelRecovery<'src>>, Vec<UnexpectedToken<'src>>>
    {
        let definition = self.next_assert(&Token::EVENT, Some("event token"));

        let type_tok = match definition.data {
            Token::PlayerEvent => EventType::Player,
            Token::EntityEvent => EventType::Entity,
            it => panic!(
                "Expected PlayerEvent or EntityEvent token, received {:?}",
                it
            ),
        };

        let name = match self.next_expect(&[Token::Iden(None)], Some("event name")) {
            Ok(it) => it,
            Err(err) => {
                return match err {
                    AdvanceUnexpected::Token(err) => {
                        let CompilerResult {
                            data,
                            error: _,
                            at_eof,
                        } = self.top_recovery();
                        let recovery = TopLevelRecovery::new(vec![
                            TopRecoveryType::Unrecognizable(vec![definition]),
                            TopRecoveryType::Unrecognizable(data),
                        ]);
                        CompilerResult::new(Err(recovery), vec![err], at_eof)
                    }
                    AdvanceUnexpected::Eof(err) => {
                        let recovery = TopLevelRecovery::new(vec![
                            TopRecoveryType::Unrecognizable(vec![definition]),
                        ]);
                        CompilerResult::new(Err(recovery), Vec::new(), Some(Box::new(err)))
                    }
                }
            }
        };

        let CompilerResult {
            data: stmts,
            error: mut errors,
            at_eof,
        } = self.statements();

        if let Some(at_eof) = at_eof {
            return CompilerResult::new(
                Err(TopLevelRecovery::new(vec![TopRecoveryType::Body(stmts)])),
                errors,
                Some(at_eof),
            );
        }

        let end = match self.next_expect(&[Token::End], None) {
            Ok(it) => it.to_empty(),
            Err(err) => {
                // Recovery tokens
                let toks = TopRecoveryType::Unrecognizable(vec![definition, name]);
                let body = TopRecoveryType::Body(stmts);
                return match err {
                    AdvanceUnexpected::Token(err) => {
                        let CompilerResult {
                            data,
                            error: _,
                            at_eof,
                        } = self.top_recovery();
                        errors.push(err);
                        let recovery = TopLevelRecovery::new(vec![
                            toks,
                            body,
                            TopRecoveryType::Unrecognizable(data),
                        ]);
                        CompilerResult::new(Err(recovery), errors, at_eof)
                    }
                    AdvanceUnexpected::Eof(err) => CompilerResult::new(
                        Err(TopLevelRecovery::new(vec![toks, body])),
                        errors,
                        Some(Box::new(err)),
                    ),
                };
            }
        };

        let range = {
            match stmts.first() {
                Some(first) => {
                    let last = stmts.last().expect("If the first exists, the last exists");

                    first.span.start..last.span.end
                }
                None => {
                    // Get a bit desperate here
                    name.span.start..end.span.end
                }
            }
        };

        let iden = {
            let data = match name.data {
                Token::Iden(it) => it,
                it => panic!("Expected Iden received {:?}", it),
            };
            Spanned::new(
                Iden::new(data.expect("Iden should only be None when finding")),
                name.span,
            )
        };

        let event = Event::new(
            Spanned::new(type_tok, definition.span),
            iden,
            Spanned::new(Statements::new(stmts), range),
            end,
        );

        CompilerResult::new(Ok(event), errors, None)
    }

    /// Looks for event, proc, func tokens
    /// This function will never syntax error
    fn top_recovery(&mut self) -> CompilerResult<'src, Vec<Spanned<Token<'src>>>, ()> {
        let mut tokens = Vec::new();
        loop {
            match self.peek() {
                Ok(tok) => match tok.data {
                    Token::PlayerEvent | Token::EntityEvent | Token::FuncDef | Token::ProcDef => {
                        break;
                    }

                    _ => match self.advance() {
                        Ok(it) => tokens.push(it),
                        Err(_err) => {
                            panic!("Unexpected EOF should have been caught before");
                        }
                    },
                },
                Err(err) => {
                    return CompilerResult::new(tokens, (), Some(Box::new(err)));
                }
            };
        }
        CompilerResult::new(tokens, (), None)
    }
}
