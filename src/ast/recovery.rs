use crate::lexer::Token;

use super::Spanned;

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
/// |-------------event-decleration-------------|
/// ```
#[derive(Debug)]
pub struct StatementRecovery<'src> {
    pub tokens: Vec<Spanned<Token<'src>>>,
}

impl<'src> StatementRecovery<'src> {
    pub fn new(tokens: Vec<Spanned<Token<'src>>>) -> Self { Self { tokens } }
}

/// Used when there is an error when parsing a top level statement,
/// this is commonly used when a func has an error.
/// It looks for top level declerations like `pevent`
///
/// This is a catch all before more specific error sytntaxes get created.
///
/// ```ts diorite
/// func (msg: text) paction Join end
/// |---recovery---| |--event-decl--|
/// ```
/// This also works with random loose tokens
/// ```lua diorite
/// // ...
/// end
/// 'Hello I am some random text' // Syntax error here and TopLevelRecovery
/// paction Join
/// // ...
/// ```
pub struct TopLevelRecovery<'src> {
    pub tokens: Vec<Spanned<Token<'src>>>,
}

impl<'src> TopLevelRecovery<'src> {
    pub fn new(tokens: Vec<Spanned<Token<'src>>>) -> Self { Self { tokens } }
}

/*
later
func print(message: text 'Text to send to the player')
paction SendMessage (lvar('message'))
end
*/
