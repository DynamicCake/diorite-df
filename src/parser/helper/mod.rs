use crate::{
    ast::{recovery::StatementRecovery, Flatten, Spanned},
    lexer::Token,
};

use super::{AdvanceUnexpected, CompilerResult, Parser, UnexpectedToken};

pub fn recover_statement<'src, T>(
    parser: &mut Parser<'src>,
    err: AdvanceUnexpected<'src>,
    tokens: Vec<Spanned<Token<'src>>>,
) -> CompilerResult<'src, Result<T, StatementRecovery<'src>>, Vec<UnexpectedToken<'src>>> {
    match err {
        AdvanceUnexpected::Token(err) => {
            let CompilerResult {
                data,
                error: _,
                at_eof,
            } = parser.statement_recovery(tokens);
            CompilerResult::new(Err(data), vec![err], at_eof)
        }
        AdvanceUnexpected::Eof(err) => CompilerResult::new(
            Err(StatementRecovery::new(tokens)),
            Vec::new(),
            Some(Box::new(err)),
        ),
    }
}

pub fn handle_result_statement<'src, T, E>(
    mut tokens: Vec<Spanned<Token<'src>>>,
    CompilerResult {
        data,
        error,
        at_eof,
    }: CompilerResult<'src, Result<T, StatementRecovery<'src>>>,
) -> Result<T, CompilerResult<'src, Result<E, StatementRecovery<'src>>, Vec<UnexpectedToken<'src>>>>
where
    T: Flatten<'src>,
{
    match data {
        Ok(it) => {
            if at_eof.is_some() {
                tokens.append(&mut it.flatten());
                return Err(CompilerResult::new(
                    Err(StatementRecovery::new(tokens)),
                    error,
                    at_eof,
                ));
            }
            Ok(it)
        }
        Err(err) => {
            return Err(CompilerResult::new(Err(err), error, at_eof));
        }
    }
}
