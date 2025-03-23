use ast::{BlockType, DfNumber};
use serde::Serialize;
use crate::common::prelude::*;

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
    #[serde(rename = "txt")]
    Text { data: ChestText<'src> },
    #[serde(rename = "num")]
    Number { data: ChestNumber },
    #[serde(rename = "part")]
    StyledText { data: ChestStyledText<'src> },
    #[serde(rename = "bl_tag")]
    BlockTag { data: ChestBlockTag<'src> },
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
pub struct ChestText<'src> {
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
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Serialize, PartialEq)]
pub struct ChestSound<'src> {
    pub sound: &'src str,
    pub pitch: f32,
    pub vol: f32,
}

#[derive(Serialize, PartialEq)]
pub struct ChestVariable<'src> {
    pub name: &'src str,
    pub scope: VariableScope,
}

#[derive(Serialize, PartialEq)]
pub struct ChestPotion<'src> {
    pub pot: &'src str,
    pub dur: u8,
    pub amp: u8,
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
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
    pub size: Option<f32>,
    pub size_variation: Option<u8>,
    pub color: Option<Color>,
    pub color_variation: Option<u8>,
    pub roll: Option<f32>,
    pub motion_variation: Option<u8>,
    pub material: Option<&'src str>,
}

#[derive(Serialize, PartialEq)]
pub struct ParticleCluster {
    pub horizontal: f32,
    pub verticle: f32,
    pub amount: u16,
}

