use std::sync::Arc;
use crate::{lexer::Token, ast::prelude::*};

use lasso::{Spur, ThreadedRodeo};
use serde::Serialize;
use span::Span;


#[derive(Debug, PartialEq)]
pub struct AstSelection {
    pub open: Spanned<()>,
    pub selection: Option<Spanned<Spur>>,
    pub close: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct AstTags {
    pub open: Spanned<()>,
    pub tags: MaybeSpan<Parameters<AstIdenPair>>,
    pub close: Spanned<()>,
}

impl AstTags {
    pub fn new(
        open: Spanned<()>,
        tags: MaybeSpan<Parameters<AstIdenPair>>,
        close: Spanned<()>,
    ) -> Self {
        Self { open, tags, close }
    }
}

#[derive(Debug, PartialEq)]
pub struct AstStatements {
    pub items: Vec<AstStatement>,
}

impl AstStatements {
    pub fn new(items: Vec<AstStatement>) -> Self {
        Self { items }
    }
}

#[derive(Debug, PartialEq)]
pub enum AstStatement {
    Simple(Spanned<AstSimpleStatement>),
    If(Spanned<AstIfStatement>),
    Repeat(Spanned<AstRepeatLoop>),
}

#[derive(Debug, PartialEq)]
pub struct AstSimpleStatement {
    pub type_tok: Spanned<ActionType>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<AstSelection>>,
    pub tags: Option<Spanned<AstTags>>,
    pub params: Spanned<Wrapped<AstExpression>>,
}

#[derive(Debug, PartialEq)]
pub struct AstIfStatement {
    pub type_tok: Spanned<IfActionType>,
    pub not: Option<Spanned<()>>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<AstSelection>>,
    pub tags: Option<Spanned<AstTags>>,
    pub params: Spanned<Wrapped<AstExpression>>,
    pub statements: AstStatements,
    pub else_block: Option<AstElseBlock>,
    pub end: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct AstElseBlock {
    pub else_tok: Spanned<()>,
    pub statements: AstStatements,
}

#[derive(Debug, PartialEq)]
pub struct AstRepeatLoop {
    pub type_tok: Spanned<()>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<AstSelection>>,
    pub tags: Option<Spanned<AstTags>>,
    pub params: Spanned<Wrapped<AstExpression>>,
    pub statements: AstStatements,
    pub end: Spanned<()>,
}


#[derive(Debug, PartialEq)]
pub struct AstIdenPair {
    pub key: Spanned<Spur>,
    pub colon: Spanned<()>,
    pub value: Spanned<Spur>,
}

#[derive(Debug, PartialEq)]
pub struct AstExpression {

    // This needs to be done differently
}

