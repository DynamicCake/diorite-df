use super::*;
// use super::error::*;

impl<'src> Parser<'src> {
    pub(super) fn top_level(&mut self) -> CompilerResult<'src, Option<TopLevel<'src>>> {
        let expected = ExpectedTokens::new(vec![
            Token::FuncDef,
            Token::ProcDef,
            Token::PlayerEvent,
            Token::EntityEvent,
        ]);



        todo!()
    }
}
