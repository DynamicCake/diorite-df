use crate::lexer::Token;


#[test]
fn print_test() {
    let tok = Token::Iden("ur mom");
    let tok2 = Token::Iden("ur dad");
    println!("{:?}", (tok == tok2));
}
