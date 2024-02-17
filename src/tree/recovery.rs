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
pub struct StatementRecovery;

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
pub struct TopLevelRecovery;
