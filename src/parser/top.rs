use crate::{
    common::prelude::*,
    tree::{
        recovery::TopLevelRecovery,
        statement::TreeStatements,
        top::{TreeEvent, TreeFuncDef, TreeFuncParamDef, TreeProcDef},
    },
};

use self::ext::{adv_top, should_return_top};

use super::*;

impl<'lex> Parser<'lex> {
    /// It is guaranteed that the next token will be a top level declaration token
    pub(super) fn top_level(&mut self) -> ParseResult<TreeTopLevel> {
        // Find first token
        let token = match self.peek_expect(&Token::TOP_LEVEL, Some("top level decleration token")) {
            Ok(it) => it,
            Err(err) => {
                return match err {
                    AdvanceUnexpected::Token(err) => ParseResult::new(
                        TreeTopLevel::Recovery(TopLevelRecovery),
                        vec![err],
                        self.top_recovery(),
                    ),
                    AdvanceUnexpected::Eof(err) => ParseResult::new(
                        TreeTopLevel::Recovery(TopLevelRecovery),
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
                    Ok(it) => TreeTopLevel::Event(it),
                    Err(err) => TreeTopLevel::Recovery(err),
                };
                ParseResult::new(data, error, at_eof)
            }
            Token::ProcDef => {
                let ParseResult {
                    data,
                    error,
                    at_eof,
                } = self.process();
                let data = match data {
                    Ok(it) => TreeTopLevel::ProcDef(it),
                    Err(err) => TreeTopLevel::Recovery(err),
                };
                ParseResult::new(data, error, at_eof)
            }
            Token::FuncDef => {
                let ParseResult {
                    data,
                    error,
                    at_eof,
                } = self.function();
                let data = match data {
                    Ok(it) => TreeTopLevel::FuncDef(it),
                    Err(err) => TreeTopLevel::Recovery(err),
                };
                ParseResult::new(data, error, at_eof)
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

    fn process(&mut self) -> ParseResult<Result<TreeProcDef, TopLevelRecovery>> {
        let type_tok = self.next_assert(&[Token::ProcDef]).to_empty();
        let name = adv_top!(self, self.next_expect(&[Token::Iden(None)], None))
            .map_inner(|it| Iden::new(it.get_iden_inner()));

        let ParseResult {
            data,
            error,
            at_eof,
        } = self.statements(false);
        if at_eof.is_some() {
            return ParseResult::new(Err(TopLevelRecovery), error, at_eof);
        }

        let end = adv_top!(self, self.next_expect(&[Token::End], None)).to_empty();

        ParseResult::new(
            Ok(TreeProcDef {
                type_tok,
                name,
                statements: TreeStatements::new(data),
                end_tok: end,
            }),
            error,
            None,
        )
    }

    fn function(&mut self) -> ParseResult<Result<TreeFuncDef, TopLevelRecovery>> {
        let type_tok = self.next_assert(&[Token::FuncDef]).to_empty();
        let name = adv_top!(self, self.next_expect(&[Token::Iden(None)], None))
            .map_inner(|it| Iden::new(it.get_iden_inner()));

        let params = {
            let open = adv_top!(self, self.next_expect(&[Token::OpenParen], None)).to_empty();
            let mut params = Vec::new();
            while {
                let next = adv_top!(
                    self,
                    self.peek_expect(
                        &[Token::CloseParen, Token::Iden(None)],
                        Some("Next param or close")
                    )
                );
                match next.data {
                    Token::CloseParen => false,
                    _ => true,
                }
            } {
                let param = should_return_top!(self.params());
                params.push(param);

                let comma = adv_top!(
                    self,
                    self.peek_expect(
                        &[Token::Comma, Token::CloseParen],
                        Some("Comma or end of paramaters")
                    )
                );
                match comma.data {
                    Token::Comma => {
                        let _comma = self.next_assert(&[Token::Comma]);
                    }
                    Token::CloseParen => {}
                    _ => panic!("should be covered by next expect"),
                }
            }
            let close = adv_top!(self, self.next_expect(&[Token::CloseParen], None)).to_empty();

            Wrapped::new(
                open,
                Parameters::new(params).try_calculate_span_wrap(),
                close,
            )
        };

        let ParseResult {
            data,
            error,
            at_eof,
        } = self.statements(false);
        if at_eof.is_some() {
            return ParseResult::new(Err(TopLevelRecovery), error, at_eof);
        }

        let end = adv_top!(self, self.next_expect(&[Token::End], None)).to_empty();

        ParseResult::new(
            Ok(TreeFuncDef {
                type_tok,
                name,
                params,
                statements: TreeStatements::new(data),
                end_tok: end,
            }),
            error,
            None,
        )
    }

    fn params(&mut self) -> ParseResult<Result<TreeFuncParamDef, TopLevelRecovery>> {
        let name = self
            .next_assert(&[Token::Iden(None)])
            .map_inner(|it| Iden::new(it.get_iden_inner()));
        let colon = adv_top!(self, self.next_expect(&[Token::Colon], None)).to_empty();
        let data_type = adv_top!(self, self.next_expect(&[Token::Iden(None)], None))
            .map_inner(|it| Iden::new(it.get_iden_inner()));
        let description = adv_top!(
            self,
            self.peek_expect(&[Token::Iden(None), Token::CloseParen], None)
        );
        let description = match description.data {
            Token::Iden(_) => Some(
                self.next_assert(&[Token::Iden(None)])
                    .map_inner(|it| Iden::new(it.get_iden_inner())),
            ),
            Token::CloseParen => None,
            _ => panic!("should be covered by peek expect"),
        };

        // .map_inner(|it| Iden::new(it.get_iden_inner()));
        ParseResult::ok(Ok(TreeFuncParamDef {
            name,
            colon,
            data_type,
            description,
        }))
    }

    /// Represents an event delceration
    /// `pevent Join (statements) end`
    /// If the compiler result data is None, then it can be treated as malformed
    fn event(&mut self) -> ParseResult<Result<TreeEvent, TopLevelRecovery>, Vec<UnexpectedToken>> {
        let definition = self.next_assert(&Token::EVENT);

        let type_tok = match definition.data {
            Token::PlayerEvent => EventType::Player,
            Token::EntityEvent => EventType::Entity,
            it => panic!(
                "Expected PlayerEvent or EntityEvent token, received {:?}",
                it
            ),
        };

        let name = adv_top!(
            self,
            self.next_expect(&[Token::Iden(None)], Some("event name"))
        );

        let ParseResult {
            data: stmts,
            error: errors,
            at_eof,
        } = self.statements(false);

        if let Some(at_eof) = at_eof {
            return ParseResult::new(Err(TopLevelRecovery), errors, Some(at_eof));
        }

        let end = adv_top!(self, self.next_expect(&[Token::End], None)).to_empty();

        let event = TreeEvent::new(
            Spanned::new(type_tok, definition.span),
            name.map_inner(|it| Iden::new(it.get_iden_inner())),
            TreeStatements::new(stmts),
            end,
        );

        ParseResult::new(Ok(event), errors, None)
    }

    /// Looks for event, proc, func tokens
    /// This function will never syntax error
    pub fn top_recovery(&mut self) -> Option<Box<UnexpectedEOF>> {
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
