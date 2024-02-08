use crate::{
    ast::{recovery::StatementRecovery, Spanned},
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
