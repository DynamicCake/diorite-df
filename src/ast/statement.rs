use logos::Span;

use crate::lexer::Token;

use super::recovery::StatementRecovery;
use super::top::*;
use super::*;

#[derive(Debug)]
pub struct Selection<'src> {
    open: Spanned<()>,
    selection: Option<Spanned<&'src str>>,
    close: Spanned<()>,
}

#[derive(Debug)]
pub struct Tags<'src> {
    open: Spanned<()>,
    tags: Option<Spanned<Parameters<'src, IdenPair<'src>>>>,
    close: Spanned<()>,
}

#[derive(Debug)]
pub struct Statements<'src> {
    pub items: Vec<Spanned<Statement<'src>>>,
}

impl<'src> Statements<'src> {
    pub fn new(items: Vec<Spanned<Statement<'src>>>) -> Self {
        Self { items }
    }
}

#[derive(Debug)]
pub enum Statement<'src> {
    Simple(SimpleStatement<'src>),
    If(IfStatement<'src>),
    Recovery(StatementRecovery<'src>),
}

impl<'src> Statement<'src> {
    pub fn calc_span(&self) -> Option<Span> {
        match self {
            Statement::Simple(stmt) => Some(stmt.calc_span()),
            Statement::If(stmt) => Some(stmt.calc_span()),
            Statement::Recovery(stmt) => stmt.calc_span(),
        }
    }
}

#[derive(Debug)]
pub struct SimpleStatement<'src> {
    pub type_tok: Spanned<ActionType>,
    pub action: Spanned<Iden<'src>>,
    pub selection: Option<Spanned<Selection<'src>>>,
    pub tags: Option<Spanned<Tags<'src>>>,
    pub params: Spanned<Parameters<'src, Expression<'src>>>,
}

impl<'src> SimpleStatement<'src> {
    pub fn calc_span(&self) -> Span {
        let start = self.type_tok.span.start;
        let end = self.params.span.end;
        start..end
    }
}

#[derive(Debug)]
pub struct IfStatement<'src> {
    type_tok: Spanned<IfActionType>,
    not: Option<Spanned<()>>,
    action: ActionType,
    selection: Option<Spanned<Selection<'src>>>,
    tags: Option<Spanned<Tags<'src>>>,
    params: Spanned<Parameters<'src, Expression<'src>>>,
}

impl<'src> IfStatement<'src> {
    pub fn calc_span(&self) -> Span {
        let start = self.type_tok.span.start;
        let end = self.params.span.end;
        start..end
    }
}

#[derive(Debug)]
pub enum IfActionType {
    Player,
    Entity,
    Game,
    Var,
}

#[derive(Debug)]
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

impl<'src> ActionType {
    pub fn from_token(token: Token<'src>) -> Result<Self, Token<'src>> {
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
pub struct IdenPair<'src> {
    pub key: Spanned<&'src str>,
    pub colon: Spanned<()>,
    pub value: Spanned<&'src str>,
}

#[derive(Debug)]
pub enum Expression<'src> {
    Static(StaticLiteral<'src>),
    Literal(ExprLiteral<'src>),
}

#[derive(Debug)]
pub struct ExprLiteral<'src> {
    pub literal_type: Spanned<ExprLitType>,
    pub args: Spanned<Parameters<'src, StaticLiteral<'src>>>,
}

#[derive(Debug)]
pub enum StaticLiteral<'src> {
    String(StringLiteral<'src>),
    Number(NumberLiteral<'src>),
}

#[derive(Debug)]
pub enum ExprLitType {
    SaveVar,
    GlobalVar,
    ThreadVar,
    LineVar,
    Location,
    Vector,
    Sound,
    Particle,
    Potion,
    GameValue,
}
