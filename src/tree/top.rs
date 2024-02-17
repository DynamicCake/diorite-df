use super::recovery::TopLevelRecovery;
use super::statement::*;
use super::*;

#[derive(Debug)]
pub enum TopLevel {
    Event(Event),
    FuncDef(FuncDef),
    ProcDef(ProcDef),
    Recovery(TopLevelRecovery),
}

// Function
#[derive(Debug)]
pub struct FuncDef {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden>,
    pub params: Parameters<FuncParamDef>,
    pub end_tok: Spanned<()>,
}

#[derive(Debug)]
pub struct FuncParamDef {
    pub name: Spanned<Spur>,
    pub colon: Spanned<()>,
    pub data_type: Spanned<Spur>,
    pub description: Option<Spanned<Spur>>,
}

impl SpanStart for FuncParamDef {
    fn start(&self) -> usize {
        self.name.span.start
    }
}

impl SpanEnd for FuncParamDef {
    fn end(&self) -> usize {
        let desc = &self.description;
        match desc {
            Some(it) => it.span.end,
            None => self.data_type.span.end,
        }
    }
}

// Process
#[derive(Debug)]
pub struct ProcDef {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden>,
    pub end_tok: Spanned<()>,
}

// Event
#[derive(Debug)]
pub struct Event {
    pub type_tok: Spanned<EventType>,
    pub name: Spanned<Iden>,
    pub statements: Statements,
    pub end_tok: Spanned<()>,
}

impl Event {
    pub fn new(
        type_tok: Spanned<EventType>,
        name: Spanned<Iden>,
        statements: Statements,
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

