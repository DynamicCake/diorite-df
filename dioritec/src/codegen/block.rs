use super::data::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct GeneratedCode<'src> {
    pub blocks: Vec<CodeBlock<'src>>,
}

#[derive(Serialize)]
#[serde(tag = "id", rename_all = "snake_case")]
pub enum CodeBlock<'src> {
    Block(Block<'src>),
    Bracket(Bracket<'src>),
}

#[derive(Serialize)]
pub struct Bracket<'src> {
    direct: &'src str,
    #[serde(rename = "type")]
    kind: BracketKind,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BracketKind {
    Norm,
    Repeat,
}

#[derive(Serialize)]
#[serde(rename = "snake_case")]
pub enum BracketState {
    Open,
    Close,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub struct Block<'src> {
    pub block: &'static str,
    pub args: Arguments<'src>,
    pub action: &'src str,
}

#[derive(Serialize)]
pub struct Arguments<'src> {
    pub items: Vec<Item<'src>>,
}

#[derive(Serialize)]
pub struct Item<'src> {
    pub item: ChestValue<'src>,
    pub slot: u8,
}
