use crate::lexer::Token;


#[test]
fn print_test() {
    let tok = Token::Iden(Some("ur mom"));
    let tok2 = Token::Iden(Some("ur dad"));
    println!("{:?}", (tok == tok2));
}
