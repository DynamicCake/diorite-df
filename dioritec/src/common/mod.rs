//! Provides useful structs to be used in parse tree and ast

use std::{collections::HashSet, sync::Arc};

use data::Iden;
use lasso::Spur;
use serde::Serialize;
use span::Referenced;

use crate::dump::Action;

pub mod ast;
pub mod data;
pub mod span;
mod test;
pub mod tree;

pub mod prelude {
    pub use crate::common::ast::*;
    pub use crate::common::data::*;
    pub use crate::common::span::*;
    pub use crate::common::tree::*;
    pub use crate::common::*;
    pub use crate::lexer::*;
}

#[derive(Debug, Clone)]
pub struct Starter(pub Referenced<Iden>);

impl std::hash::Hash for Starter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.spanned.data.inner.hash(state);
    }
}

impl Starter {
    pub fn new(value: Referenced<Iden>) -> Self {
        Self(value)
    }
}

impl PartialEq for Starter {
    fn eq(&self, other: &Self) -> bool {
        self.0.spanned.data.inner == other.0.spanned.data.inner
    }
}

impl Eq for Starter {}

#[derive(Debug, PartialEq)]
pub struct StarterSet {
    pub player_event: HashSet<Starter>,
    pub entity_event: HashSet<Starter>,
    pub function: HashSet<Starter>,
    pub process: HashSet<Starter>,
}

impl Default for StarterSet {
    fn default() -> Self {
        Self::new()
    }
}

impl StarterSet {
    pub fn new() -> Self {
        Self {
            player_event: HashSet::new(),
            entity_event: HashSet::new(),
            function: HashSet::new(),
            process: HashSet::new(),
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct Color(u16);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Self(
            (r as u16 & 0b1111_1111)
                | ((g as u16 & 0b1111_1111) << 5)
                | ((b as u16 & 0b1111_1111) << 10),
        )
    }
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VariableScope {
    Line,
    Local,
    #[serde(rename = "unsaved")]
    Game,
    #[serde(rename = "saved")]
    Global,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum GValSelector {
    Selection,
    Default,
    Killer,
    Damager,
    Victim,
    Shooter,
    Projectile,
    LastEntity,
}

#[derive(Debug, PartialEq)]
pub enum ActionSelector {
    Selection,
    Default,
    Killer,
    Damager,
    Shooter,
    Victim,
    AllPlayers,
    Projectile,
    AllEntities,
    AllMobs,
    LastEntity,
    Other(Option<Arc<Action>>),
}

impl ActionSelector {
    /// WARNING: This function can never return ActionSelector::Other(Some(_))
    /// It is advised to check if ActionSelector::Other could find an action
    pub fn basic_from_str(value: &str) -> ActionSelector {
        match value {
            "selection" => ActionSelector::Selection,
            "default" => ActionSelector::Default,
            "killer" => ActionSelector::Killer,
            "damager" => ActionSelector::Damager,
            "shooter" => ActionSelector::Shooter,
            "victim" => ActionSelector::Victim,
            "allplayers" => ActionSelector::AllPlayers,
            "projectile" => ActionSelector::Projectile,
            "allentities" => ActionSelector::AllEntities,
            "allmobs" => ActionSelector::AllMobs,
            "lastentity" => ActionSelector::LastEntity,
            _ => ActionSelector::Other(None),
        }
    }
}
