use std::sync::Arc;

use lasso::ThreadedRodeo;
use logos::Span;
use serde::Serialize;

use crate::{lexer::Token, span::{CalcSpan, MaybeSpan, SpanEnd, SpanStart, Spanned, TryCalcSpan, TrySpanEnd, TrySpanStart}};

use super::*;

#[derive(Debug)]
pub struct Selection {
    pub open: Spanned<()>,
    pub selection: Option<Spanned<Spur>>,
    pub close: Spanned<()>,
}

impl CalcSpan for Selection {
    fn calculate_span(&self) -> Span {
        self.open.span.start..self.close.span.end
    }
}

#[derive(Debug)]
pub struct Tags {
    pub open: Spanned<()>,
    pub tags: MaybeSpan<Parameters<IdenPair>>,
    pub close: Spanned<()>,
}

impl Tags {
    pub fn new(
        open: Spanned<()>,
        tags: MaybeSpan<Parameters<IdenPair>>,
        close: Spanned<()>,
    ) -> Self {
        Self { open, tags, close }
    }
}

impl CalcSpan for Tags {
    fn calculate_span(&self) -> Span {
        self.open.span.start..self.close.span.end
    }
}

#[derive(Debug)]
pub struct Statements {
    pub items: Vec<Statement>,
}

impl Statements {
    pub fn new(items: Vec<Statement>) -> Self {
        Self { items }
    }
}

impl TryCalcSpan for Statements {
    // TODO test this function
    fn try_calculate_span(&self) -> Option<Span> {
        let mut iter = self.items.iter().peekable();

        let start = loop {
            let stmt = iter.peek();
            if let Some(stmt) = stmt {
                match stmt {
                    Statement::Simple(it) => break it.span.start,
                    Statement::If(it) => break it.span.start,
                    Statement::Recovery => iter.next(),
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
                    Statement::Simple(it) => break it.span.end,
                    Statement::If(it) => break it.span.end,
                    Statement::Recovery => iter.next(),
                }
            } else {
                return None;
            };
        };

        Some(start..end)
    }
}

#[derive(Debug)]
pub enum Statement {
    Simple(Spanned<SimpleStatement>),
    If(Spanned<IfStatement>),
    Recovery,
}

#[derive(Debug)]
pub struct SimpleStatement {
    pub type_tok: Spanned<ActionType>,
    pub action: Spanned<Iden>,
    pub selection: Option<Spanned<Selection>>,
    pub tags: Option<Spanned<Tags>>,
    pub params: Spanned<Wrapped<Expression>>,
}

impl SimpleStatement {
    pub fn calc_span(&self) -> Span {
        let start = self.type_tok.span.start;
        let end = self.params.span.end;
        start..end
    }
}

#[derive(Debug)]
pub struct IfStatement {
    type_tok: Spanned<IfActionType>,
    not: Option<Spanned<()>>,
    action: ActionType,
    selection: Option<Spanned<Selection>>,
    tags: Option<Spanned<Tags>>,
    params: Spanned<Parameters<Expression>>,
}

impl IfStatement {
    pub fn calc_span(&self) -> Span {
        self.type_tok.span.start..self.params.span.end
    }
}

#[derive(Debug)]
pub enum IfActionType {
    Player,
    Entity,
    Game,
    Var,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    PlayerAction,
    EntityAction,
    GameAction,
    Control,
    CallFunction,
    CallProcess,
    Select,
    Var,
}


impl ActionType {
    pub fn from_token(token: Token) -> Result<Self, Token> {
        match token {
            Token::PlayerAction => Ok(Self::PlayerAction),
            Token::EntityAction => Ok(Self::EntityAction),
            Token::GameAction => Ok(Self::GameAction),
            Token::Control => Ok(Self::Control),
            Token::CallFunction => Ok(Self::CallFunction),
            Token::CallProcess => Ok(Self::CallProcess),
            Token::Select => Ok(Self::Select),
            Token::SetVar => Ok(Self::Var),
            tok => Err(tok),
        }
    }
}

