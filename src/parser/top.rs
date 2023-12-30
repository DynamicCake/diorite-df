use logos::Span;

use crate::ast::{
    recovery::{Recovery, StatementRecovery, TopLevelRecovery, TopRecoveryType},
    statement::{ActionType, IfStatement, SimpleStatement, Statement, Statements},
    top::{Event, EventType, FuncDef, ProcDef},
    Iden,
};

use super::error::*;
use super::stmt::*;
use super::*;

impl<'src> Parser<'src> {
    /// It is guaranteed that the next token will be a top level declaration token
    pub(super) fn top_level(&mut self) -> CompilerResult<TopLevel<'src>, CompilerError<'src>> {
        let token = match self.peek_expect(
            &ExpectedTokens::new(Token::TOP_LEVEL.to_vec()),
            Some("top level decleration token"),
        ) {
            Ok(it) => Spanned::new(it.data.to_owned(), it.span),
            Err(err) => {
                let CompilerResult { data, error } = self.top_recovery();
                let mut errors: Vec<CompilerError<'src>> =
                    error.into_iter().map(|it| it.into()).collect();
                errors.push(err);
                return CompilerResult::new(
                    TopLevel::Recovery(TopLevelRecovery::new(vec![
                        TopRecoveryType::Unrecognizable(data),
                    ])),
                    errors,
                );
            }
        };

        let top: _ = match &token.data {
            Token::PlayerEvent | Token::EntityEvent => {
                let CompilerResult { data, error } = self.event();
                let data = match data {
                    Ok(it) => TopLevel::Event(it),
                    Err(err) => TopLevel::Recovery(err),
                };
                CompilerResult::new(data, error)
            }
            Token::ProcDef => {
                let def = self.process();
                todo!()
            }
            Token::FuncDef => {
                let def = self.function();
                todo!()
            }
            it => {
                let CompilerResult { data, error } = self.top_recovery();
                let error = error.into_iter().map(|it| it.into()).collect();
                let data = TopLevel::Recovery(TopLevelRecovery::new(vec![
                    TopRecoveryType::Unrecognizable(data),
                ]));

                CompilerResult::new(data, error)
            }
        };

        top
    }

    fn process(&mut self) -> CompilerResult<ProcDef<'src>, CompilerError<'src>> {
        todo!()
    }

    fn function(&mut self) -> CompilerResult<FuncDef<'src>, CompilerError<'src>> {
        todo!()
    }

    /// Represents an event delceration
    /// `pevent Join (statements) end`
    /// If the compiler result data is None, then it can be treated as malformed
    fn event(
        &mut self,
    ) -> CompilerResult<Result<Event<'src>, TopLevelRecovery<'src>>, CompilerError<'src>> {
        let definition = self.next_assert(
            &ExpectedTokens::new(vec![Token::PlayerEvent, Token::PlayerEvent]),
            Some("event token"),
        );

        let type_tok = match definition.data {
            Token::PlayerEvent => EventType::Player,
            Token::EntityEvent => EventType::Entity,
            it => panic!(
                "Expected PlayerEvent or EntityEvent token, received {:?}",
                it
            ),
        };

        let name = match self.next_expect(&Token::Iden("").into(), Some("event name")) {
            Ok(it) => it,
            Err(err) => {
                let CompilerResult { data, error } = self.top_recovery();
                let mut def =
                    TopLevelRecovery::new(vec![TopRecoveryType::Unrecognizable(vec![definition])]);
                def.items.push(TopRecoveryType::Unrecognizable(data));
                let mut errors: Vec<CompilerError<'src>> =
                    error.into_iter().map(|it| it.into()).collect();
                errors.push(err);
                return CompilerResult::new(Err(def), errors);
            }
        };

        let CompilerResult {
            data: stmts,
            error: mut errors,
        } = self.statements();

        let end = match self.next_expect(&Token::End.into(), None) {
            Ok(it) => it.to_empty(),
            Err(err) => {
                let mut toks = Vec::new();
                toks.push(definition);
                toks.push(name);
                match &err {
                    CompilerError::Unexpected(it) => {
                        let CompilerResult { data, error } = self.top_recovery();
                        
                        // vec![TopRecoveryType::Body(stmts)]);
                        todo!()
                    }
                    CompilerError::UnexpectedEOF(it) => {
                        return CompilerResult::new(
                            Err(TopLevelRecovery::new(vec![TopRecoveryType::Body(stmts)])),
                            errors,
                        );
                    }
                    CompilerError::LexerError(it) => {
                    }
                }
                return CompilerResult::new(Result::Err(TopLevelRecovery::new(todo!())), errors);
                errors.push(err);
                // TODO HERE
            }
        };

        let range = {
            match stmts.first() {
                Some(first) => {
                    let last = stmts.last().expect("If the first exists, the last exists");

                    last.span.start..last.span.end
                }
                None => {
                    // Get a bit desperate here
                    name.span.start..end.span.end
                }
            }
        };

        let iden = {
            let data = match name.data {
                Token::Iden(it) => it,
                it => panic!("Expected Iden received {:?}", it),
            };
            Spanned::new(Iden::new(data), name.span)
        };

        let event = Event::new(
            Spanned::new(type_tok, definition.span),
            iden,
            Spanned::new(Statements::new(stmts), range),
            end,
        );

        CompilerResult::new(Ok(event), errors)
    }

    /// Looks for event, proc, func tokens
    /// This function will never syntax error
    fn top_recovery(&mut self) -> CompilerResult<Vec<Spanned<Token<'src>>>, TokAdvanceError<'src>> {
        let mut errors = Vec::new();
        let mut tokens = Vec::new();
        loop {
            match self.peek() {
                Ok(tok) => match tok.data {
                    Token::PlayerEvent | Token::EntityEvent | Token::FuncDef | Token::ProcDef => {
                        break;
                    }

                    _ => {
                        let a = self.next().expect("Peek succeeded before");
                        tokens.push(Spanned::new(a.data, a.span));
                    }
                },
                Err(err) => {
                    match &err {
                        TokAdvanceError::UnexpectedEOF(UnexpectedEOF {
                            expected: _,
                            expected_name: _,
                        }) => {
                            return CompilerResult::single_err(tokens, err);
                        }
                        TokAdvanceError::Lexer(span) => {
                            tokens.push(Token::Invalid.spanned(span.token.span.clone()));
                        }
                    }
                    errors.push(err);
                }
            };
        }
        CompilerResult::new(tokens, errors)
    }
}
