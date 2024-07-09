use super::prelude::*;

#[derive(Debug, PartialEq)]
pub enum TopLevel {
    Event(Event),
    FuncDef(FuncDef),
    ProcDef(ProcDef),
}

// Function
#[derive(Debug, PartialEq)]
pub struct FuncDef {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden>,
    pub params: Wrapped<FuncParamDef>,
    pub statements: Statements,
    pub end_tok: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct FuncParamDef {
    pub name: Spanned<Iden>,
    pub colon: Spanned<()>,
    pub data_type: Spanned<Iden>,
    pub description: Option<Spanned<Iden>>,
}

impl SpanStart for FuncParamDef {
    fn start(&self) -> SpanSize {
        self.name.span.start
    }
}

impl SpanEnd for FuncParamDef {
    fn end(&self) -> SpanSize {
        let desc = &self.description;
        match desc {
            Some(it) => it.span.end,
            None => self.data_type.span.end,
        }
    }
}

// Process
#[derive(Debug, PartialEq)]
pub struct ProcDef {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden>,
    pub statements: Statements,
    pub end_tok: Spanned<()>,
}

// Event
#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum EventType {
    Player,
    Entity,
}
