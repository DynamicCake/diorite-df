use enum_assoc::Assoc;
use lasso::Spur;

use crate::common::prelude::*;

#[derive(Assoc)]
#[func(pub const fn severe(&self) -> bool { false })]
/// Represents a semantic error duing semantic anaylsis.
///
/// the `severe()` function returns true it is impossible to compile with this error.
/// Do note that that you usually don't want to put errors in the ignore list
pub enum SemanticError {
    /// Only thing that makes no data mutation sharing across threads impossible
    #[assoc(severe = true)]
    DuplicateLineStarter(DuplicateLineStarter),
    #[assoc(severe = true)]
    NumberTooPrecise(),
    #[assoc(severe = true)]
    NumberOutOfBounds(),

    ActionNotFound(MissingInDumpError),
    TagNotFound(MissingInDumpError),
    GameValueNotFound(MissingInDumpError),
    ParticleNotFound(MissingInDumpError),
    SoundNotFound(MissingInDumpError),
    PotionNotFound(MissingInDumpError),
    /// Remember: Selector sometimes could be like IsSneaking because of subActionBlocks
    SelectorNotFound(SelectorNotFound),

    /// Great inconvenience is placed apon the developers on the code when it isn't UTF-8
    #[assoc(severe = true)]
    NonUtf8FileName(Spur),
}

pub struct MissingInDumpError {
    pub token: Referenced<Spur>,
    pub suggestions: Vec<Box<str>>,
}

pub struct DuplicateLineStarter {
    pub original: Reference,
    pub doppelganger: Reference,
}

pub struct SelectorNotFound {
    pub offener: Referenced<Spur>,
}
