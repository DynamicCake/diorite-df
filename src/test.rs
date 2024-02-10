use logos::Logos;

use crate::lexer::{self, Token};


#[test]
fn print_test() {
    let tok = Token::Iden(Some("ur mom"));
    let tok2 = Token::Iden(Some("ur dad"));
    println!("{:?}", (tok == tok2));
}

#[test]
fn iden_quotes() {
    const SRC: &str = "'hello'";
    let mut lexer = lexer::Token::lexer(SRC);
    let left = lexer.next().unwrap().unwrap();
    let right = Token::Iden(Some("hello"));
    assert_eq!(left, right);
}
