use crate::ast::recovery::StatementRecovery;

use super::{AdvanceUnexpected, ParseResult, Parser};

pub fn recover_statement<'src, T>(
    parser: &mut Parser<'src>,
    err: AdvanceUnexpected<'src>,
) -> ParseResult<'src, Result<T, StatementRecovery>> {
    match err {
        AdvanceUnexpected::Token(err) => {
            let at_eof = parser.statement_recovery();
            ParseResult::new(Err(StatementRecovery), vec![err], at_eof)
        }
        AdvanceUnexpected::Eof(err) => {
            ParseResult::new(Err(StatementRecovery), Vec::new(), Some(Box::new(err)))
        }
    }
}

pub fn handle_result_statement<'src, T, E>(
    ParseResult {
        data,
        error,
        at_eof,
    }: ParseResult<'src, Result<T, StatementRecovery>>,
) -> Result<T, ParseResult<'src, Result<E, StatementRecovery>>> {
    match data {
        Ok(it) => {
            if at_eof.is_some() {
                return Err(ParseResult::new(Err(StatementRecovery), error, at_eof));
            }
            Ok(it)
        }
        Err(err) => {
            return Err(ParseResult::new(Err(err), error, at_eof));
        }
    }
}
