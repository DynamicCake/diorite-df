use crate::{tree::recovery::{StatementRecovery, TopLevelRecovery}, error::syntax::ParseResult};

use super::{AdvanceUnexpected, Parser};

pub fn recover_statement<'lex, T>(
    parser: &mut Parser<'lex>,
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

pub fn recover_top_level<'lex, T>(
    parser: &mut Parser<'lex>,
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

pub fn handle_result_statement<'src, T, E>(
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
            return Err(ParseResult::new(Err(err), error, at_eof));
        }
    }
}

pub fn should_return_func<'src, T, R>(
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
        Err(err) => {
            return Err(ParseResult::new(Err(err), error, at_eof));
        }
    };

    Ok(lit)
}

pub fn should_return_top_func<'src, T, R>(
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
