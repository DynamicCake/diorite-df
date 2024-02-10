#[test]
fn print_test() {
    use crate::lexer::Token;
    let tok = Token::Iden(Some("ur mom"));
    let tok2 = Token::Iden(Some("ur dad"));
    println!("{:?}", (tok == tok2));
}

#[test]
fn iden_quotes() {
    use logos::Logos;

    use crate::lexer::Token;

    const SRC: &str = "'hello'";
    let mut lexer = Token::lexer(SRC);
    let left = lexer.next().unwrap().unwrap();
    let right = Token::Iden(Some("hello"));
    assert_eq!(left, right);
}
