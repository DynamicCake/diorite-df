use super::*;
use crate::ast::recovery::StatementRecovery;

use crate::ast::CalcSpan;
use crate::ast::Iden;
use crate::ast::Parameters;
use crate::{
    ast::{
        statement::{ActionType, IfStatement, SimpleStatement},
        Spanned,
    },
    lexer::Token,
};

impl<'src> Parser<'src> {
    pub fn regular_statement(
        &mut self,
    ) -> ParseResult<
        'src,
        Result<SimpleStatement<'src>, StatementRecovery>,
        Vec<UnexpectedToken<'src>>,
    > {
        // paction
        let type_tok = self.next_assert(&Token::SIMPLE_STATEMENT);

        // SendMessage
        let action = match self.next_expect(&[Token::Iden(None)], None) {
            Ok(it) => it,
            Err(err) => return helper::recover_statement(self, err),
        };

        // <selection>
        let selector_start = match self.peek() {
            Ok(it) => it.data,
            Err(err) => {
                return ParseResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)))
            }
        };

        let selection = match selector_start {
            Token::OpenComp => {
                let ParseResult {
                    data,
                    error,
                    at_eof,
                } = self.selector();
                let selector = match data {
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
                Some(selector)
            }
            _ => None,
        };

        // [key: 'and value', pairs: here,]
        let selector_start = match self.peek() {
            Ok(it) => it.data,
            Err(err) => {
                return ParseResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)));
            }
        };

        let tags = match selector_start {
            Token::OpenBracket => {
                let ParseResult {
                    data,
                    error,
                    at_eof,
                } = self.tags();
                let tags = match data {
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
                Some(tags)
            }
            _ => None,
        };

        let params = match self.peek_expect(&[Token::OpenParen], None) {
            Ok(_) => {
                let ParseResult {
                    data,
                    error,
                    at_eof,
                } = self.call_params();
                match data {
                    Ok(it) => {
                        if at_eof.is_some() {
                            return ParseResult::new(Err(StatementRecovery), error, at_eof);
                        }
                        let span = it.calculate_span();
                        Spanned::new(it, span)
                    }
                    Err(err) => return ParseResult::new(Err(err), error, at_eof)
                }
            }
            Err(err) => return helper::recover_statement(self, err),
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

        ParseResult::ok(Ok(SimpleStatement {
            type_tok,
            action: action.map_inner(|i| Iden::new(i.get_iden_inner())),
            selection,
            tags,
            params
        }))
    }

    pub fn if_statement(&mut self) -> ParseResult<'src, IfStatement<'src>> {
        todo!()
    }
}
