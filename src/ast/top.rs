use super::statement::*;
use super::*;

pub enum TopLevel<'src> {
    Event(Event<'src>),
    FuncDef(FuncDef<'src>),
    ProcDef(ProcDef<'src>),
}

// Function
pub struct FuncDef<'src> {
    type_tok: Spanned<()>,
    name: Spanned<Iden<'src>>,
    params: Parameters<FuncParamDef<'src>>,
    end_tok: Spanned<()>,
}

pub struct FuncParamDef<'src> {
    name: Spanned<&'src str>,
    colon: Spanned<()>,
    data_type: Spanned<&'src str>,
    description: Spanned<&'src str>,
}

// Process
pub struct ProcDef<'src> {
    type_tok: Spanned<()>,
    name: Spanned<Iden<'src>>,
    end_tok: Spanned<()>,
}

// Event
pub struct Event<'src> {
    type_tok: Spanned<EventType>,
    name: Spanned<Iden<'src>>,
    statements: Spanned<Statements<'src>>,
    end_tok: Spanned<()>,
}

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
