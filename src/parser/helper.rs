use crate::{
    error::syntax::ParseResult,
    tree::recovery::{StatementRecovery, TopLevelRecovery},
};

use super::{AdvanceUnexpected, Parser};

/// [Parser::statement_recovery] that takes into account end of line errors
pub fn recover_statement<T>(
    parser: &mut Parser<'_>,
    err: AdvanceUnexpected,
) -> ParseResult<Result<T, StatementRecovery>> {
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
/// [recover_statement] but for the top level instead
pub fn recover_top_level<T>(
    parser: &mut Parser<'_>,
    err: AdvanceUnexpected,
) -> ParseResult<Result<T, TopLevelRecovery>> {
    match err {
        AdvanceUnexpected::Token(err) => {
            let at_eof = parser.top_recovery();
            ParseResult::new(Err(TopLevelRecovery), vec![err], at_eof)
        }
        AdvanceUnexpected::Eof(err) => {
            ParseResult::new(Err(TopLevelRecovery), Vec::new(), Some(Box::new(err)))
        }
    }
}

#[deprecated]
pub fn handle_result_statement<T, E>(
    ParseResult {
        data,
        error,
        at_eof,
    }: ParseResult<Result<T, StatementRecovery>>,
) -> Result<T, ParseResult<Result<E, StatementRecovery>>> {
    match data {
        Ok(it) => {
            if at_eof.is_some() {
                return Err(ParseResult::new(Err(StatementRecovery), error, at_eof));
            }
            Ok(it)
        }
        Err(err) => {
            Err(ParseResult::new(Err(err), error, at_eof))
        }
    }
}

/// Puts ParseResults without errors into `Ok(T)` and the ones with errors into `Err(E)`
pub fn should_return_func<T, R>(
    result: ParseResult<Result<T, StatementRecovery>>,
) -> Result<T, ParseResult<Result<R, StatementRecovery>>> {
    let ParseResult {
        data,
        error,
        at_eof,
    } = result;
    let lit = match data {
        Ok(it) => {
            if at_eof.is_some() {
                return Err(ParseResult::new(Err(StatementRecovery), error, at_eof));
            }
            it
        }
        Err(_err) => {
            return Err(ParseResult::new(Err(StatementRecovery), error, at_eof));
        }
    };

    Ok(lit)
}

/// [should_return_func] but for top level
pub fn should_return_top_func<T, R>(
    result: ParseResult<Result<T, TopLevelRecovery>>,
) -> Result<T, ParseResult<Result<R, TopLevelRecovery>>> {
    let ParseResult {
        data,
        error,
        at_eof,
    } = result;
    let lit = match data {
        Ok(it) => {
            if at_eof.is_some() {
                return Err(ParseResult::new(Err(TopLevelRecovery), error, at_eof));
            }
            it
        }
        Err(err) => {
            return Err(ParseResult::new(Err(err), error, at_eof));
        }
    };

    Ok(lit)
}
