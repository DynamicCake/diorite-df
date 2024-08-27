//! Provides useful structs to be used in parse tree and ast

use std::collections::HashSet;

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

#[derive(Debug)]
pub struct Starter(pub Referenced<Spur>);

impl std::hash::Hash for Starter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.spanned.data.hash(state);
    }
}

impl Starter {
    pub fn new(value: Referenced<Spur>) -> Self {
        Self(value)
    }
}

impl PartialEq for Starter {
    fn eq(&self, other: &Self) -> bool {
        self.0.spanned.data == other.0.spanned.data
    }
}


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
