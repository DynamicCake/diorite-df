use std::sync::Arc;

use crate::{
    lexer::Token,
    parser::error::{ExpectedTokens, UnexpectedToken},
};

#[test]
fn print_test() {
    use crate::lexer::Token;
    use lasso::ThreadedRodeo;

    let rodeo = ThreadedRodeo::new();
    let tok = Token::Iden(Some(rodeo.get_or_intern("ur mom")));
    let tok2 = Token::Iden(Some(rodeo.get_or_intern("ur dad")));
    println!("{:?}", (tok == tok2));
}

#[test]
fn iden_quotes() {
    use lasso::ThreadedRodeo;
    use logos::Logos;
    use std::sync::Arc;

    use crate::lexer::Token;

    let rodeo = Arc::new(ThreadedRodeo::new());
    const SRC: &str = "'hello'";
    let mut lexer = Token::lexer(SRC);
    let left = lexer.next().unwrap().unwrap();
    let right = Token::Iden(Some(rodeo.get_or_intern("hello")));
    assert_eq!(left, right);
}

#[test]
fn unexpected() {
    let a = UnexpectedToken::new(
        ExpectedTokens::new(Arc::new([Token::Iden(None)])),
        Token::Colon.spanned(1..3),
        Some("les go".into()),
    );
    println!("{}", a.expected_print());
}
