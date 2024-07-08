use enum_assoc::Assoc;
use lasso::Spur;

use crate::common::span::{Span, Spanned};

#[derive(Assoc)]
#[func(pub const fn severe(&self) -> bool { false })]
/// Represents a semantic error duing semantic anaylsis (No duh)
/// the `severe()` function returns true if the program cannot compile with the error
pub enum SemanitcError {
    ActionNotFound(MissingInDumpError),
    TagNotFound(MissingInDumpError),
    GameValueNotFound(MissingInDumpError),
    ParticleNotFound(MissingInDumpError),
    SoundNotFound(MissingInDumpError),
    PotionNotFound(MissingInDumpError),
    #[assoc(severe = true)]
    SelectorNotFound(MissingInDumpError),
    // action, tag, gamevalue, particle, sound, poition, selector
}

pub struct MissingInDumpError {
    token: Spanned<Spur>,
}
