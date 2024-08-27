use crate::tree::prelude::*;

#[derive(Debug, PartialEq)]
pub enum TreeTopLevel {
    Event(TreeEvent),
    FuncDef(TreeFuncDef),
    ProcDef(TreeProcDef),
    Recovery(TopLevelRecovery),
}

// Function
#[derive(Debug, PartialEq)]
pub struct TreeFuncDef {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden>,
    pub params: Wrapped<TreeFuncParamDef>,
    pub statements: TreeStatements,
    pub end_tok: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct TreeFuncParamDef {
    pub name: Spanned<Iden>,
    pub colon: Spanned<()>,
    pub data_type: Spanned<Iden>,
    pub description: Option<Spanned<Iden>>,
}

impl SpanStart for TreeFuncParamDef {
    fn start(&self) -> SpanSize {
        self.name.span.start
    }
}

impl SpanEnd for TreeFuncParamDef {
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
pub struct TreeProcDef {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden>,
    pub statements: TreeStatements,
    pub end_tok: Spanned<()>,
}

// Event
#[derive(Debug, PartialEq)]
pub struct TreeEvent {
    pub type_tok: Spanned<EventType>,
    pub name: Spanned<Iden>,
    pub statements: TreeStatements,
    pub end_tok: Spanned<()>,
}

impl TreeEvent {
    pub fn new(
        type_tok: Spanned<EventType>,
        name: Spanned<Iden>,
        statements: TreeStatements,
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
