pub mod ast;
pub mod lexer;
pub mod parser;

use logos::Logos;

fn main() {
    lexer::Token::lexer("");
}
