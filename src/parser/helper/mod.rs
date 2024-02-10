use crate::{
    ast::{recovery::StatementRecovery, Spanned},
    lexer::Token,
};

use super::{AdvanceUnexpected, CompilerResult, Parser, UnexpectedToken};

pub fn recover_statement<'src, T>(
    parser: &mut Parser<'src>,
    err: AdvanceUnexpected<'src>,
) -> CompilerResult<'src, Result<T, StatementRecovery>> {
    match err {
        AdvanceUnexpected::Token(err) => {
            let at_eof = parser.statement_recovery();
            CompilerResult::new(Err(StatementRecovery), vec![err], at_eof)
        }
        AdvanceUnexpected::Eof(err) => {
            CompilerResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)))
        }
    }
}

pub fn handle_result_statement<'src, T, E>(
    CompilerResult {
        data,
        error,
        at_eof,
    }: CompilerResult<'src, Result<T, StatementRecovery>>,
) -> Result<T, CompilerResult<'src, Result<E, StatementRecovery>>> {
    match data {
        Ok(it) => {
            if at_eof.is_some() {
                return Err(CompilerResult::new(Err(StatementRecovery), error, at_eof));
            }
            Ok(it)
        }
        Err(err) => {
            return Err(CompilerResult::new(Err(err), error, at_eof));
        }
    }
}
