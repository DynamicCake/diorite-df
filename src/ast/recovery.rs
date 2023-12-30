use logos::Span;

use crate::lexer::Token;

use super::{Spanned, statement::Statement};

#[derive(Debug)]
pub enum Recovery<'src> {
    Statement(StatementRecovery<'src>),
    TopLevel(TopLevelRecovery<'src>),
}

/// Used when a statement is malformed and continues until
/// it finds another statement decleration like `paction` or `end`
///
/// Examples:
///
/// Looking for another start
/// ```diorite
/// var Some malformed text paction SendMessage ("Hello diorite!")
/// |-------recovery------| |-------------statement--------------|
/// ```
/// Looking for an end
/// ```diorite
/// pevent Join var some more nonsense lalala end
///             |----------recovery---------|
/// |-------------event-declaration-------------|
/// ```
#[derive(Debug)]
pub struct StatementRecovery<'src> {
    pub tokens: Vec<Spanned<Token<'src>>>,
}
impl<'src> StatementRecovery<'src> {
    pub fn new(tokens: Vec<Spanned<Token<'src>>>) -> Self {
        Self { tokens }
    }
    pub fn empty() -> Self {
        Self { tokens: Vec::new() }
    }
    pub fn calc_span(&self) -> Option<Span> {
        let toks = &self.tokens;
        if toks.is_empty() {
            None
        } else {
            let start = toks
                .first()
                .expect("Non empty array must have a first")
                .span
                .start;
            let end = toks
                .last()
                .expect("Non empty array must have a last")
                .span
                .end;
            Some(start..end)
        }
    }
}

/// Used when there is an error when parsing a top level statement,
/// this is commonly used when a func has an error.
/// It looks for top level declarations like `pevent`
///
/// This is a catch all before more specific error syntaxes get created.
///
/// ```diorite
/// func (msg: text) paction Join end
/// |---recovery---| |--event-decl--|
/// ```
/// This also works with random loose tokens
/// ```diorite
/// // ...
/// end
/// 'Hello I am some random text' // Syntax error here and TopLevelRecovery
/// paction Join
/// // ...
/// ```
#[derive(Debug)]
pub struct TopLevelRecovery<'src> {
    pub items: Vec<TopRecoveryType<'src>>,
}

impl<'src> TopLevelRecovery<'src> {
    pub fn new(items: Vec<TopRecoveryType<'src>>) -> Self {
        Self { items }
    }
}

#[derive(Debug)]
pub enum TopRecoveryType<'src> {
    Body(Vec<Spanned<Statement<'src>>>),
    Unrecognizable(Vec<Spanned<Token<'src>>>)
}

/*
later
func print(message: text 'Text to send to the player')
paction SendMessage (lvar('message'))
end
*/
