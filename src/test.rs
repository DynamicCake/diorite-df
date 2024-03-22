

#[test]
fn print_tes() {
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
    use crate::error::syntax::{UnexpectedToken, ExpectedTokens};
    use crate::lexer::Token;
    use std::sync::Arc;

    UnexpectedToken::new(
        ExpectedTokens::new(Arc::new([Token::Iden(None)])),
        Token::Colon.spanned(1..3),
        Some("les go".into()),
        "test".into()
    ).expected_print();

}

