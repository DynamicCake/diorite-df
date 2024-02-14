use super::*;
use crate::ast::recovery::StatementRecovery;

use crate::ast::statement::ExprLitType;
use crate::ast::statement::ExprLiteral;
use crate::ast::statement::Expression;
use crate::ast::statement::IdenPair;
use crate::ast::statement::Selection;
use crate::ast::statement::StaticLiteral;
use crate::ast::statement::Tags;
use crate::ast::statement::Wrapped;
use crate::ast::CalcThenWrap;
use crate::ast::MaybeSpan;
use crate::ast::NumberLiteral;
use crate::ast::Parameters;
use crate::ast::StringLiteral;
use crate::ast::TryCalcSpan;
use crate::ast::TryCalcThenWrap;
use crate::{ast::Spanned, lexer::Token};

impl Parser<'_> {
    /// Must start with a `[`
    pub fn tags(&mut self) -> ParseResult<Result<Tags, StatementRecovery>> {
        let open = self.next_assert(&[Token::OpenBracket]);

        let params = match helper::should_return(self.pair_list()) {
            Ok(it) => it,
            Err(err) => return err,
        };

        let close = match self.next_expect(&[Token::CloseBracket], None) {
            Ok(it) => it,
            Err(err) => {
                return helper::recover_statement(self, err);
            }
        };

        ParseResult::ok(Ok(Tags {
            open: open.to_empty(),
            tags: params.try_calculate_span_wrap(),
            close: close.to_empty(),
        }))
    }

    /// Must start with an iden
    pub fn pair_list(&mut self) -> ParseResult<Result<Parameters<IdenPair>, StatementRecovery>> {
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
            let pair = match helper::should_return(self.iden_pair()) {
                Ok(it) => it,
                Err(err) => return err,
            };

            pairs.push(pair);

            let tok = match self.peek_expect(&[Token::CloseBracket, Token::Comma], None) {
                Ok(it) => it,
                Err(err) => return helper::recover_statement(self, err),
            };

            match tok.data {
                Token::CloseBracket => break,
                Token::Comma => {
                    self.next_assert(&[Token::Comma]);

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
    pub fn iden_pair(&mut self) -> ParseResult<Result<IdenPair, StatementRecovery>> {
        let key = self
            .next_assert(&[Token::Iden(None)])
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
    pub fn selector(&mut self) -> ParseResult<Result<Selection, StatementRecovery>> {
        let open = self.next_assert(&[Token::OpenComp]);

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

    // Must start with '('
    pub fn call_params(&mut self) -> ParseResult<Result<Wrapped<Expression>, StatementRecovery>> {
        let open = self.next_assert(&[Token::OpenParen]);

        let mut items: Vec<Expression> = Vec::new();

        let next = match self.peek_expect(
            &[
                Token::CloseParen,
                Token::String(None),
                Token::Number(None),
                Token::Iden(None),
            ],
            None,
        ) {
            Ok(it) => it,
            Err(err) => panic!("{:#?}", err),
        };

        match next.data {
            Token::CloseParen => {
                let params = Parameters::new(Vec::new());
                let span = params.try_calculate_span();
                return ParseResult::ok(Ok(Wrapped::new(
                    open.to_empty(),
                    MaybeSpan::new(params, span),
                    next.to_empty(),
                )));
            }
            Token::String(_) | Token::Number(_) | Token::Iden(_) => {}
            _ => panic!("Should have been caught by peek_expect"),
        }
        while {
            match self.peek() {
                Ok(it) => match it.data {
                    Token::CloseParen => false,
                    _ => true,
                },
                Err(err) => {
                    return ParseResult::new(
                        Err(StatementRecovery),
                        Vec::new(),
                        Some(Box::new(err)),
                    )
                }
            }
        } {
            // Check if can parse argument
            let next = match self.peek_expect(
                &Token::POSSIBLE_PARAM,
                Some("Arg start"),
            ) {
                Ok(it) => it,
                Err(err) => {
                    return helper::recover_statement(self, err);
                }
            };
            // and parsing
            let item = match next.data {
                Token::String(_) | Token::Number(_) => {
                    let lit = match helper::should_return(self.literal()) {
                        Ok(it) => it,
                        Err(err) => return err,
                    };
                    Expression::Static(lit)
                }
                Token::Iden(_) => {
                    let lit = match helper::should_return(self.expression()) {
                        Ok(it) => it,
                        Err(err) => return err,
                    };
                    Expression::Literal(lit)
                }
                _ => panic!("Should be covered by peek expect"),
            };

            items.push(item);

            let tok = match self.peek_expect(&[Token::CloseParen, Token::Comma], None) {
                Ok(it) => it,
                Err(err) => return helper::recover_statement(self, err),
            };

            match tok.data {
                Token::Comma => {
                    self.next_assert(&[Token::Comma]);

                    let tok = match self.peek_expect(
                        // TODO make this a thing in lexer
                        &[
                            Token::CloseParen,
                            Token::Number(None),
                            Token::String(None),
                            Token::Iden(None),
                        ],
                        None,
                    ) {
                        Ok(it) => it,
                        Err(err) => return helper::recover_statement(self, err),
                    };

                    match tok.data {
                        Token::CloseParen => {
                            break;
                        }
                        Token::Number(_) | Token::String(_) | Token::Iden(_) => {}
                        _ => panic!("Should be covered by next expect"),
                    }
                }
                Token::CloseParen => {}
                _ => panic!("Should be covered by next expect"),
            };
        }

        let close = match self.next_expect(&[Token::CloseParen], None) {
            Ok(it) => it,
            Err(err) => return helper::recover_statement(self, err),
        };

        let params = Parameters::new(items);
        let span = params.try_calculate_span();

        ParseResult::ok(Ok(Wrapped::new(
            open.to_empty(),
            MaybeSpan::new(params, span),
            close.to_empty(),
        )))
    }

    pub fn expression(&mut self) -> ParseResult<Result<ExprLiteral, StatementRecovery>> {
        // loc
        let kind = self.next_assert(&[Token::Iden(None)]);
        let kind =
            kind.map_inner(|it| ExprLitType::from_spur(&it.get_iden_inner(), self.rodeo.clone()));

        // loc(
        let open = match self.next_expect(&[Token::OpenParen], None) {
            Ok(it) => it,
            Err(err) => return helper::recover_statement(self, err),
        };

        let mut items: Vec<StaticLiteral> = Vec::new();

        // loc(   check for ')', string or number
        let next = match self.peek_expect(
            &[Token::CloseParen, Token::String(None), Token::Number(None), Token::Iden(None)],
            None,
        ) {
            Ok(it) => it,
            Err(err) => panic!("{:#?}", err),
        };

        match next.data {
            Token::CloseParen => {
                println!("close");
                let params = Parameters::new(items);
                let span = params.try_calculate_span();

                let wrapped = Wrapped::new(
                    open.to_empty(),
                    MaybeSpan::new(params, span),
                    next.to_empty(),
                );
                return ParseResult::ok(Ok(ExprLiteral::new(kind, wrapped.calculate_span_wrap())));
            }
            Token::String(_) | Token::Number(_) | Token::Iden(_) => {}
            _ => panic!("Should have been caught by peek_expect"),
        }
        while {
            match self.peek() {
                Ok(it) => match it.data {
                    Token::CloseParen => false,
                    _ => true,
                },
                Err(err) => {
                    return ParseResult::new(
                        Err(StatementRecovery),
                        Vec::new(),
                        Some(Box::new(err)),
                    )
                }
            }
        } {
            let next = match self.peek_expect(
                &[Token::String(None), Token::Number(None)],
                Some("Arg start"),
            ) {
                Ok(it) => it,
                Err(err) => {
                    return helper::recover_statement(self, err);
                }
            };
            // and parsing
            let item = match next.data {
                Token::String(_) | Token::Number(_) => {
                    let lit = match helper::should_return(self.literal()) {
                        Ok(it) => it,
                        Err(err) => return err,
                    };
                    lit
                }
                _ => panic!("Should be covered by peek expect"),
            };

            items.push(item);

            let tok = match self.peek_expect(&[Token::CloseParen, Token::Comma], None) {
                Ok(it) => it,
                Err(err) => return helper::recover_statement(self, err),
            };

            match tok.data {
                Token::Comma => {
                    self.next_assert(&[Token::Comma]);

                    let tok = match self.peek_expect(
                        &[Token::CloseParen, Token::Number(None), Token::String(None)],
                        None,
                    ) {
                        Ok(it) => it,
                        Err(err) => return helper::recover_statement(self, err),
                    };

                    match tok.data {
                        Token::CloseParen => {
                            break;
                        }
                        Token::Number(_) | Token::String(_) => {}
                        _ => panic!("Should be covered by next expect"),
                    }
                }
                Token::CloseParen => {}
                _ => panic!("Should be covered by next expect"),
            };
        }

        let close = match self.next_expect(&[Token::CloseParen], None) {
            Ok(it) => it,
            Err(err) => return helper::recover_statement(self, err),
        };

        let params = Parameters::new(items).try_calculate_span_wrap();

        ParseResult::ok(Ok(ExprLiteral::new(
            kind,
            Wrapped::new(open.to_empty(), params, close.to_empty()).calculate_span_wrap(),
        )))
    }

    pub fn literal(&mut self) -> ParseResult<Result<StaticLiteral, StatementRecovery>> {
        let lit = self.next_assert(&[Token::String(None), Token::Number(None)]);
        let lit = match lit.data {
            Token::String(it) => {
                let str_lit = StringLiteral::new(it.expect("Lexer dosen't produce empty Strings"));
                StaticLiteral::String(Spanned::new(str_lit, lit.span))
            }
            Token::Number(it) => {
                let str_lit = NumberLiteral::new(it.expect("Lexer dosen't produce empty Strings"));
                StaticLiteral::Number(Spanned::new(str_lit, lit.span))
            }
            _ => panic!("Should be covered by next assert"),
        };
        ParseResult::ok(Ok(lit))
    }
}
