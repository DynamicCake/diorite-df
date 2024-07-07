use self::ext::*;

use super::*;
use crate::error::syntax::UnexpectedToken;
use crate::span::CalcThenWrap;
use crate::tree::recovery::StatementRecovery;

use crate::tree::statement::{ElseBlock, RepeatLoop, Statements};
use crate::tree::Iden;
use crate::{
    lexer::Token,
    tree::statement::{IfStatement, SimpleStatement},
};

impl<'lex> Parser<'lex> {
    pub fn regular_statement(
        &mut self,
    ) -> ParseResult<Result<SimpleStatement, StatementRecovery>, Vec<UnexpectedToken>> {
        // paction
        let type_tok = self
            .next_assert(&Token::SIMPLE_STATEMENT)
            .map_inner(|inner| {
                inner
                    .try_into()
                    .expect("A non action token managed to sneak in")
            });

        // paction SendMessage
        let action = adv_stmt!(self, self.next_expect(&[Token::Iden(None)], None))
            .map_inner(|i| Iden::new(i.get_iden_inner()));
        // paction SendMessage <selection>
        let selection = match self.peek() {
            Ok(it) => match it.data {
                Token::OpenComp => Some((should_return!(self.selector())).calculate_span_wrap()),
                _ => None,
            },
            Err(err) => {
                return ParseResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)))
            }
        };

        // SendMessage <selection> [key: 'and value', pairs: here,]
        let tags = match self.peek() {
            Ok(it) => match it.data {
                Token::OpenBracket => Some((should_return!(self.tags())).calculate_span_wrap()),
                _ => None,
            },
            Err(err) => {
                return ParseResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)));
            }
        };

        let params = match self.peek_expect(&[Token::OpenParen], None) {
            Ok(_) => should_return!(self.call_params()).calculate_span_wrap(),
            Err(err) => return helper::recover_statement(self, err),
        };

        ParseResult::ok(Ok(SimpleStatement {
            type_tok,
            action,
            selection,
            tags,
            params,
        }))
    }

    pub fn if_statement(&mut self) -> ParseResult<Result<IfStatement, StatementRecovery>> {
        // ifplayer
        let type_tok = self
            .next_assert(&Token::IF_STATEMENT)
            .map_inner(|tok| tok.try_into().expect("next assert should cover"));

        // ifplayer not
        let not = adv_stmt!(
            self,
            self.peek_expect(&[Token::Not, Token::Iden(None)], Some("Action or NOT"))
        );

        let not = match not.data {
            Token::Not => {
                let tok = self.next_assert(&[Token::Not]);
                Some(tok.to_empty())
            }
            Token::Iden(_) => None,
            _ => panic!("Peek should catch"),
        };

        // ifplayer not IsLookingAt
        let action = adv_stmt!(self, self.next_expect(&[Token::Iden(None)], Some("Action")))
            .map_inner(|it| Iden::new(it.get_iden_inner()));

        // not IsLookingAt <default>
        let selection = match self.peek() {
            Ok(it) => match it.data {
                Token::OpenComp => Some((should_return!(self.selector())).calculate_span_wrap()),
                _ => None,
            },
            Err(err) => {
                return ParseResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)))
            }
        };

        // IsLookingAt <default> ['Fluid Mode': 'Ignore Fluids']
        let tags = match self.peek() {
            Ok(it) => match it.data {
                Token::OpenBracket => Some((should_return!(self.tags())).calculate_span_wrap()),
                _ => None,
            },
            Err(err) => {
                return ParseResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)));
            }
        };

        // <default> ['Fluid Mode': 'Ignore Fluids'] (item('{Count:1b,id:"minecraft:grass_block"}'))
        let params = match self.peek_expect(&[Token::OpenParen], None) {
            Ok(_) => should_return!(self.call_params()).calculate_span_wrap(),
            Err(err) => return helper::recover_statement(self, err),
        };

        // ['Fluid Mode': 'Ignore Fluids'] (item('{Count:1b,id:"minecraft:grass_block"}'))
        //      gaction Explosion (gval(Location), 4)
        let ParseResult {
            data: statements,
            // NOTE use this vec when returning
            mut error,
            at_eof,
        } = self.statements(true);
        if at_eof.is_some() {
            return ParseResult::new(Err(StatementRecovery), error, at_eof);
        };

        // (item('{Count:1b,id:"minecraft:grass_block"}'))
        //      gaction Explosion (gval(Location), 4)
        // end
        let what_now = adv_stmt!(self, self.peek_expect(&[Token::End, Token::Else], None));

        //      gaction Explosion (gval(Location), 4)
        // else
        //      <whatever>
        let else_block = match what_now.data {
            Token::End => None,
            Token::Else => {
                let else_tok = self.next_assert(&[Token::Else]);
                let ParseResult {
                    data,
                    error: mut errs,
                    at_eof,
                } = self.statements(false);
                if at_eof.is_some() {
                    return ParseResult::new(Err(StatementRecovery), error, at_eof);
                };
                error.append(&mut errs);

                Some(ElseBlock {
                    else_tok: else_tok.to_empty(),
                    statements: Statements::new(data),
                })
            }
            _ => panic!("should be covered by next expect"),
        };

        //  else
        //      <whatever>
        //  end
        let end = adv_stmt!(self, self.next_expect(&[Token::End], None)).to_empty();

        // NOTE See the part where statements get parsed, you might want to return that error vec
        ParseResult::new(
            Ok(IfStatement {
                type_tok,
                not,
                action,
                selection,
                tags,
                params,
                statements: Statements::new(statements),
                else_block,
                end,
            }),
            error,
            None,
        )
    }

    pub fn repeat(&mut self) -> ParseResult<Result<RepeatLoop, StatementRecovery>> {
        // repeat
        let type_tok = self.next_assert(&[Token::Repeat]).to_empty();

        // repeat While
        let action = adv_stmt!(self, self.next_expect(&[Token::Iden(None)], Some("Action")))
            .map_inner(|it| Iden::new(it.get_iden_inner()));

        // repeat While <IsSneaking>
        let selection = match self.peek() {
            Ok(it) => match it.data {
                Token::OpenComp => Some((should_return!(self.selector())).calculate_span_wrap()),
                _ => None,
            },
            Err(err) => {
                return ParseResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)))
            }
        };

        // While <IsSneaking> []
        let tags = match self.peek() {
            Ok(it) => match it.data {
                Token::OpenBracket => Some((should_return!(self.tags())).calculate_span_wrap()),
                _ => None,
            },
            Err(err) => {
                return ParseResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)));
            }
        };

        // <IsSneaking> [] ()
        let params = match self.peek_expect(&[Token::OpenParen], None) {
            Ok(_) => should_return!(self.call_params()).calculate_span_wrap(),
            Err(err) => return helper::recover_statement(self, err),
        };

        // [] ()
        //      gaction Explosion (gval(Location), 4)
        let ParseResult {
            data: statements,
            // NOTE use this vec when returning
            error,
            at_eof,
        } = self.statements(true);
        if at_eof.is_some() {
            return ParseResult::new(Err(StatementRecovery), error, at_eof);
        };

        // ()
        //      gaction Explosion (gval(Location), 4)
        // end
        let end = adv_stmt!(self, self.next_expect(&[Token::End], None)).to_empty();

        ParseResult::new(
            Ok(RepeatLoop {
                type_tok,
                action,
                selection,
                tags,
                params,
                statements: Statements::new(statements),
                end,
            }),
            error,
            None,
        )
    }
}
