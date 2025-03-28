use super::data::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct GeneratedCode<'src> {
    blocks: Vec<CodeBlock<'src>>,
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
pub struct Block<'src> {
    id: &'static str,
    block: &'static str,
    args: Arguments<'src>,
    action: &'src str,
}

#[derive(Serialize)]
pub struct Arguments<'src> {
    items: Vec<Item<'src>>,
}

#[derive(Serialize)]
pub struct Item<'src> {
    item: ChestValue<'src>,
    slot: u8,
}

