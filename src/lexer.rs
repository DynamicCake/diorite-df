use std::{fmt::Display, rc::Rc, sync::Arc};

use logos::{Logos, Span};

use crate::{
    ast::{statement::ActionType, Iden, Spanned},
    parser::error::ExpectedTokens,
};

#[derive(Logos, Clone, Debug)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'src> {
    #[token("pevent")]
    PlayerEvent,
    #[token("eevent")]
    EntityEvent,
    #[token("proc")]
    ProcDef,
    #[token("func")]
    FuncDef,

    #[token("end")]
    End,

    #[token("not")]
    Not,

    #[token(",")]
    Comma,
    #[token(":")]
    Colon,

    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token("<")]
    OpenComp,
    #[token(">")]
    CloseComp,

    #[regex(r#"([a-zA-Z_][a-zA-Z0-9_]*)|('([^'\\]*(?:\\.[^'\\]*)*)')"#, |lexer| Some(lexer.slice()) )]
    Iden(Option<&'src str>),
    #[regex(r"\d+(\.\d+)?", |lexer| Some(lexer.slice()) )]
    Number(Option<&'src str>),
    #[regex(r#""([^"\\]*(?:\\.[^"\\]*)*)""#, |lexer| Some(lexer.slice()))]
    String(Option<&'src str>),

    #[token("svar")]
    SaveVar,
    #[token("gvar")]
    GlobalVar,
    #[token("tvar")]
    ThreadVar,
    #[token("lvar")]
    LineVar,
    #[token("loc")]
    Location,
    #[token("vec")]
    Vector,
    #[token("snd")]
    Sound,
    #[token("part")]
    Particle,
    #[token("pot")]
    Potion,
    #[token("gval")]
    GameValue,

    #[token("paction")]
    PlayerAction,
    #[token("eaction")]
    EntityAction,
    #[token("gaction")]
    GameAction,
    #[token("control")]
    Control,
    #[token("callf")]
    CallFunction,
    #[token("callp")]
    CallProcess,
    #[token("select")]
    Select,
    #[token("var")]
    SetVar,

    #[token("ifplayer")]
    IfPlayer,
    #[token("ifentity")]
    IfEntity,
    #[token("ifgame")]
    IfGame,
    #[token("ifvar")]
    IfVar,

    /// Represents a lexer error
    Invalid,
}

impl<'src> PartialEq for Token<'src> {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl<'src> Token<'src> {
    pub fn spanned(self, span: Span) -> Spanned<Self> {
        Spanned::new(self, span)
    }

    // HACK Not sure if it is but get another pair of eyes on this
    /// When having `Expected: Whatever, Something`, it makes it so the inner contents aren't visible,
    /// This feels hacky and I don't really like it
    pub fn expected_print(&self) -> String {
        match self {
            Token::Iden(_) => "Iden".to_string(),
            Token::Number(_) => "Number".to_string(),
            Token::String(_) => "String".to_string(),
            it => {
                format!("{:?}", it)
            }
        }
    }

    /// Gets the iden, if the varient isn't a Iden, this function panics
    pub fn get_iden(&self) -> &'src str {
        match self {
            Self::Iden(it) => return it.unwrap(),
            it => panic!("Expected Iden recieved {:#?}", it)
        }
    }
}

impl<'src> Token<'src> {
    pub const STATEMENT: [Token<'src>; 12] = [
        Token::PlayerAction,
        Token::EntityAction,
        Token::GameAction,
        Token::Control,
        Token::CallFunction,
        Token::CallProcess,
        Token::Select,
        Token::SetVar,
        Token::IfPlayer,
        Token::IfEntity,
        Token::IfGame,
        Token::IfVar,
    ];

    pub const STATEMENT_LOOP: [Token<'src>; 13] = [
        Token::PlayerAction,
        Token::EntityAction,
        Token::GameAction,
        Token::Control,
        Token::CallFunction,
        Token::CallProcess,
        Token::Select,
        Token::SetVar,
        Token::IfPlayer,
        Token::IfEntity,
        Token::IfGame,
        Token::IfVar,
        Token::End,
    ];

    pub const IF_STATEMENT: [Token<'src>; 4] = [
        Token::IfPlayer,
        Token::IfEntity,
        Token::IfGame,
        Token::IfVar,
    ];

    pub const SIMPLE_STATEMENT: [Token<'src>; 8] = [
        Token::PlayerAction,
        Token::EntityAction,
        Token::GameAction,
        Token::Control,
        Token::CallFunction,
        Token::CallProcess,
        Token::Select,
        Token::SetVar,
    ];

    pub const TOP_LEVEL: [Token<'src>; 4] = [
        Token::FuncDef,
        Token::ProcDef,
        Token::PlayerEvent,
        Token::EntityEvent,
    ];

    pub const EVENT: [Token<'src>; 2] = [Token::PlayerEvent, Token::EntityEvent];
}
