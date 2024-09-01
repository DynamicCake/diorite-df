use crate::{
    ast::prelude::*,
    dump::{Action, Choice, Tag},
    lexer::Token,
};
use std::{marker::PhantomData, sync::Arc};

use lasso::{Spur, ThreadedRodeo};
use serde::Serialize;
use span::Span;

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
    Variable { data: Variable },
    Potion { data: Potion },
    GameValue { data: GameValue },
    Particle { data: Particle },
    Sound { data: Sound },
    Location { data: Location },
    Vector { data: Vec3D },
    Text { data: Text },
    StyledText { data: StyledText },
    BlockTag { data: BlockTag },
}

#[derive(Debug, PartialEq)]
pub struct BlockTag {
    option: Spur,
    tag: Spur,
    action: Spur,
    block: BlockType,
}

#[derive(Debug, PartialEq)]
pub struct StyledText {
    name: Spur,
}

#[derive(Debug, PartialEq)]
pub struct Text {
    name: Spur,
}

#[derive(Debug, PartialEq)]
pub struct Vec3D {
    x: DfNumber,
    y: DfNumber,
    z: DfNumber,
}

#[derive(Debug, PartialEq)]
pub struct Location {
    is_block: bool,
    loc: LocationData,
}

#[derive(Debug, PartialEq)]
pub struct LocationData {
    x: DfNumber,
    y: DfNumber,
    z: DfNumber,
    pitch: DfNumber,
    yaw: DfNumber,
}

#[derive(Debug, PartialEq)]
pub struct Sound {
    sound: Spur,
    pitch: DfNumber,
    vol: DfNumber,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: Spur,
    pub scope: VariableScope,
}

#[derive(Debug, PartialEq)]
pub struct Potion {
    pub pot: Spur,
    pub dur: u8,
    pub amp: u8,
}

#[derive(Debug, PartialEq)]
pub struct GameValue {
    kind: Spur,
        target: GValSelector
}

#[derive(Debug, PartialEq)]
pub struct Particle {
    particle: Spur,
    cluster: ParticleCluster,
    // ParticleData is very big and inflates the Data enum by around 2 times
    // Some allocation can't really hurt
    data: Box<ParticleData>,
}

// This could be smaller if this was a union but eh
#[derive(Debug, PartialEq)]
pub struct ParticleData {
    x: Option<DfNumber>,
    y: Option<DfNumber>,
    z: Option<DfNumber>,
    size: Option<DfNumber>,
    size_variation: Option<u8>,
    color: Option<Color>,
    color_variation: Option<u8>,
    roll: Option<DfNumber>,
    motion_variation: Option<u8>,
    material: Option<Spur>,
}

#[derive(Debug, PartialEq)]
pub struct ParticleCluster {
    horizontal: DfNumber,
    verticle: DfNumber,
    amount: u16,
}
