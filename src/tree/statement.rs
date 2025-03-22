use crate::{lexer::Token, tree::prelude::*};
use std::sync::Arc;

use lasso::{Spur, ThreadedRodeo};
use serde::Serialize;
use span::Span;

#[derive(Debug, PartialEq)]
pub struct TreeSelection {
    pub open: Spanned<()>,
    pub selection: Option<Spanned<Spur>>,
    pub close: Spanned<()>,
}

impl CalcSpan for TreeSelection {
    fn calculate_span(&self) -> Span {
        self.open.span.start..self.close.span.end
    }
}

#[derive(Debug, PartialEq)]
pub struct TreeTags {
    pub open: Spanned<()>,
    pub inner_tags: MaybeSpan<Parameters<TreeIdenPair>>,
    pub close: Spanned<()>,
}

impl TreeTags {
    pub fn new(
        open: Spanned<()>,
        tags: MaybeSpan<Parameters<TreeIdenPair>>,
        close: Spanned<()>,
    ) -> Self {
        Self {
            open,
            inner_tags: tags,
            close,
        }
    }
}

impl CalcSpan for TreeTags {
    fn calculate_span(&self) -> Span {
        self.open.span.start..self.close.span.end
    }
}

#[derive(Debug, PartialEq)]
pub struct TreeStatements {
    pub items: Vec<TreeStatement>,
}

impl TreeStatements {
    pub fn new(items: Vec<TreeStatement>) -> Self {
        Self { items }
    }
}

impl TryCalcSpan for TreeStatements {
    // TODO test this function
    fn try_calculate_span(&self) -> Option<Span> {
        let mut iter = self.items.iter().peekable();

        let start = loop {
            let stmt = iter.peek();
            if let Some(stmt) = stmt {
                match stmt {
                    TreeStatement::Simple(it) => break it.span.start,
                    TreeStatement::If(it) => break it.span.start,
                    TreeStatement::Repeat(it) => break it.span.start,
                    TreeStatement::Recovery => iter.next(),
                }
            } else {
                return None;
            };
        };

        let mut iter = iter.rev().peekable();
        let end = loop {
            let stmt = iter.peek();
            if let Some(stmt) = stmt {
                match stmt {
                    TreeStatement::Simple(it) => break it.span.end,
                    TreeStatement::If(it) => break it.span.end,
                    TreeStatement::Repeat(it) => break it.span.end,
                    TreeStatement::Recovery => iter.next(),
                }
            } else {
                return None;
            };
        };

        Some(start..end)
    }
}

#[derive(Debug, PartialEq)]
pub enum TreeStatement {
    Simple(Spanned<TreeSimpleStatement>),
    If(Spanned<TreeIfStatement>),
    Repeat(Spanned<TreeRepeatLoop>),
    Recovery,
}

#[derive(Debug, PartialEq)]
pub struct TreeSimpleStatement {
    pub type_tok: Spanned<ActionType>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<TreeSelection>>,
    pub tags: Option<Spanned<TreeTags>>,
    pub params: Spanned<Wrapped<TreeExpression>>,
}

impl TreeSimpleStatement {
    pub fn calc_span(&self) -> Span {
        let start = self.type_tok.span.start;
        let end = self.params.span.end;
        start..end
    }
}

#[derive(Debug, PartialEq)]
pub struct TreeIfStatement {
    pub type_tok: Spanned<IfActionType>,
    pub not: Option<Spanned<()>>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<TreeSelection>>,
    pub tags: Option<Spanned<TreeTags>>,
    pub params: Spanned<Wrapped<TreeExpression>>,
    pub statements: TreeStatements,
    pub else_block: Option<TreeElseBlock>,
    pub end: Spanned<()>,
}

#[derive(Debug, PartialEq)]
pub struct TreeElseBlock {
    pub else_tok: Spanned<()>,
    pub statements: TreeStatements,
}

