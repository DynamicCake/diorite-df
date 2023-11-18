use crate::lexer::Token;


#[test]
fn print_test() {
    let tok = Token::Iden("ur mom").expected_print();
    println!("{}", tok);
}