#[derive(Debug)]
pub struct IdenPair {
    pub key: Spanned<Spur>,
    pub colon: Spanned<()>,
    pub value: Spanned<Spur>,
}

impl SpanStart for IdenPair {
    fn start(&self) -> usize {
        self.key.span.start
    }
}

impl SpanEnd for IdenPair {
    fn end(&self) -> usize {
        self.value.span.end
    }
}

impl CalcSpan for IdenPair {
    fn calculate_span(&self) -> Span {
        self.start()..self.end()
    }
}

#[derive(Debug)]
pub struct Wrapped<T> {
    pub open: Spanned<()>,
    pub tags: MaybeSpan<Parameters<T>>,
    pub close: Spanned<()>,
}

impl<T> Wrapped<T> {
    pub fn new(open: Spanned<()>, tags: MaybeSpan<Parameters<T>>, close: Spanned<()>) -> Self {
        Self { open, tags, close }
    }
}

impl<T> CalcSpan for Wrapped<T> {
    fn calculate_span(&self) -> Span {
        self.open.span.start..self.close.span.end
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(StaticLiteral),
    Expr(ExprLiteral),
}

impl SpanStart for Expression {
    fn start(&self) -> usize {
        let range = match self {
            Self::Expr(lit) => lit.literal_type.span.start,
            Self::Literal(lit) => match lit {
                StaticLiteral::String(lit) => lit.span.start,
                StaticLiteral::Number(lit) => lit.span.start,
            }
            .clone(),
        };
        range
    }
}

impl SpanEnd for Expression {
    fn end(&self) -> usize {
        let range = match self {
            Self::Expr(lit) => lit.literal_type.span.end,
            Self::Literal(lit) => match lit {
                StaticLiteral::String(lit) => lit.span.end,
                StaticLiteral::Number(lit) => lit.span.end,
            }
            .clone(),
        };
        range
    }
}

#[derive(Debug)]
pub struct ExprLiteral {
    pub literal_type: Spanned<ExprLitType>,
    pub args: Spanned<Wrapped<StaticLiteral>>,
}

impl ExprLiteral {
    pub fn new(literal_type: Spanned<ExprLitType>, args: Spanned<Wrapped<StaticLiteral>>) -> Self {
        Self { literal_type, args }
    }
}

#[derive(Debug)]
pub enum StaticLiteral {
    String(Spanned<StringLiteral>),
    Number(Spanned<NumberLiteral>),
}

impl SpanStart for StaticLiteral {
    fn start(&self) -> usize {
        match self {
            StaticLiteral::String(it) => it.span.start,
            StaticLiteral::Number(it) => it.span.start,
        }
    }
}

impl SpanEnd for StaticLiteral {
    fn end(&self) -> usize {
        match self {
            StaticLiteral::String(it) => it.span.end,
            StaticLiteral::Number(it) => it.span.end,
        }
    }
}

impl TrySpanStart for StaticLiteral {
    fn try_start(&self) -> Option<usize> {
        Some(self.start())
    }
}

impl TrySpanEnd for StaticLiteral {
    fn try_end(&self) -> Option<usize> {
        Some(self.end())
    }
}

#[derive(Debug)]
pub enum ExprLitType {
    Unknown(Spur),
    SaveVar,
    GlobalVar,
    LocalVar,
    LineVar,
    Location,
    Vector,
    Sound,
    Particle,
    Potion,
    GameValue,
}

impl ExprLitType {
    pub fn from_spur(value: &Spur, rodeo: Arc<ThreadedRodeo>) -> Self {
        let res = rodeo.resolve(value);
        match res {
            "svar" => Self::SaveVar,
            "gvar" => Self::GlobalVar,
            "lvar" => Self::LocalVar,
            "var" => Self::LineVar,
            "loc" => Self::Location,
            "vec" => Self::Vector,
            "sound" => Self::Sound,
            "part" => Self::Particle,
            "pot" => Self::Potion,
            "gval" => Self::GameValue,
            _ => Self::Unknown(value.clone()),
        }
    }
}