impl CalcSpan for TreeIfStatement {
    fn calculate_span(&self) -> Span {
        self.type_tok.span.start..self.params.span.end
    }
}

#[derive(Debug, PartialEq)]
pub struct TreeRepeatLoop {
    pub type_tok: Spanned<()>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<TreeSelection>>,
    pub tags: Option<Spanned<TreeTags>>,
    pub params: Spanned<Wrapped<TreeExpression>>,
    pub statements: TreeStatements,
    pub end: Spanned<()>,
}

impl CalcSpan for TreeRepeatLoop {
    fn calculate_span(&self) -> Span {
        self.type_tok.span.start..self.params.span.end
    }
}

#[derive(Debug, PartialEq)]
pub struct TreeIdenPair {
    pub key: Spanned<Spur>,
    pub colon: Spanned<()>,
    pub value: Spanned<Spur>,
}

impl SpanStart for TreeIdenPair {
    fn start(&self) -> SpanSize {
        self.key.span.start
    }
}

impl SpanEnd for TreeIdenPair {
    fn end(&self) -> SpanSize {
        self.value.span.end
    }
}

impl CalcSpan for TreeIdenPair {
    fn calculate_span(&self) -> Span {
        self.start()..self.end()
    }
}

#[derive(Debug, PartialEq)]
pub enum TreeExpression {
    Literal(TreeStaticLiteral),
    Expr(TreeExprLiteral),
}

impl SpanStart for TreeExpression {
    fn start(&self) -> SpanSize {
        match self {
            Self::Expr(lit) => lit.literal_type.span.start,
            Self::Literal(lit) => match lit {
                TreeStaticLiteral::String(lit) => lit.span.start,
                TreeStaticLiteral::Number(lit) => lit.span.start,
            }
        }
    }
}

impl SpanEnd for TreeExpression {
    fn end(&self) -> SpanSize {
        match self {
            Self::Expr(lit) => lit.literal_type.span.end,
            Self::Literal(lit) => match lit {
                TreeStaticLiteral::String(lit) => lit.span.end,
                TreeStaticLiteral::Number(lit) => lit.span.end,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TreeExprLiteral {
    pub literal_type: Spanned<Iden>,
    pub args: Spanned<Wrapped<TreeExprValue>>,
}

#[derive(Debug, PartialEq)]
pub enum TreeExprValue {
    Iden(Spanned<Iden>),
    Number(Spanned<NumberLiteral>),
}

impl TreeExprLiteral {
    pub fn new(literal_type: Spanned<Iden>, args: Spanned<Wrapped<TreeExprValue>>) -> Self {
        Self { literal_type, args }
    }
}

impl SpanStart for TreeExprValue {
    fn start(&self) -> SpanSize {
        match self {
            TreeExprValue::Iden(it) => it.span.start,
            TreeExprValue::Number(it) => it.span.start,
        }
    }
}

impl SpanEnd for TreeExprValue {
    fn end(&self) -> SpanSize {
        match self {
            TreeExprValue::Iden(it) => it.span.end,
            TreeExprValue::Number(it) => it.span.end,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TreeStaticLiteral {
    String(Spanned<StringLiteral>),
    Number(Spanned<NumberLiteral>),
}

impl SpanStart for TreeStaticLiteral {
    fn start(&self) -> SpanSize {
        match self {
            TreeStaticLiteral::String(it) => it.span.start,
            TreeStaticLiteral::Number(it) => it.span.start,
        }
    }
}

impl SpanEnd for TreeStaticLiteral {
    fn end(&self) -> SpanSize {
        match self {
            TreeStaticLiteral::String(it) => it.span.end,
            TreeStaticLiteral::Number(it) => it.span.end,
        }
    }
}

impl TrySpanStart for TreeStaticLiteral {
    fn try_start(&self) -> Option<SpanSize> {
        Some(self.start())
    }
}

impl TrySpanEnd for TreeStaticLiteral {
    fn try_end(&self) -> Option<SpanSize> {
        Some(self.end())
    }
}
