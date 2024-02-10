use logos::Span;

use crate::lexer::Token;



use super::*;

#[derive(Debug)]
pub struct Selection<'src> {
    pub open: Spanned<()>,
    pub selection: Option<Spanned<&'src str>>,
    pub close: Spanned<()>,
}

impl<'src> CalcSpan for Selection<'src> {
    fn calculate_span(&self) -> super::Span {
        self.open.span.start..self.close.span.end
    }
}

#[derive(Debug)]
pub struct Tags<'src> {
    pub open: Spanned<()>,
    pub tags: Option<MaybeSpan<Parameters<IdenPair<'src>>>>,
    pub close: Spanned<()>,
}

impl<'src> CalcSpan for Tags<'src> {
    fn calculate_span(&self) -> super::Span {
        self.open.span.start..self.close.span.end
    }
}

#[derive(Debug)]
pub struct Statements<'src> {
    pub items: Vec<Statement<'src>>,
}

impl<'src> Statements<'src> {
    pub fn new(items: Vec<Statement<'src>>) -> Self {
        Self { items }
    }
}

impl<'src> TryCalcSpan for Statements<'src> {
    // TODO test this function
    fn try_calculate_span(&self) -> Option<Span> {
        let mut iter = self.items.iter().peekable();

        let start = loop {
            let stmt = iter.peek();
            if let Some(stmt) = stmt {
                match stmt {
                    Statement::Simple(it) => break it.span.start,
                    Statement::If(it) => break it.span.start,
                    Statement::Recovery => {
                        iter.next()
                    }
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
                    Statement::Recovery => {
                        iter.next()
                    }
                }
            } else {
                return None;
            };

        };

        Some(start..end)
    }
}

#[derive(Debug)]
pub enum Statement<'src> {
    Simple(Spanned<SimpleStatement<'src>>),
    If(Spanned<IfStatement<'src>>),
    Recovery,
}

#[derive(Debug)]
pub struct SimpleStatement<'src> {
    pub type_tok: Spanned<ActionType>,
    pub action: Spanned<Iden<'src>>,
    pub selection: Option<Spanned<Selection<'src>>>,
    pub tags: Option<Spanned<Tags<'src>>>,
    pub params: Spanned<Parameters<Expression<'src>>>,
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
    params: Spanned<Parameters<Expression<'src>>>,
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

impl<'src> CalcSpan for IdenPair<'src> {
    fn calculate_span(&self) -> Span {
        self.key.span.start..self.value.span.end
    }
}

#[derive(Debug)]
pub enum Expression<'src> {
    Static(StaticLiteral<'src>),
    Literal(ExprLiteral<'src>),
}

#[derive(Debug)]
pub struct ExprLiteral<'src> {
    pub literal_type: Spanned<ExprLitType>,
    pub args: Spanned<Parameters<StaticLiteral<'src>>>,
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
