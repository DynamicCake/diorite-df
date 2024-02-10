use std::char::ParseCharError;
use std::ops::Range;

use super::error::*;
use super::top::*;
use super::*;
use crate::ast::recovery::StatementRecovery;
use crate::ast::recovery::TopLevelRecovery;
use crate::ast::statement::Expression;
use crate::ast::statement::IdenPair;
use crate::ast::statement::Selection;
use crate::ast::statement::Tags;
use crate::ast::CalcSpan;
use crate::ast::Iden;
use crate::ast::MaybeSpan;
use crate::ast::Parameters;
use crate::ast::TryCalcSpan;
use crate::{
    ast::{
        statement::{ActionType, IfStatement, SimpleStatement, Statement, Statements},
        Spanned,
    },
    lexer::Token,
    parser::error::ExpectedTokens,
};

use super::{error::CompilerResult, Parser};

impl<'src> Parser<'src> {
    pub fn statements(
        &mut self,
    ) -> CompilerResult<'src, Vec<Statement<'src>>, Vec<UnexpectedToken<'src>>> {
        let mut statements: Vec<Statement<'src>> = Vec::new();
        let mut errors = Vec::new();

        loop {
            match self.peek_expect(&Token::STATEMENT_LOOP, Some("statement declaration or end")) {
                Ok(it) => {
                    // Tokens are guaranteed to be end or statement
                    if let Token::End = it.data {
                        break;
                    } else {
                        let CompilerResult {
                            data,
                            mut error,
                            at_eof,
                        } = self.statement();
                        errors.append(&mut error);
                        match data {
                            Ok(it) => {
                                statements.push(it);
                            }
                            Err(_err) => statements.push(Statement::Recovery),
                        };
                        // Because it is in a loop, a break will happen if at_eof is some
                        if let Some(at_eof) = at_eof {
                            return CompilerResult::new(statements, errors, Some(at_eof));
                        }
                    }
                }
                Err(err) => {
                    match err {
                        AdvanceUnexpected::Token(err) => {
                            errors.push(err);
                        }
                        AdvanceUnexpected::Eof(err) => {
                            return CompilerResult::new(statements, errors, Some(Box::new(err)))
                        }
                    }
                    let at_eof = self.statement_recovery();

                    // If the tokens are empty, there is no reason to push them to the output as no
                    // processing is going to be done on them
                    statements.push(Statement::Recovery);
                    if let Some(at_eof) = at_eof {
                        return CompilerResult::new(statements, errors, Some(at_eof));
                    }
                }
            }
        }

