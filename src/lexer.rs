use logos::Logos;

#[derive(Logos, Clone, PartialEq, Debug)]
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

    #[regex(r#"([a-zA-Z_][a-zA-Z0-9_]*)|('([^'\\]*(?:\\.[^'\\]*)*)')"#, |lexer| lexer.slice() )]
    Identifier(&'src str),
    #[regex(r"\d+(\.\d+)?", |lexer| lexer.slice() )]
    Number(&'src str),
    #[regex(r#""([^"\\]*(?:\\.[^"\\]*)*)""#, |lexer| lexer.slice())]
    String(&'src str),

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
}
