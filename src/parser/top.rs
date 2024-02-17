use crate::tree::{
    recovery::TopLevelRecovery,
    statement::Statements,
    top::{Event, EventType, FuncDef, ProcDef},
    Iden,
};


use super::*;

impl<'lex> Parser<'lex> {
    /// It is guaranteed that the next token will be a top level declaration token
    pub(super) fn top_level(&mut self) -> ParseResult<TopLevel> {
        // Find first token
        let token = match self.peek_expect(&Token::TOP_LEVEL, Some("top level decleration token")) {
            Ok(it) => it,
            Err(err) => {
                return match err {
                    AdvanceUnexpected::Token(err) => ParseResult::new(
                        TopLevel::Recovery(TopLevelRecovery),
                        vec![err],
                        self.top_recovery(),
                    ),
                    AdvanceUnexpected::Eof(err) => ParseResult::new(
                        TopLevel::Recovery(TopLevelRecovery),
                        Vec::new(),
                        Some(Box::new(err)),
                    ),
                }
            }
        };

        let top = match &token.data {
            Token::PlayerEvent | Token::EntityEvent => {
                let ParseResult {
                    data,
                    error,
                    at_eof,
                } = self.event();
                let data = match data {
                    Ok(it) => TopLevel::Event(it),
                    Err(err) => TopLevel::Recovery(err),
                };
                ParseResult::new(data, error, at_eof)
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
                    "Filter {:#?} should have caught {:#?}",
                    Token::TOP_LEVEL,
                    it
                );
            }
        };

        top
    }

    fn process(&mut self) -> ParseResult<ProcDef, UnexpectedToken> {
        todo!()
    }

    fn function(&mut self) -> ParseResult<FuncDef, UnexpectedToken> {
        todo!()
    }

    /// Represents an event delceration
    /// `pevent Join (statements) end`
    /// If the compiler result data is None, then it can be treated as malformed
    fn event(&mut self) -> ParseResult<Result<Event, TopLevelRecovery>, Vec<UnexpectedToken>> {
        let definition = self.next_assert(&Token::EVENT);

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
                        ParseResult::new(Err(TopLevelRecovery), vec![err], self.top_recovery())
                    }
                    AdvanceUnexpected::Eof(err) => {
                        ParseResult::new(Err(TopLevelRecovery), Vec::new(), Some(Box::new(err)))
                    }
                }
            }
        };

        let ParseResult {
            data: stmts,
            error: errors,
            at_eof,
        } = self.statements();

        if let Some(at_eof) = at_eof {
            return ParseResult::new(Err(TopLevelRecovery), errors, Some(at_eof));
        }

        let end = match self.next_expect(&[Token::End], None) {
            Ok(it) => it.to_empty(),
            Err(err) => {
                return match err {
                    AdvanceUnexpected::Token(err) => {
                        ParseResult::new(Err(TopLevelRecovery), vec![err], self.top_recovery())
                    }
                    AdvanceUnexpected::Eof(err) => {
                        ParseResult::new(Err(TopLevelRecovery), errors, Some(Box::new(err)))
                    }
                };
            }
        };

        let event = Event::new(
            Spanned::new(type_tok, definition.span),
            name.map_inner(|it| Iden::new(it.get_iden_inner())),
            Statements::new(stmts),
            end,
        );

        ParseResult::new(Ok(event), errors, None)
    }

    /// Looks for event, proc, func tokens
    /// This function will never syntax error
    fn top_recovery(&mut self) -> Option<Box<UnexpectedEOF>> {
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
