//! Basically contains the token struct

use std::sync::Arc;

use lasso::{Spur, ThreadedRodeo};
use logos::{Lexer, Logos};

use crate::common::span::{Span, Spanned};

#[derive(Logos, Clone, Debug)]
#[logos(skip r"[ \t\n\f]+", extras = Arc<ThreadedRodeo>)]
pub enum Token {
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

    #[token("else")]
    Else,

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

    #[regex(
        r#"([a-zA-Z_][a-zA-Z0-9_]*)|('([^'\\]*(?:\\.[^'\\]*)*)')"#,
        process_iden
    )]
    Iden(Option<Spur>),
    #[regex(r"\d+(\.\d+)?", process_number)]
    Number(Option<Spur>),
    #[regex(r#""([^"\\]*(?:\\.[^"\\]*)*)""#, process_string)]
    String(Option<Spur>),
    #[regex(r#"\$"([^"\\]*(?:\\.[^"\\]*)*)""#, process_styled_text)]
    StyledText(Option<Spur>),

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

    #[token("repeat")]
    Repeat,

    #[token(r"//[^\n]*")]
    Comment,
    #[token("/*", callback = comment)]
    MultilineComment,

    /// Represents a lexer error
    Invalid,
}

fn process_iden(lex: &mut Lexer<'_, Token>) -> Option<Spur> {
    let text = lex.slice();
    let res = if text.len() >= 2 && text.starts_with('\'') && text.ends_with('\'') {
        &text[1..text.len() - 1]
    } else {
        text
    };
    let spur = lex.extras.get_or_intern(res);

    Some(spur)
}

fn process_string(lex: &mut Lexer<'_, Token>) -> Option<Spur> {
    let text = lex.slice();
    let res = &text[1..text.len() - 1];
    let spur = lex.extras.get_or_intern(res);

    Some(spur)
}

fn process_styled_text(lex: &mut Lexer<'_, Token>) -> Option<Spur> {
    let text = lex.slice();
    let res = &text[2..text.len() - 1];
    let spur = lex.extras.get_or_intern(res);

    Some(spur)
}

fn process_number(lex: &mut Lexer<'_, Token>) -> Option<Spur> {
    let text = lex.slice();
    let spur = lex.extras.get_or_intern(text);

    Some(spur)
}

// FIXME: For now this doesn't work
fn comment(lexer: &mut Lexer<'_, Token>) -> Result<(), ()> {
    println!("Comment triggered!");
    #[derive(Logos, Debug)]
    enum CommentHelper {
        #[token(r"\*")]
        Open,
        #[token(r"*\")]
        Close,
        #[regex(".")]
        AnythingElse,
    }
    let comment_start = lexer.remainder();
    let mut comment_lexer = CommentHelper::lexer(comment_start);
    let mut depth = 1; //we're already inside a comment, so we start from 1
    while depth != 0 {
        match comment_lexer.next() {
            Some(Ok(CommentHelper::Open)) => depth += 1,
            Some(Ok(CommentHelper::Close)) => depth -= 1,
            Some(Ok(CommentHelper::AnythingElse)) => {}
            Some(Err(_)) => return Ok(()),
            None => return Err(()), //unclosed comment
        }
    }
    let comment_end = comment_lexer.remainder();
    let comment_length = comment_end as *const str as *const () as usize
        - comment_start as *const str as *const () as usize;
    lexer.bump(comment_length);
    Ok(())
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Token {
    pub fn spanned(self, span: Span) -> Spanned<Self> {
        Spanned::new(self, span.start..span.end)
    }

    // HACK: Not sure if it is but get another pair of eyes on this

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
    pub fn get_iden_inner(self) -> Spur {
        match self {
            Self::Iden(it) => it.unwrap(),
            it => panic!("Expected Iden, recieved {:#?}", it),
        }
    }
}

impl Token {
    /// Statement starters
    pub const STATEMENT: [Token; 13] = [
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
        Token::Repeat,
    ];

    /// Statement starters and an end to allow for escape
    pub const STATEMENT_LOOP: [Token; 14] = [
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
        Token::Repeat,
        Token::End,
    ];

    /// Starting tokens that can appear in a loop body
    pub const IF_BODY_LOOP: [Token; 15] = [
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
        Token::Repeat,
        Token::End,
        Token::Else,
    ];

    /// Starting tokens for if statements
    pub const IF_STATEMENT: [Token; 4] = [
        Token::IfPlayer,
        Token::IfEntity,
        Token::IfGame,
        Token::IfVar,
    ];

    /// Starting tokens for simple (regular) statements
    pub const SIMPLE_STATEMENT: [Token; 9] = [
        Token::PlayerAction,
        Token::EntityAction,
        Token::GameAction,
        Token::Control,
        Token::CallFunction,
        Token::CallProcess,
        Token::Select,
        Token::SetVar,
        Token::Repeat,
    ];

    /// Starting tokens for top level tokens
    pub const TOP_LEVEL: [Token; 4] = [
        Token::FuncDef,
        Token::ProcDef,
        Token::PlayerEvent,
        Token::EntityEvent,
    ];

    /// Starting tokens for events
    pub const EVENT: [Token; 2] = [Token::PlayerEvent, Token::EntityEvent];

    /// Acceptable arguement tokens for expressions?
    #[deprecated]
    pub const EXPRESSION_ARG: [Token; 2] = [Token::Number(None), Token::String(None)];

    /// Acceptable tokens for paramaters for a codeblock
    pub const POSSIBLE_PARAM: [Token; 3] =
        [Token::Number(None), Token::String(None), Token::Iden(None)];
}
