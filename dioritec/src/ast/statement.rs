//! Basically tree.rs but for the analyzer

use crate::{
    ast::prelude::*,
    dump::{Action, Choice, Tag},
};

use lasso::Spur;

#[derive(Debug, PartialEq)]
pub struct AstSelection<'d> {
    pub open: Spanned<()>,
    pub selection: Option<Spanned<ActionSelector<'d>>>,
    pub close: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct AstTags<'d> {
    pub open: Spanned<()>,
    pub tags: MaybeSpan<Parameters<AstIdenPair<'d>>>,
    pub close: Spanned<()>,
}

impl<'d> AstTags<'d> {
    pub fn new(
        open: Spanned<()>,
        tags: MaybeSpan<Parameters<AstIdenPair<'d>>>,
        close: Spanned<()>,
    ) -> Self {
        Self { open, tags, close }
    }
}

#[derive(Debug, PartialEq)]
pub struct AstStatements<'d> {
    pub items: Vec<AstStatement<'d>>,
}

impl<'d> AstStatements<'d> {
    pub fn new(items: Vec<AstStatement<'d>>) -> Self {
        Self { items }
    }
}

#[derive(Debug, PartialEq)]
pub enum AstStatement<'d> {
    Simple(Spanned<AstSimpleStatement<'d>>),
    If(Spanned<AstIfStatement<'d>>),
    Repeat(Spanned<AstRepeatLoop<'d>>),
}

#[derive(Debug, PartialEq)]
pub struct AstSimpleStatement<'d> {
    pub type_tok: Spanned<ActionType>,
    pub action: Spanned<Iden>,
    pub resolved: Option<&'d Action>,
    pub selection: Option<Spanned<AstSelection<'d>>>,
    pub tags: Option<Spanned<AstTags<'d>>>,
    pub params: Spanned<Wrapped<AstExpression>>,
}

#[derive(Debug, PartialEq)]
pub struct AstIfStatement<'d> {
    pub type_tok: Spanned<IfActionType>,
    pub not: Option<Spanned<()>>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<AstSelection<'d>>>,
    pub tags: Option<Spanned<AstTags<'d>>>,
    pub params: Spanned<Wrapped<AstExpression>>,
    pub statements: AstStatements<'d>,
    pub else_block: Option<AstElseBlock<'d>>,
    pub end: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct AstElseBlock<'d> {
    pub else_tok: Spanned<()>,
    pub statements: AstStatements<'d>,
}

#[derive(Debug, PartialEq)]
pub struct AstRepeatLoop<'d> {
    pub type_tok: Spanned<()>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<AstSelection<'d>>>,
    pub tags: Option<Spanned<AstTags<'d>>>,
    pub params: Spanned<Wrapped<AstExpression>>,
    pub statements: AstStatements<'d>,
    pub end: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct AstIdenPair<'d> {
    pub key: Spanned<Spur>,
    pub colon: Spanned<()>,
    pub value: Spanned<Spur>,
    pub tag: &'d Tag,
    pub choice: &'d Choice,
}
#[derive(Debug, PartialEq)]
pub enum AstExpression {
    Variable(AstVariable),
    Potion(AstPotion),
    GameValue(AstGameValue),
    Particle(AstParticle),
    Sound(AstSound),
    Location(AstLocation),
    Vector(AstVec3D),
    Text(AstText),
    Number(AstNumber),
    StyledText(AstStyledText),
    BlockTag(AstBlockTag),
}

#[derive(Debug, PartialEq)]
pub struct AstBlockTag {
    pub option: Spur,
    pub tag: Spur,
    pub action: Spur,
    pub block: BlockType,
}

#[derive(Debug, PartialEq)]
pub struct AstStyledText {
    pub name: Spur,
}

#[derive(Debug, PartialEq)]
pub struct AstText {
    pub name: Spur,
}

#[derive(Debug, PartialEq)]
pub struct AstNumber {
    pub name: DfNumber,
}

#[derive(Debug, PartialEq)]
pub struct AstVec3D {
    pub x: DfNumber,
    pub y: DfNumber,
    pub z: DfNumber,
}

#[derive(Debug, PartialEq)]
pub struct AstLocation {
    pub is_block: bool,
    pub loc: ChestLocationData,
}

#[derive(Debug, PartialEq)]
pub struct ChestLocationData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Debug, PartialEq)]
pub struct AstSound {
    pub sound: Spur,
    pub pitch: f32,
    pub vol: f32,
}

#[derive(Debug, PartialEq)]
pub struct AstVariable {
    pub name: Spanned<Spur>,
    pub scope: VariableScope,
}

#[derive(Debug, PartialEq)]
pub struct AstPotion {
    pub pot: Spur,
    pub dur: u8,
    pub amp: u8,
}

#[derive(Debug, PartialEq)]
pub struct AstGameValue {
    pub kind: Spur,
    pub target: GValSelector,
}

#[derive(Debug, PartialEq)]
pub struct AstParticle {
    pub particle: Spur,
    pub cluster: ParticleCluster,
    // ParticleData is very big and inflates the Data enum by around 2 times
    // Some allocation can't really hurt
    pub data: Box<ParticleData>,
}

// This could be smaller if this was a union but eh
#[derive(Debug, PartialEq)]
pub struct ParticleData {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
    pub size: Option<f32>,
    pub size_variation: Option<u8>,
    pub color: Option<Color>,
    pub color_variation: Option<u8>,
    pub roll: Option<f32>,
    pub motion_variation: Option<u8>,
    pub material: Option<Spur>,
}

#[derive(Debug, PartialEq)]
pub struct ParticleCluster {
    pub horizontal: f32,
    pub verticle: f32,
    pub amount: u16,
}