        CompilerResult::new(statements, errors, None)
    }

    pub fn statement_recovery(
        &mut self,
    ) -> Option<Box<UnexpectedEOF<'src>>> {
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
                        self.advance().expect("Peek succeeded before");
                    }
                },
                Err(err) => {
                    return Some(Box::new(err));
                }
            };
        }
        None
    }

    fn statement(
        &mut self,
    ) -> CompilerResult<'src, Result<Statement<'src>, StatementRecovery>, Vec<UnexpectedToken<'src>>>
    {
        let decl_token = match self.peek_expect(&Token::STATEMENT, Some("statements")) {
            Ok(it) => it.data.to_owned().spanned(it.span),
            Err(err) => match err {
                AdvanceUnexpected::Token(err) => {
                    return CompilerResult::new(Err(StatementRecovery), vec![err], None)
                }
                AdvanceUnexpected::Eof(err) => {
                    return CompilerResult::new(
                        Err(StatementRecovery),
                        Vec::new(),
                        Some(Box::new(err)),
                    )
                }
            },
        };

        // I am aware that this will become a nightmare when adding new tokens or features... Too bad!
        // TODO create statement functions
        match decl_token.data {
            Token::PlayerAction
            | Token::EntityAction
            | Token::GameAction
            | Token::Control
            | Token::CallFunction
            | Token::CallProcess
            | Token::Select
            | Token::SetVar => {
                let CompilerResult {
                    data,
                    error,
                    at_eof,
                } = self.regular_statement();

                match data {
                    Ok(it) => {
                        let span = it.calc_span();
                        CompilerResult::new(
                            Ok(Statement::Simple(Spanned::new(it, span))),
                            error,
                            at_eof,
                        )
                    }
                    Err(err) => CompilerResult::new(Err(err), error, at_eof),
                }
            }
            Token::IfPlayer | Token::IfEntity | Token::IfGame | Token::IfVar => {
                self.if_statement();
                todo!()
            }
            _ => {
                let at_eof = self.statement_recovery();
                return CompilerResult::new(Err(StatementRecovery), Vec::new(), at_eof);
            }
        }

        // self.consume(expected, None);
        // CompilerResult::new(Ok(Spanned::new(, 1..2)), Vec::new(), None)
    }

    fn regular_statement(
        &mut self,
    ) -> CompilerResult<
        'src,
        Result<SimpleStatement<'src>, StatementRecovery>,
        Vec<UnexpectedToken<'src>>,
    > {
        // paction
        let type_tok = self.next_assert(&Token::SIMPLE_STATEMENT, Some("simple statement"));

        // SendMessage
        let action = match self.next_expect(&[Token::Iden(None)], None) {
            Ok(it) => it,
            Err(err) => return helper::recover_statement(self, err),
        };

        // <selection>
        let selector_start = match self.peek() {
            Ok(it) => it.data,
            Err(err) => {
                return CompilerResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)))
            }
        };

        let selection = match selector_start {
            Token::OpenComp => {
                let CompilerResult {
                    data,
                    error,
                    at_eof,
                } = self.selector();
                let selector = match data {
                    Ok(it) => {
                        if at_eof.is_some() {
                            return CompilerResult::new(Err(StatementRecovery), error, at_eof);
                        }
                        it
                    }
                    Err(err) => {
                        return CompilerResult::new(Err(err), error, at_eof);
                    }
                };
                Some(selector)
            }
            _ => None,
        };

        // [key: 'and value', pairs: here,]
        let selector_start = match self.peek() {
            Ok(it) => it.data,
            Err(err) => {
                return CompilerResult::new(
                    Err(StatementRecovery),
                    Vec::new(),
                    Some(Box::new(err)),
                );
            }
        };

        let tags = match selector_start {
            Token::OpenBracket => {
                let CompilerResult {
                    data,
                    error,
                    at_eof,
                } = self.tags();
                let tags = match data {
                    Ok(it) => {
                        if at_eof.is_some() {
                            return CompilerResult::new(Err(StatementRecovery), error, at_eof);
                        }
                        it
                    }
                    Err(err) => {
                        return CompilerResult::new(Err(err), error, at_eof);
                    }
                };
                Some(tags)
            }
            _ => None,
        };

        let selection = selection.map(|sel| {
            let span = sel.calculate_span();
            Spanned::new(sel, span)
        });

        let type_tok = type_tok.map_inner(|inner| {
            ActionType::from_token(inner).expect("A non action token managed to sneak in")
        });

        let tags = tags.map(|it| {
            let span = it.calculate_span();
            Spanned::new(it, span)
        });

        CompilerResult::new(
            Ok(SimpleStatement {
                type_tok,
                action: action.map_inner(|i| Iden::new(i.get_iden_inner())),
                selection,
                tags,
                params: Spanned::new(Parameters { items: Vec::new() }, Range { start: 0, end: 0 }),
            }),
            Vec::new(),
            None,
        )
    }

    fn if_statement(&mut self) -> CompilerResult<'src, IfStatement<'src>> {
        const EXPECTED: [Token<'_>; 4] = [
            Token::IfPlayer,
            Token::IfEntity,
            Token::IfGame,
            Token::IfVar,
        ];
        todo!()
    }

    /// Must start with a `[`
    fn tags(&mut self) -> CompilerResult<'src, Result<Tags<'src>, StatementRecovery>> {
        let open = self.next_assert(&[Token::OpenBracket], None);

        let CompilerResult {
            data,
            error,
            at_eof,
        } = self.pair_list();
        let params = match data {
            Ok(it) => {
                if at_eof.is_some() {
                    return CompilerResult::new(Err(StatementRecovery), error, at_eof);
                }
                it
            }
            Err(err) => {
                return CompilerResult::new(Err(err), error, at_eof);
            }
        };

        let close = match self.next_expect(&[Token::CloseBracket], None) {
            Ok(it) => it,
            Err(err) => {
                return helper::recover_statement(self, err);
            }
        };

        let tag_span = params.try_calculate_span();
        CompilerResult::new(
            Ok(Tags {
                open: open.to_empty(),
                tags: Some(MaybeSpan::new(params, tag_span)),
                close: close.to_empty(),
            }),
            Vec::new(),
            None,
        )
    }

    fn pair_list(
        &mut self,
    ) -> CompilerResult<'src, Result<Parameters<IdenPair<'src>>, StatementRecovery>> {
        let mut pairs = Vec::new();

        let next = match self.peek_expect(&[Token::CloseBracket, Token::Iden(None)], None) {
            Ok(it) => it,
            // TODO pass in correct token list
            Err(err) => return helper::recover_statement(self, err),
        };

        match next.data {
            Token::CloseBracket => {
                return CompilerResult::new(Ok(Parameters::new(pairs)), Vec::new(), None)
            }
            Token::Iden(_) => {}
            _ => panic!("Should have been caught by peek_expect"),
        }

        loop {
            let CompilerResult {
                data,
                error,
                at_eof,
            } = self.iden_pair();
            let pair = match data {
                Ok(it) => {
                    if at_eof.is_some() {
                        return CompilerResult::new(Err(StatementRecovery), error, at_eof);
                    }
                    it
                }
                Err(err) => {
                    return CompilerResult::new(Err(err), error, at_eof);
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
                            // TODO I had it with recovery tokens
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

        CompilerResult::new(Ok(Parameters::new(pairs)), Vec::new(), None)
    }

    /// Must start with an iden
    fn iden_pair(&mut self) -> CompilerResult<'src, Result<IdenPair<'src>, StatementRecovery>> {
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

        CompilerResult::new(Ok(IdenPair { key, colon, value }), Vec::new(), None)
    }

    /// Must start with a `<`
    fn selector(&mut self) -> CompilerResult<'src, Result<Selection<'src>, StatementRecovery>> {
        let open = self.next_assert(&[Token::OpenComp], None);

        let selection = match self.next_expect(&[Token::Iden(None)], None) {
            Ok(it) => it,
            Err(err) => {
                return match err {
                    AdvanceUnexpected::Token(it) => {
                        match it.received.data {
                            // This allows <>
                            Token::CloseComp => CompilerResult::new(
                                Ok(Selection {
                                    open: open.to_empty(),
                                    selection: None,
                                    close: Spanned::<()>::empty(it.received.span),
                                }),
                                Vec::new(),
                                None,
                            ),
                            _ => {
                                let at_eof = self.statement_recovery();
                                CompilerResult::new(Err(StatementRecovery), Vec::new(), at_eof)
                            }
                        }
                    }
                    AdvanceUnexpected::Eof(err) => {
                        CompilerResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)))
                    }
                };
            }
        };

        let close = match self.next_expect(&[Token::CloseComp], None) {
            Ok(it) => it,
            Err(err) => return helper::recover_statement(self, err),
        };

        CompilerResult::new(
            Ok(Selection {
                open: open.to_empty(),
                selection: Some(selection.map_inner(|it| it.get_iden_inner())),
                close: close.to_empty(),
            }),
            Vec::new(),
            None,
        )
    }
}
