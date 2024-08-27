use crate::ast::prelude::*;

#[derive(Debug, PartialEq)]
pub enum AstTopLevel {
    Event(AstEvent),
    FuncDef(AstFuncDef),
    ProcDef(AstProcDef),
}

// Function
#[derive(Debug, PartialEq)]
pub struct AstFuncDef {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden>,
    pub params: Wrapped<AstFuncParamDef>,
    pub statements: AstStatements,
    pub end_tok: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct AstFuncParamDef {
    pub name: Spanned<Iden>,
    pub colon: Spanned<()>,
    pub data_type: Spanned<Iden>,
    pub description: Option<Spanned<Iden>>,
}

// Process
#[derive(Debug, PartialEq)]
pub struct AstProcDef {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden>,
    pub statements: AstStatements,
    pub end_tok: Spanned<()>,
}

// Event
#[derive(Debug, PartialEq)]
pub struct AstEvent {
    pub type_tok: Spanned<EventType>,
    pub name: Spanned<Iden>,
    pub statements: AstStatements,
    pub end_tok: Spanned<()>,
}

impl AstEvent {
    pub fn new(
        type_tok: Spanned<EventType>,
        name: Spanned<Iden>,
        statements: AstStatements,
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

