use crate::common::prelude::*;
use ast::{BlockType, DfNumber};
use lasso::{RodeoReader, RodeoResolver};
use serde::Serialize;


#[derive(Serialize, PartialEq)]
#[serde(tag = "id")]
pub enum ChestValue<'src> {
    #[serde(rename = "var")]
    Variable { data: ChestVariable<'src> },
    #[serde(rename = "pot")]
    Potion { data: ChestPotion<'src> },
    #[serde(rename = "g_val")]
    GameValue { data: ChestGameValue<'src> },
    #[serde(rename = "part")]
    Particle { data: ChestParticle<'src> },
    #[serde(rename = "snd")]
    Sound { data: ChestSound<'src> },
    #[serde(rename = "loc")]
    Location { data: ChestLocation },
    #[serde(rename = "vec")]
    Vector { data: ChestVec3D },
    #[serde(rename = "num")]
    Number { data: ChestNumber },
    #[serde(rename = "hint")]
    FunctionParam { data: ChestFunctionParam<'src> },
    // TODO: StyledText? String?
    #[serde(rename = "comp")]
    StyledText { data: ChestStyledText<'src> },
    #[serde(rename = "txt")]
    String { data: ChestString<'src> },
    #[serde(rename = "bl_tag")]
    BlockTag { data: ChestBlockTag<'src> },
}
// TODO: Figure out params
#[derive(Serialize, PartialEq)]
pub struct ChestFunctionParam<'src> {
    pub name: &'src str,
    #[serde(rename = "type")]
    pub typ: &'src str,
    pub plural: bool,
    pub optional: bool,
    pub description: Option<&'src str>,
    // note is not supported lol
}

#[derive(Serialize, PartialEq)]
pub struct ChestBlockTag<'src> {
    pub option: &'src str,
    pub tag: &'src str,
    pub action: &'src str,
    pub block: BlockType,
}

#[derive(Serialize, PartialEq)]
pub struct ChestStyledText<'src> {
    pub name: &'src str,
}

#[derive(Serialize, PartialEq)]
pub struct ChestString<'src> {
    pub name: &'src str,
}

#[derive(Serialize, PartialEq)]
pub struct ChestNumber {
    pub name: DfNumber,
}

#[derive(Serialize, PartialEq)]
pub struct ChestVec3D {
    pub x: DfNumber,
    pub y: DfNumber,
    pub z: DfNumber,
}

#[derive(Serialize, PartialEq)]
#[serde(rename = "camelCase")]
pub struct ChestLocation {
    pub is_block: bool,
    pub loc: ChestLocationData,
}

#[derive(Serialize, PartialEq)]
#[serde(rename = "camelCase")]
pub struct ChestLocationData {
    pub x: DfNumber,
    pub y: DfNumber,
    pub z: DfNumber,
    pub pitch: DfNumber,
    pub yaw: DfNumber,
}

#[derive(Serialize, PartialEq)]
pub struct ChestSound<'src> {
    pub sound: &'src str,
    pub pitch: DfNumber,
    pub vol: DfNumber,
}

#[derive(Serialize, PartialEq)]
pub struct ChestVariable<'src> {
    pub name: &'src str,
    pub scope: VariableScope,
}

#[derive(Serialize, PartialEq)]
pub struct ChestPotion<'src> {
    pub pot: &'src str,
    pub dur: DfNumber,
    pub amp: DfNumber,
}

#[derive(Serialize, PartialEq)]
pub struct ChestGameValue<'src> {
    #[serde(rename = "type")]
    pub kind: &'src str,
    pub target: GValSelector,
}

#[derive(Serialize, PartialEq)]
pub struct ChestParticle<'src> {
    pub particle: &'src str,
    pub cluster: ParticleCluster,
    // ParticleData is very big and inflates the Data enum by around 2 times
    // Some allocation can't really hurt
    pub data: Box<ParticleData<'src>>,
}

// This could be smaller if this was a union but eh
#[derive(Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParticleData<'src> {
    pub x: Option<DfNumber>,
    pub y: Option<DfNumber>,
    pub z: Option<DfNumber>,
    pub size: Option<DfNumber>,
    pub size_variation: Option<DfNumber>,
    pub color: Option<Color>,
    pub color_variation: Option<DfNumber>,
    pub roll: Option<DfNumber>,
    pub motion_variation: Option<DfNumber>,
    pub material: Option<&'src str>,
}

#[derive(Serialize, PartialEq)]
pub struct ParticleCluster {
    pub horizontal: DfNumber,
    pub verticle: DfNumber,
    pub amount: DfNumber,
}

