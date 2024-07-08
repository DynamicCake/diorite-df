use crate::common::{span::*, *};
use crate::tree::recovery::StatementRecovery;

use super::ext::*;
use super::*;
use crate::lexer::Token;
use crate::tree::statement::*;
use crate::tree::*;

impl Parser<'_> {
    /// Must start with a `[`
    pub fn tags(&mut self) -> ParseResult<Result<Tags, StatementRecovery>> {
        // [
        let open = self.next_assert(&[Token::OpenBracket]).to_empty();

        // [ key: value
        let tags = should_return!(self.pair_list()).try_calculate_span_wrap();

        // [ key: value ]
        let close = adv_stmt!(self, self.next_expect(&[Token::CloseBracket], None)).to_empty();

        ParseResult::ok(Ok(Tags::new(open, tags, close)))
    }

    /// Must start with an iden
    pub fn pair_list(&mut self) -> ParseResult<Result<Parameters<IdenPair>, StatementRecovery>> {
        let mut pairs = Vec::new();

        // Allow [] to happen
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
            let pair = should_return!(self.iden_pair());

            pairs.push(pair);

            let tok = adv_stmt!(
                self,
                self.peek_expect(&[Token::CloseBracket, Token::Comma], None)
            );

            match tok.data {
                Token::CloseBracket => break,
                // Here we go again
                Token::Comma => {
                    self.next_assert(&[Token::Comma]);

                    let tok = adv_stmt!(
                        self,
                        self.peek_expect(&[Token::CloseBracket, Token::Iden(None)], None)
                    );

                    match tok.data {
                        // To allow trailing commas
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
        // key
        let key = self
            .next_assert(&[Token::Iden(None)])
            .map_inner(|it| it.get_iden_inner());

        // key:
        let colon = adv_stmt!(self, self.next_expect(&[Token::Colon], None)).to_empty();

        // key: value
        let value = adv_stmt!(self, self.next_expect(&[Token::Iden(None)], None))
            .map_inner(|it| it.get_iden_inner());

        ParseResult::ok(Ok(IdenPair { key, colon, value }))
    }

    /// Must start with a `<`
    pub fn selector(&mut self) -> ParseResult<Result<Selection, StatementRecovery>> {
        // <
        let open = self.next_assert(&[Token::OpenComp]);

        // < default
        // To allow <>
        // Lengthy but you win some you lose some
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

        // < default >
        let close = adv_stmt!(self, self.next_expect(&[Token::CloseComp], None));

        ParseResult::ok(Ok(Selection {
            open: open.to_empty(),
            selection: Some(selection.map_inner(|it| it.get_iden_inner())),
            close: close.to_empty(),
        }))
    }

    // Must start with '('
    pub fn call_params(&mut self) -> ParseResult<Result<Wrapped<Expression>, StatementRecovery>> {
        // (
        let open = self.next_assert(&[Token::OpenParen]);

        let mut items: Vec<Expression> = Vec::new();

        // Allowing ()
        // () or ( 1      ...
        let next = adv_stmt!(
            self,
            self.peek_expect(
                &[
                    Token::CloseParen,
                    Token::String(None),
                    Token::Number(None),
                    Token::Iden(None),
                ],
                None,
            )
        );
        match next.data {
            // TODO this should be done by default without needing this
            Token::CloseParen => {
                let next = self.next_assert(&[Token::CloseParen]);
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
            // Only repeat when ')' not in sight
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
            // Check if can parse argument if iden, string, number
            let next = adv_stmt!(
                self,
                self.peek_expect(&Token::POSSIBLE_PARAM, Some("Arg start"))
            );

            // and parsing
            let item = match next.data {
                Token::String(_) | Token::Number(_) => {
                    Expression::Literal(should_return!(self.literal()))
                }
                Token::Iden(_) => Expression::Expr(should_return!(self.expression())),
                _ => panic!("Should be covered by peek expect"),
            };

            items.push(item);

            let tok = adv_stmt!(
                self,
                self.peek_expect(&[Token::CloseParen, Token::Comma], None)
            );

            match tok.data {
                Token::Comma => {
                    self.next_assert(&[Token::Comma]);

                    // TODO make this a thing in lexer
                    let tok = adv_stmt!(
                        self,
                        self.peek_expect(
                            &[
                                Token::CloseParen,
                                Token::Number(None),
                                Token::String(None),
                                Token::Iden(None),
                            ],
                            None,
                        )
                    );
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

        let close = adv_stmt!(self, self.next_expect(&[Token::CloseParen], None));

        let params = Parameters::new(items).try_calculate_span_wrap();

        ParseResult::ok(Ok(Wrapped::new(open.to_empty(), params, close.to_empty())))
    }

    pub fn expression(&mut self) -> ParseResult<Result<ExprLiteral, StatementRecovery>> {
        // loc
        let kind = self
            .next_assert(&[Token::Iden(None)])
            .map_inner(|it| Iden::new(it.get_iden_inner()));

        // loc(
        let open = adv_stmt!(self, self.next_expect(&[Token::OpenParen], None));

        let mut items: Vec<ExprValue> = Vec::new();

        // loc(   check for ')', string or number
        let next = adv_stmt!(
            self,
            self.peek_expect(
                &[Token::CloseParen, Token::Number(None), Token::Iden(None),],
                None,
            )
        );

        match next.data {
            Token::CloseParen => {
                let next = self.next_assert(&[Token::CloseParen]);
                let wrapped = Wrapped::new(
                    open.to_empty(),
                    Parameters::new(items).try_calculate_span_wrap(),
                    next.to_empty(),
                );
                return ParseResult::ok(Ok(ExprLiteral::new(kind, wrapped.calculate_span_wrap())));
            }
            Token::Number(_) | Token::Iden(_) => {}
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
            let next = adv_stmt!(
                self,
                self.peek_expect(&[Token::Iden(None), Token::Number(None)], Some("Arg start"),)
            );

            // and parsing
            let item = match next.data {
                Token::Iden(_) | Token::Number(_) => should_return!(self.expr_value()),
                _ => panic!("Should be covered by peek expect"),
            };

            items.push(item);

            let tok = adv_stmt!(
                self,
                self.peek_expect(&[Token::CloseParen, Token::Comma], None)
            );

            match tok.data {
                Token::Comma => {
                    self.next_assert(&[Token::Comma]);

                    let tok = adv_stmt!(
                        self,
                        self.peek_expect(
                            &[Token::CloseParen, Token::Number(None), Token::Iden(None)],
                            None,
                        )
                    );

                    match tok.data {
                        Token::CloseParen => {
                            break;
                        }
                        Token::Number(_) | Token::Iden(_) => {}
                        _ => panic!("Should be covered by next expect"),
                    }
                }
                Token::CloseParen => {}
                _ => panic!("Should be covered by next expect"),
            };
        }

        let close = adv_stmt!(self, self.next_expect(&[Token::CloseParen], None));

        let params = Parameters::new(items).try_calculate_span_wrap();

        ParseResult::ok(Ok(ExprLiteral::new(
            kind,
            Wrapped::new(open.to_empty(), params, close.to_empty()).calculate_span_wrap(),
        )))
    }

    pub fn literal(&mut self) -> ParseResult<Result<StaticLiteral, StatementRecovery>> {
        let lit = self.next_assert(&[Token::String(None), Token::Number(None)]);
        let lit = match lit.data {
            Token::String(it) => StaticLiteral::String(lit.map_inner(|_| {
                StringLiteral::new(it.expect("Lexer dosen't produce empty Strings"))
            })),
            Token::Number(it) => StaticLiteral::Number(lit.map_inner(|_| {
                NumberLiteral::new(it.expect("Lexer dosen't produce empty Strings"))
            })),
            _ => panic!("Should be covered by next assert"),
        };
        ParseResult::ok(Ok(lit))
    }

    pub fn expr_value(&mut self) -> ParseResult<Result<ExprValue, StatementRecovery>> {
        let lit = self.next_assert(&[Token::Iden(None), Token::Number(None)]);
        let lit = match lit.data {
            Token::Iden(it) => ExprValue::Iden(
                lit.map_inner(|_| Iden::new(it.expect("Lexer dosen't produce empty Strings"))),
            ),
            Token::Number(it) => ExprValue::Number(lit.map_inner(|_| {
                NumberLiteral::new(it.expect("Lexer dosen't produce empty Strings"))
            })),
            _ => panic!("Should be covered by next assert"),
        };
        ParseResult::ok(Ok(lit))
    }
}
