use super::recovery::TopLevelRecovery;
use super::statement::*;
use super::*;

#[derive(Debug)]
pub enum TopLevel<'src> {
    Event(Event<'src>),
    FuncDef(FuncDef<'src>),
    ProcDef(ProcDef<'src>),
    Recovery(TopLevelRecovery<'src>),
}

// Function
#[derive(Debug)]
pub struct FuncDef<'src> {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden<'src>>,
    pub params: Parameters<'src, FuncParamDef<'src>>,
    pub end_tok: Spanned<()>,
}

#[derive(Debug)]
pub struct FuncParamDef<'src> {
    pub name: Spanned<&'src str>,
    pub colon: Spanned<()>,
    pub data_type: Spanned<&'src str>,
    pub description: Spanned<&'src str>,
}

// Process
#[derive(Debug)]
pub struct ProcDef<'src> {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden<'src>>,
    pub end_tok: Spanned<()>,
}

// Event
#[derive(Debug)]
pub struct Event<'src> {
    pub type_tok: Spanned<EventType>,
    pub name: Spanned<Iden<'src>>,
    pub statements: Spanned<Statements<'src>>,
    pub end_tok: Spanned<()>,
}

impl<'src> Event<'src> {
    pub fn new(
        type_tok: Spanned<EventType>,
        name: Spanned<Iden<'src>>,
        statements: Spanned<Statements<'src>>,
        end_tok: Spanned<()>,
    ) -> Self {
        Self {
            type_tok,
            name,
            statements,
            end_tok,
        }
    }
}

#[derive(Debug)]
pub enum EventType {
    Player,
    Entity,
}

impl EventType {
    fn from(string: &str) -> Option<Self> {
        match string {
            "pevent" => Some(Self::Player),
            "eevent" => Some(Self::Player),
            _ => None,
        }
    }
}

