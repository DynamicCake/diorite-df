use ast::{BlockType, DfNumber};
use serde::Serialize;
use crate::common::prelude::*;

#[derive(Serialize, PartialEq)]
#[serde(tag = "id")]
pub enum ChestValue<'src> {
    #[serde(rename = "var")]
    Variable { data: Variable<'src> },
    #[serde(rename = "pot")]
    Potion { data: Potion<'src> },
    #[serde(rename = "g_val")]
    GameValue { data: GameValue<'src> },
    #[serde(rename = "part")]
    Particle { data: Particle<'src> },
    #[serde(rename = "snd")]
    Sound { data: Sound<'src> },
    #[serde(rename = "loc")]
    Location { data: Location },
    #[serde(rename = "vec")]
    Vector { data: Vec3D },
    #[serde(rename = "txt")]
    Text { data: Text<'src> },
    #[serde(rename = "part")]
    StyledText { data: StyledText<'src> },
    #[serde(rename = "bl_tag")]
    BlockTag { data: BlockTag<'src> },
}

#[derive(Serialize, PartialEq)]
pub struct BlockTag<'src> {
    option: &'src str,
    tag: &'src str,
    action: &'src str,
    block: BlockType,
}

#[derive(Serialize, PartialEq)]
pub struct StyledText<'src> {
    name: &'src str,
}

#[derive(Serialize, PartialEq)]
pub struct Text<'src> {
    name: &'src str,
}

#[derive(Serialize, PartialEq)]
pub struct Vec3D {
    x: DfNumber,
    y: DfNumber,
    z: DfNumber,
}

#[derive(Serialize, PartialEq)]
#[serde(rename = "camelCase")]
pub struct Location {
    is_block: bool,
    loc: LocationData,
}

#[derive(Serialize, PartialEq)]
#[serde(rename = "camelCase")]
pub struct LocationData {
    x: DfNumber,
    y: DfNumber,
    z: DfNumber,
    pitch: DfNumber,
    yaw: DfNumber,
}

#[derive(Serialize, PartialEq)]
pub struct Sound<'src> {
    sound: &'src str,
    pitch: DfNumber,
    vol: DfNumber,
}

#[derive(Serialize, PartialEq)]
pub struct Variable<'src> {
    pub name: &'src str,
    pub scope: VariableScope,
}

#[derive(Serialize, PartialEq)]
pub struct Potion<'src> {
    pub pot: &'src str,
    pub dur: u8,
    pub amp: u8,
}

#[derive(Serialize, PartialEq)]
pub struct GameValue<'src> {
    #[serde(rename = "type")]
    kind: &'src str,
    target: GValSelector,
}

#[derive(Serialize, PartialEq)]
pub struct Particle<'src> {
    particle: &'src str,
    cluster: ParticleCluster,
    // ParticleData is very big and inflates the Data enum by around 2 times
    // Some allocation can't really hurt
    data: Box<ParticleData<'src>>,
}

// This could be smaller if this was a union but eh
#[derive(Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParticleData<'src> {
    x: Option<DfNumber>,
    y: Option<DfNumber>,
    z: Option<DfNumber>,
    size: Option<DfNumber>,
    size_variation: Option<u8>,
    color: Option<Color>,
    color_variation: Option<u8>,
    roll: Option<DfNumber>,
    motion_variation: Option<u8>,
    material: Option<&'src str>,
}

#[derive(Serialize, PartialEq)]
pub struct ParticleCluster {
    horizontal: DfNumber,
    verticle: DfNumber,
    amount: u16,
}

