use super::*;
use crate::ast::recovery::StatementRecovery;

use crate::ast::statement::IdenPair;
use crate::ast::statement::Selection;
use crate::ast::statement::Tags;
use crate::ast::CalcSpan;
use crate::ast::MaybeSpan;
use crate::ast::Parameters;
use crate::ast::TryCalcSpan;
use crate::{ast::Spanned, lexer::Token};

impl<'src> Parser<'src> {
    /// Must start with a `[`
    pub fn tags(&mut self) -> ParseResult<'src, Result<Tags<'src>, StatementRecovery>> {
        let open = self.next_assert(&[Token::OpenBracket], None);

        let ParseResult {
            data,
            error,
            at_eof,
        } = self.pair_list();
        let params = match data {
            Ok(it) => {
                if at_eof.is_some() {
                    return ParseResult::new(Err(StatementRecovery), error, at_eof);
                }
                it
            }
            Err(err) => {
                return ParseResult::new(Err(err), error, at_eof);
            }
        };

        let close = match self.next_expect(&[Token::CloseBracket], None) {
            Ok(it) => it,
            Err(err) => {
                return helper::recover_statement(self, err);
            }
        };

        let tag_span = params.try_calculate_span();
        ParseResult::ok(Ok(Tags {
            open: open.to_empty(),
            tags: Some(MaybeSpan::new(params, tag_span)),
            close: close.to_empty(),
        }))
    }

    /// Must start with an iden
    pub fn pair_list(
        &mut self,
    ) -> ParseResult<'src, Result<Parameters<IdenPair<'src>>, StatementRecovery>> {
        let mut pairs = Vec::new();

        let next = match self.peek_expect(&[Token::CloseBracket, Token::Iden(None)], None) {
            Ok(it) => it,
            Err(err) => return helper::recover_statement(self, err),
        };

        match next.data {
            Token::CloseBracket => {
                return ParseResult::new(Ok(Parameters::new(pairs)), Vec::new(), None)
            }
            Token::Iden(_) => {}
            _ => panic!("Should have been caught by peek_expect"),
        }

        loop {
            let ParseResult {
                data,
                error,
                at_eof,
            } = self.iden_pair();
            let pair = match data {
                Ok(it) => {
                    if at_eof.is_some() {
                        return ParseResult::new(Err(StatementRecovery), error, at_eof);
                    }
                    it
                }
                Err(err) => {
                    return ParseResult::new(Err(err), error, at_eof);
                }
            };

            let span = pair.calculate_span();
            pairs.push(Spanned::new(pair, span));

            let tok = match self.peek_expect(&[Token::CloseBracket, Token::Comma], None) {
                Ok(it) => it,
                // TODO I had it with recovery tokens
                Err(err) => return helper::recover_statement(self, err),
            };

            match tok.data {
                Token::CloseBracket => break,
                Token::Comma => {
                    self.next_assert(&[Token::Comma], None);

                    let tok =
                        match self.peek_expect(&[Token::CloseBracket, Token::Iden(None)], None) {
                            Ok(it) => it,
                            Err(err) => return helper::recover_statement(self, err),
                        };

                    match tok.data {
                        Token::CloseBracket => {
                            break;
                        }
                        Token::Iden(_) => {}
                        _ => panic!("Should be covered by next expect"),
                    }
                }
                _ => panic!("Should be covered by next expect"),
            };
        }

        ParseResult::ok(Ok(Parameters::new(pairs)))
    }

    /// Must start with an iden
    pub fn iden_pair(&mut self) -> ParseResult<'src, Result<IdenPair<'src>, StatementRecovery>> {
        let key = self
            .next_assert(&[Token::Iden(None)], None)
            .map_inner(|it| it.get_iden_inner());

        let colon = match self.next_expect(&[Token::Colon], None) {
            Ok(it) => it.to_empty(),
            Err(err) => return helper::recover_statement(self, err),
        };

        let value = match self.next_expect(&[Token::Iden(None)], None) {
            Ok(it) => it.map_inner(|it| it.get_iden_inner()),
            Err(err) => return helper::recover_statement(self, err),
        };

        ParseResult::ok(Ok(IdenPair { key, colon, value }))
    }

    /// Must start with a `<`
    pub fn selector(&mut self) -> ParseResult<'src, Result<Selection<'src>, StatementRecovery>> {
        let open = self.next_assert(&[Token::OpenComp], None);

        let selection = match self.next_expect(&[Token::Iden(None)], None) {
            Ok(it) => it,
            Err(err) => {
                return match err {
                    AdvanceUnexpected::Token(it) => {
                        match it.received.data {
                            // This allows <>
                            Token::CloseComp => ParseResult::ok(Ok(Selection {
                                open: open.to_empty(),
                                selection: None,
                                close: Spanned::<()>::empty(it.received.span),
                            })),
                            _ => ParseResult::new(
                                Err(StatementRecovery),
                                Vec::new(),
                                self.statement_recovery(),
                            ),
                        }
                    }
                    AdvanceUnexpected::Eof(err) => {
                        ParseResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)))
                    }
                };
            }
        };

        let close = match self.next_expect(&[Token::CloseComp], None) {
            Ok(it) => it,
            Err(err) => return helper::recover_statement(self, err),
        };

        ParseResult::ok(Ok(Selection {
            open: open.to_empty(),
            selection: Some(selection.map_inner(|it| it.get_iden_inner())),
            close: close.to_empty(),
        }))
    }

}
