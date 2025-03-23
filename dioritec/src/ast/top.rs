use crate::{ast::prelude::*, dump::Action};

#[derive(Debug, PartialEq)]
pub enum AstTopLevel<'d> {
    Event(AstEvent<'d>),
    FuncDef(AstFuncDef<'d>),
    ProcDef(AstProcDef<'d>),
}

// Function
#[derive(Debug, PartialEq)]
pub struct AstFuncDef<'d> {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden>,
    pub params: Wrapped<AstFuncParamDef>,
    pub statements: AstStatements<'d>,
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
pub struct AstProcDef<'d> {
    pub type_tok: Spanned<()>,
    pub name: Spanned<Iden>,
    pub statements: AstStatements<'d>,
    pub end_tok: Spanned<()>,
}

// Event
#[derive(Debug, PartialEq)]
pub struct AstEvent<'d> {
    pub type_tok: Spanned<EventType>,
    pub name: Spanned<Iden>,
    pub statements: AstStatements<'d>,
    pub end_tok: Spanned<()>,
    pub action: Option<&'d Action>,
}

impl<'d> AstEvent<'d> {
    pub fn new(
        type_tok: Spanned<EventType>,
        name: Spanned<Iden>,
        statements: AstStatements<'d>,
        end_tok: Spanned<()>,
        action: Option<&'d Action>,
    ) -> Self {
        Self {
            type_tok,
            name,
            statements,
            end_tok,
            action,
        }
    }
}
