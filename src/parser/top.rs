use crate::ast::{
    recovery::TopLevelRecovery,
    statement::Statements,
    top::{Event, EventType, FuncDef, ProcDef},
    Iden,
};

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
                return match err {
                    AdvanceUnexpected::Token(err) => {
                        let at_eof = self.top_recovery();
                        CompilerResult::new(TopLevel::Recovery(TopLevelRecovery), vec![err], at_eof)
                    }
                    AdvanceUnexpected::Eof(err) => CompilerResult::new(
                        TopLevel::Recovery(TopLevelRecovery),
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
                let _def = self.process();
                todo!()
            }
            Token::FuncDef => {
                let _def = self.function();
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
    ) -> CompilerResult<'src, Result<Event<'src>, TopLevelRecovery>, Vec<UnexpectedToken<'src>>>
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
                        let at_eof = self.top_recovery();
                        let recovery = TopLevelRecovery;
                        CompilerResult::new(Err(recovery), vec![err], at_eof)
                    }
                    AdvanceUnexpected::Eof(err) => {
                        let recovery = TopLevelRecovery;
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
            return CompilerResult::new(Err(TopLevelRecovery), errors, Some(at_eof));
        }

        let end = match self.next_expect(&[Token::End], None) {
            Ok(it) => it.to_empty(),
            Err(err) => {
                // Recovery tokens
                return match err {
                    AdvanceUnexpected::Token(err) => {
                        let at_eof = self.top_recovery();
                        errors.push(err);
                        let recovery = TopLevelRecovery;
                        CompilerResult::new(Err(recovery), errors, at_eof)
                    }
                    AdvanceUnexpected::Eof(err) => {
                        CompilerResult::new(Err(TopLevelRecovery), errors, Some(Box::new(err)))
                    }
                };
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
        let stmts = Statements::new(stmts);

        let event = Event::new(Spanned::new(type_tok, definition.span), iden, stmts, end);

        CompilerResult::new(Ok(event), errors, None)
    }

    /// Looks for event, proc, func tokens
    /// This function will never syntax error
    fn top_recovery(&mut self) -> Option<Box<UnexpectedEOF<'src>>> {
        loop {
            match self.peek() {
                Ok(tok) => match tok.data {
                    Token::PlayerEvent | Token::EntityEvent | Token::FuncDef | Token::ProcDef => {
                        break;
                    }

                    _ => match self.advance() {
                        Ok(_it) => {}
                        Err(_err) => {
                            panic!("Unexpected EOF should have been caught before");
                        }
                    },
                },
                Err(err) => {
                    return Some(Box::new(err));
                }
            };
        }
        None
    }
}
