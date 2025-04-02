//! Basically tree.rs but for the analyzer

use crate::{
    ast::prelude::*,
    codegen::data,
    dump::{Action, Choice, Tag},
};
use std::sync::Arc;

use lasso::{RodeoResolver, Spur};

#[derive(Debug, PartialEq)]
pub struct AstSelection {
    pub open: Spanned<()>,
    pub selection: Option<Spanned<ActionSelector>>,
    pub close: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct AstTags {
    pub open: Spanned<()>,
    pub tags: MaybeSpan<Parameters<AstIdenPair>>,
    pub close: Spanned<()>,
}

impl AstTags {
    pub fn new(
        open: Spanned<()>,
        tags: MaybeSpan<Parameters<AstIdenPair>>,
        close: Spanned<()>,
    ) -> Self {
        Self { open, tags, close }
    }
}

#[derive(Debug, PartialEq)]
pub struct AstStatements {
    pub items: Vec<AstStatement>,
}

impl AstStatements {
    pub fn new(items: Vec<AstStatement>) -> Self {
        Self { items }
    }
}

#[derive(Debug, PartialEq)]
pub enum AstStatement {
    Simple(Spanned<AstSimpleStatement>),
    If(Spanned<AstIfStatement>),
    Repeat(Spanned<AstRepeatLoop>),
}

#[derive(Debug, PartialEq)]
pub struct AstSimpleStatement {
    pub type_tok: Spanned<ActionType>,
    pub action: Spanned<Iden>,
    pub resolved: Option<Arc<Action>>,
    pub selection: Option<Spanned<AstSelection>>,
    pub tags: Option<Spanned<AstTags>>,
    pub params: Spanned<Wrapped<AstExpression>>,
}

#[derive(Debug, PartialEq)]
pub struct AstIfStatement {
    pub type_tok: Spanned<IfActionType>,
    pub not: Option<Spanned<()>>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<AstSelection>>,
    pub tags: Option<Spanned<AstTags>>,
    pub params: Spanned<Wrapped<AstExpression>>,
    pub statements: AstStatements,
    pub else_block: Option<AstElseBlock>,
    pub end: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct AstElseBlock {
    pub else_tok: Spanned<()>,
    pub statements: AstStatements,
}

#[derive(Debug, PartialEq)]
pub struct AstRepeatLoop {
    pub type_tok: Spanned<()>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<AstSelection>>,
    pub tags: Option<Spanned<AstTags>>,
    pub params: Spanned<Wrapped<AstExpression>>,
    pub statements: AstStatements,
    pub end: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct AstIdenPair {
    pub key: Spanned<Spur>,
    pub colon: Spanned<()>,
    pub value: Spanned<Spur>,
    pub tag: Arc<Tag>,
    pub choice: Arc<Choice>,
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
    String(AstString),
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
pub struct AstString {
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
    pub x: DfNumber,
    pub y: DfNumber,
    pub z: DfNumber,
    pub pitch: DfNumber,
    pub yaw: DfNumber,
}

impl ChestLocationData {
    pub fn resolve(&self) -> data::ChestLocationData {
        data::ChestLocationData {
            x: self.x,
            y: self.y,
            z: self.z,
            pitch: self.pitch,
            yaw: self.yaw,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AstSound {
    pub sound: Spur,
    pub pitch: DfNumber,
    pub vol: DfNumber,
}

#[derive(Debug, PartialEq)]
pub struct AstVariable {
    pub name: Spanned<Spur>,
    pub scope: VariableScope,
}

#[derive(Debug, PartialEq)]
pub struct AstPotion {
    pub pot: Spur,
    pub dur: DfNumber,
    pub amp: DfNumber,
}

#[derive(Debug, PartialEq)]
pub struct AstGameValue {
    pub kind: Spur,
    pub target: GValSelector,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AstParticle {
    pub particle: Spur,
    pub cluster: ParticleCluster,
    // ParticleData is very big and inflates the Data enum by around 2 times
    // Some allocation can't really hurt
    pub data: Box<ParticleData>,
}

// This could be smaller if this was a union but eh
#[derive(Debug, PartialEq, Clone)]
pub struct ParticleData {
    pub x: Option<DfNumber>,
    pub y: Option<DfNumber>,
    pub z: Option<DfNumber>,
    pub size: Option<DfNumber>,
    pub size_variation: Option<DfNumber>,
    pub color: Option<Color>,
    pub color_variation: Option<DfNumber>,
    pub roll: Option<DfNumber>,
    pub motion_variation: Option<DfNumber>,
    pub material: Option<Spur>,
}

impl ParticleData {
    pub fn resolve<'src>(&self, resolver: &'src RodeoResolver) -> data::ParticleData<'src> {
        data::ParticleData {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
            size: self.size.clone(),
            size_variation: self.size_variation.clone(),
            color: self.color.clone(),
            color_variation: self.color_variation.clone(),
            roll: self.roll.clone(),
            motion_variation: self.motion_variation.clone(),
            material: self.material.map(|mat| resolver.resolve(&mat)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParticleCluster {
    pub horizontal: DfNumber,
    pub verticle: DfNumber,
    pub amount: DfNumber,
}

impl ParticleCluster {
    pub fn resolve(&self) -> data::ParticleCluster {
        data::ParticleCluster {
            horizontal: self.horizontal,
            verticle: self.verticle,
            amount: self.amount,
        }
    }
}
