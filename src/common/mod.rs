//! Provides useful structs to be used in parse tree and ast

use std::collections::HashSet;

use data::Iden;
use lasso::Spur;
use span::Referenced;

pub mod data;
pub mod span;
pub mod tree;

pub mod prelude {
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
        self.0.spanned.data.name.hash(state);
    }
}

impl Starter {
    pub fn new(value: Referenced<Iden>) -> Self {
        Self(value)
    }
}

impl PartialEq for Starter {
    fn eq(&self, other: &Self) -> bool {
        self.0.spanned.data.name == other.0.spanned.data.name
    }
}

impl Eq for Starter {}

#[derive(Debug)]
pub struct StarterSet {
    pub player_event: HashSet<Starter>,
    pub entity_event: HashSet<Starter>,
    pub function: HashSet<Starter>,
    pub process: HashSet<Starter>,
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
