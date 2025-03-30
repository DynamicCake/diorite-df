use enum_assoc::Assoc;
use lasso::Spur;

use crate::{
    common::prelude::*,
    dump::{Action, Choice, Tag},
};

#[derive(Debug, PartialEq, Assoc)]
#[func(pub const fn severe(&self) -> bool { false })]
/// Represents a semantic error duing semantic anaylsis.
///
/// the `severe()` function returns true it is impossible to compile with this error.
/// Do note that that you usually don't want to put errors in the ignore list
pub enum SemanticError<'d> {
    /// Only thing that makes no data mutation sharing across threads impossible
    #[assoc(severe = true)]
    DuplicateLineStarter(DuplicateLineStarter),
    #[assoc(severe = true)]
    NumberTooPrecise(Referenced<Spur>),
    #[assoc(severe = true)]
    NumberOutOfBounds(Referenced<Spur>),

    EventNotFound(ActionNotFoundError<'d>),
    ActionNotFound(ActionNotFoundError<'d>),
    SubactionNotFound(SubActionNotFoundError<'d>),

    TagKeyNotFound(TagKeyNotFoundError<'d>),
    TagValueNotFound(TagValueNotFoundError<'d>),
    GameValueNotFound(ActionNotFoundError<'d>),
    ParticleNotFound(ActionNotFoundError<'d>),
    SoundNotFound(ActionNotFoundError<'d>),
    PotionNotFound(ActionNotFoundError<'d>),
    // Remember: Selector sometimes could be like IsSneaking because of subActionBlocks
    SelectorNotFound(SelectorNotFound),
    #[assoc(severe = true)]
    InvalidExprParam(InvalidParamError),

    /// Great inconvenience is placed apon the developers on the code when it isn't UTF-8
    #[assoc(severe = true)]
    NonUtf8FileName(Spur),
}

pub enum InvalidParamType {}

#[derive(Debug, PartialEq)]
pub enum InvalidParamError {
    UnexpectedType {},
}

impl<'d> SemanticError<'d> {
    pub fn from_num(num: Referenced<Spur>, err: DfNumberParseError) -> SemanticError<'d> {
        match err {
            DfNumberParseError::TooBig => SemanticError::NumberOutOfBounds(num),
            DfNumberParseError::TooPercise => SemanticError::NumberTooPrecise(num),
            DfNumberParseError::UnexpectedChar => panic!("Unexpected character {:#?}", num),
            DfNumberParseError::EmptyInput => panic!("Empty input {:#?}", num),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ActionReference {
    pub block: BlockType,
    pub name: Spur,
}

#[derive(Debug, PartialEq)]
pub struct SubActionReference {
    pub blocks: Vec<BlockType>,
    pub name: Spur,
}

impl SubActionReference {
    pub fn new(blocks: Vec<BlockType>, name: Spur) -> Self {
        Self { blocks, name }
    }
}

impl ActionReference {
    pub fn new(block: BlockType, name: Spur) -> Self {
        Self { block, name }
    }
}

#[derive(Debug, PartialEq)]
pub struct TagKeyNotFoundError<'d> {
    pub action: &'d Action,
    pub token: Referenced<Spur>,
    pub suggestions: Vec<&'d Tag>,
}

#[derive(Debug, PartialEq)]
pub struct TagValueNotFoundError<'d> {
    pub key: &'d Tag,
    pub token: Referenced<Spur>,
    pub suggestions: Vec<&'d Choice>,
}

#[derive(Debug, PartialEq)]
pub struct SubActionNotFoundError<'d> {
    pub token: Referenced<SubActionReference>,
    pub suggestions: Vec<&'d Action>,
}

#[derive(Debug, PartialEq)]
pub struct ActionNotFoundError<'d> {
    pub token: Referenced<ActionReference>,
    pub suggestions: Vec<&'d Action>,
}

#[derive(Debug, PartialEq)]
pub struct DuplicateLineStarter {
    pub original: Referenced<()>,
    pub doppelganger: Referenced<()>,
}

#[derive(Debug, PartialEq)]
pub struct SelectorNotFound {
    pub selector: Referenced<Spur>,
}
