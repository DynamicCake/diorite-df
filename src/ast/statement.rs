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

impl<'src> NonTerminal<'src> for Selection<'src> {
    fn collect_tokens(self) -> Vec<Spanned<Token<'src>>> {
        let mut out = Vec::new();
        out.push(self.open.map_inner(|_| Token::OpenComp));
        if let Some(it) = self.selection {
            out.push(it.map_inner(|it| Token::Iden(it)))
        };
        out.push(self.close.map_inner(|_| Token::CloseComp));
        out
    }
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

impl<'src> NonTerminal<'src> for Statement<'src> {
    fn collect_tokens(self) -> Vec<Spanned<Token<'src>>> {
        match self {
            Statement::Simple(inner) => inner.to_tokens(),
            Statement::If(inner) => inner.to_tokens(),
            Statement::Recovery(inner) => inner.to_tokens(),
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

impl<'src> NonTerminal<'src> for SimpleStatement<'src> {
    fn collect_tokens(self) -> Vec<Spanned<Token<'src>>> {
        let mut out = vec![
            self.type_tok.map_inner(|it| it.into()),
            self.action.map_inner(|it| it.into()),
        ];
        if let Some(it) = self.selection {
            out.push(it.map_inner(|i| i));
        }

        out.into()
    }
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

impl<'src> Terminal<'src> for ActionType {
    fn to_token(self) -> Token<'src> {
        match self {
            ActionType::PlayerAction => Token::PlayerAction,
            ActionType::EntityAction => Token::EntityAction,
            ActionType::GameAction => Token::GameAction,
            ActionType::Control => Token::Control,
            ActionType::CallFunction => Token::CallFunction,
            ActionType::CallProcess => Token::CallProcess,
            ActionType::Select => Token::Select,
            ActionType::Var => Token::SetVar,
        }
    }
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

impl<'src> NonTerminal<'src> for Expression<'src> {
    fn collect_tokens(self) -> Vec<Spanned<Token<'src>>> {
        match self {
            Expression::Static(it) => it,
            Expression::Literal(it) => it,
        }
    }
}

#[derive(Debug)]
pub struct ExprLiteral<'src> {
    pub literal_type: Spanned<ExprLitType>,
    pub args: Spanned<Parameters<'src, StaticLiteral<'src>>>,
}

impl<'src> NonTerminal<'src> for ExprLiteral<'src> {
    fn collect_tokens(self) -> Vec<Spanned<Token<'src>>> {
        let mut out = Vec::new();
        out.push(self.literal_type.map_inner(|it| it.collect_tokens()));
        out.append(&mut self.args.data.collect_tokens())
    }

}
#[derive(Debug)]
pub enum StaticLiteral<'src> {
    String(StringLiteral<'src>),
    Number(NumberLiteral<'src>),
}

/// Ok ok I get it, this isn't a terminal but deal with it.
/// This was done because I am unable to provide the span for the token
impl<'src> Terminal<'src> for StaticLiteral<'src> {
    fn to_token(self) -> Token<'src> {
        match self {
            StaticLiteral::String(it) => Token::String(it),
            StaticLiteral::Number(it) => Token::Number(it)
        }
    }
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

impl<'src> Terminal<'src> for ExprLitType {
    fn to_token(self) -> Token<'src> {
        match self {
            ExprLitType::SaveVar => Token::SaveVar,
            ExprLitType::GlobalVar => Token::GlobalVar,
            ExprLitType::ThreadVar => Token::ThreadVar,
            ExprLitType::LineVar => Token::LineVar,
            ExprLitType::Location => Token::Location,
            ExprLitType::Vector => Token::Vector,
            ExprLitType::Sound => Token::Sound,
            ExprLitType::Particle => Token::Particle,
            ExprLitType::Potion => Token::Potion,
            ExprLitType::GameValue => Token::GameValue,
        }
    }
}
