use self::ext::*;

use super::*;
use crate::ast::recovery::StatementRecovery;

use crate::ast::CalcThenWrap;
use crate::ast::Iden;
use crate::{
    ast::{
        statement::{ActionType, IfStatement, SimpleStatement},
        Spanned,
    },
    lexer::Token,
};

impl<'lex> Parser<'lex> {
    pub fn regular_statement(
        &mut self,
    ) -> ParseResult<Result<SimpleStatement, StatementRecovery>, Vec<UnexpectedToken>> {
        // paction
        let type_tok = self.next_assert(&Token::SIMPLE_STATEMENT);

        // paction SendMessage
        let action = adv_stmt!(self, self.next_expect(&[Token::Iden(None)], None));

        // paction SendMessage <selection>
        let selector_start = match self.peek() {
            Ok(it) => it.data,
            Err(err) => {
                return ParseResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)))
            }
        };

        let selection = match selector_start {
            Token::OpenComp => {
                let selector = match helper::should_return(self.selector()) {
                    Ok(it) => it,
                    Err(err) => return err,
                };
                Some(selector)
            }
            _ => None,
        };

        // SendMessage <selection> [key: 'and value', pairs: here,]
        let selector_start = match self.peek() {
            Ok(it) => it.data,
            Err(err) => {
                return ParseResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)));
            }
        };

        let tags = match selector_start {
            Token::OpenBracket => {
                let tags = match helper::should_return(self.tags()) {
                    Ok(it) => it,
                    Err(err) => return err,
                };

                Some(tags)
            }
            _ => None,
        };

        let params = match self.peek_expect(&[Token::OpenParen], None) {
            Ok(_) => should_return!(self.call_params()).calculate_span_wrap(),
            Err(err) => return helper::recover_statement(self, err),
        };

        let selection = selection.map(|sel| {
            sel.calculate_span_wrap()
        });

        let type_tok = type_tok.map_inner(|inner| {
            ActionType::from_token(inner).expect("A non action token managed to sneak in")
        });

        let tags = tags.map(|it| {
            it.calculate_span_wrap()
        });

        ParseResult::ok(Ok(SimpleStatement {
            type_tok,
            action: action.map_inner(|i| Iden::new(i.get_iden_inner())),
            selection,
            tags,
            params,
        }))
    }

    pub fn if_statement(&mut self) -> ParseResult<IfStatement> {
        todo!()
    }
}
