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

impl<'src> Parser<'src> {
    /// It is guaranteed that the next token will be a top level declaration token
    pub(super) fn top_level(
        &mut self,
    ) -> CompilerResult<'src, TopLevel<'src>, Vec<UnexpectedToken<'src>>> {
        // Find first token
        let token = match self.peek_expect(&Token::TOP_LEVEL, Some("top level decleration token")) {
            Ok(it) => Spanned::new(it.data.to_owned(), it.span),
            Err(err) => {
                let CompilerResult {
                    data,
                    error,
                    at_eof,
                } = self.top_recovery();
                return CompilerResult::new_with_eof(
                    TopLevel::Recovery(TopLevelRecovery::new(vec![
                        TopRecoveryType::Unrecognizable(data),
                    ])),
                    Vec::new(),
                    at_eof,
                );
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
                CompilerResult::new_with_eof(data, error, at_eof)
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
                // Recover
                let CompilerResult {
                    data,
                    error,
                    at_eof,
                } = self.top_recovery();
                let data = TopLevel::Recovery(TopLevelRecovery::new(vec![
                    TopRecoveryType::Unrecognizable(data),
                ]));

                CompilerResult::new_with_eof(data, Vec::new(), at_eof)
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

        let name = match self.next_expect(&[Token::Iden("")], Some("event name")) {
            Ok(it) => it,
            Err(err) => {
                let CompilerResult {
                    data,
                    error,
                    at_eof,
                } = self.top_recovery();
                let mut def =
                    TopLevelRecovery::new(vec![TopRecoveryType::Unrecognizable(vec![definition])]);
                def.items.push(TopRecoveryType::Unrecognizable(data));
                return CompilerResult::new_with_eof(Err(def), Vec::new(), at_eof);
            }
        };

        let CompilerResult {
            data: stmts,
            error: mut errors,
            at_eof,
        } = self.statements();

        let end = match self.next_expect(&[Token::End], None) {
            Ok(it) => it.to_empty(),
            Err(err) => {
                // Recovery tokens
                let mut toks = Vec::new();
                toks.push(definition);
                toks.push(name);
                return match err {
                    AdvanceUnexpected::Token(err) => {
                        let CompilerResult {
                            mut data,
                            error,
                            at_eof,
                        } = self.top_recovery();
                        toks.append(&mut data);

                        errors.push(err);
                        let recovery =
                            TopLevelRecovery::new(vec![TopRecoveryType::Unrecognizable(toks)]);
                        CompilerResult::new_with_eof(Err(recovery), errors, at_eof)
                    }
                    AdvanceUnexpected::Eof(err) => CompilerResult::new_with_eof(
                        Err(TopLevelRecovery::new(vec![TopRecoveryType::Body(stmts)])),
                        errors,
                        Some(Box::new(err))
                    ),
                };
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
                    name.span.start..end.span.end
                }
            }
        };

        let iden = {
            let data = match name.data {
                Token::Iden(it) => it,
                it => panic!("Expected Iden received {:?}", it),
            };
            Spanned::new(Iden::new(data), name.span)
        };

        let event = Event::new(
            Spanned::new(type_tok, definition.span),
            iden,
            Spanned::new(Statements::new(stmts), range),
            end,
        );

        CompilerResult::new(Ok(event), errors)
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

                    _ => match self.next() {
                        Ok(it) => tokens.push(it),
                        Err(err) => {
                            panic!("Unexpected EOF should have been caught before");
                        }
                    },
                },
                Err(err) => {
                    return CompilerResult::new_with_eof(tokens, (), Some(Box::new(err)));
                }
            };
        }
        CompilerResult::new(tokens, ())
    }
}
