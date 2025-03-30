use semantic::SemanticError;
use syntax::{LexerError, UnexpectedEOF, UnexpectedToken};

pub mod semantic;
pub mod syntax;

#[derive(Debug, PartialEq)]
pub enum CompilerError<'d> {
    Lexer(LexerError),
    Parse(UnexpectedToken),
    Eof(UnexpectedEOF),
    Semantic(SemanticError<'d>)
}


