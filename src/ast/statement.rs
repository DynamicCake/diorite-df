use std::{marker::PhantomData, sync::Arc};
use crate::{ast::prelude::*, dump::{Action, Choice, Tag}, lexer::Token};

use lasso::{Spur, ThreadedRodeo};
use serde::Serialize;
use span::Span;


#[derive(Debug, PartialEq)]
pub struct AstSelection<'d> {
    pub open: Spanned<()>,
    pub selection: Option<Spanned<Spur>>,
    pub sub_action: Option<&'d Action>,
    pub close: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct AstTags<'d> {
    pub open: Spanned<()>,
    pub tags: MaybeSpan<Parameters<AstIdenPair<'d>>>,
    pub close: Spanned<()>,
}

impl<'d> AstTags<'d> {
    pub fn new(
        open: Spanned<()>,
        tags: MaybeSpan<Parameters<AstIdenPair<'d>>>,
        close: Spanned<()>,
    ) -> Self {
        Self { open, tags, close }
    }
}

#[derive(Debug, PartialEq)]
pub struct AstStatements<'d> {
    pub items: Vec<AstStatement<'d>>,
}

impl<'d> AstStatements<'d> {
    pub fn new(items: Vec<AstStatement<'d>>) -> Self {
        Self { items }
    }
}

#[derive(Debug, PartialEq)]
pub enum AstStatement<'d> {
    Simple(Spanned<AstSimpleStatement<'d>>),
    If(Spanned<AstIfStatement<'d>>),
    Repeat(Spanned<AstRepeatLoop<'d>>),
}

#[derive(Debug, PartialEq)]
pub struct AstSimpleStatement<'d> {
    pub type_tok: Spanned<ActionType>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<AstSelection<'d>>>,
    pub tags: Option<Spanned<AstTags<'d>>>,
    pub params: Spanned<Wrapped<AstExpression<'d>>>,
}

#[derive(Debug, PartialEq)]
pub struct AstIfStatement<'d> {
    pub type_tok: Spanned<IfActionType>,
    pub not: Option<Spanned<()>>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<AstSelection<'d>>>,
    pub tags: Option<Spanned<AstTags<'d>>>,
    pub params: Spanned<Wrapped<AstExpression<'d>>>,
    pub statements: AstStatements<'d>,
    pub else_block: Option<AstElseBlock<'d>>,
    pub end: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct AstElseBlock<'d> {
    pub else_tok: Spanned<()>,
    pub statements: AstStatements<'d>,
}

#[derive(Debug, PartialEq)]
pub struct AstRepeatLoop<'d> {
    pub type_tok: Spanned<()>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<AstSelection<'d>>>,
    pub tags: Option<Spanned<AstTags<'d>>>,
    pub params: Spanned<Wrapped<AstExpression<'d>>>,
    pub statements: AstStatements<'d>,
    pub end: Spanned<()>,
}


#[derive(Debug, PartialEq)]
pub struct AstIdenPair<'d> {
    pub key: Spanned<Spur>,
    pub colon: Spanned<()>,
    pub value: Spanned<Spur>,
    pub tag: &'d Tag,
    pub choice: &'d Choice
}

#[derive(Debug, PartialEq)]
pub struct AstExpression<'d> {
    _ph: &'d ()

    // This needs to be done differently
}

