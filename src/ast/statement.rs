use super::top::*;
use super::*;


pub struct Selection<'src> {
    open: Spanned<()>,
    selection: Option<Spanned<&'src str>>,
    close: Spanned<()>,
}

pub struct Tags<'src> {
    open: Spanned<()>,
    tags: Option<Spanned<Parameters<IdenPair<'src>>>>,
    close: Spanned<()>,
}

pub struct Statements<'src> {
    items: Vec<Statement<'src>>,
}

pub enum Statement<'src> {
    Standard(StandardStatement<'src>),
    If(IfStatement<'src>),
}

pub struct IfStatement<'src> {
    type_tok: Spanned<IfActionType>,
    not: Option<Spanned<()>>,
    action: ActionType,
    selection: Option<Spanned<Selection<'src>>>,
    tags: Option<Spanned<Tags<'src>>>,
    params: Spanned<Parameters<Expression<'src>>>,
}

pub enum IfActionType {
    Player,
    Entity,
    Game,
    Var,
}

pub struct StandardStatement<'src> {
    type_tok: Spanned<ActionType>,
    action: ActionType,
    selection: Option<Spanned<Selection<'src>>>,
    tags: Option<Spanned<Tags<'src>>>,
    params: Spanned<Parameters<Expression<'src>>>,
}

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

pub struct IdenPair<'src> {
    key: Spanned<&'src str>,
    colon: Spanned<()>,
    value: Spanned<&'src str>,
}
pub enum Expression<'src> {
    Static(StaticLiteral<'src>),
    Literal(ExprLiteral<'src>),
}

pub struct ExprLiteral<'src> {
    literal_type: Spanned<ExprLitType>,
    args: Spanned<Parameters<StaticLiteral<'src>>>,
}

pub enum StaticLiteral<'src> {
    String(StringLiteral<'src>),
    Number(NumberLiteral<'src>),
}

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
    GaveValue,
}