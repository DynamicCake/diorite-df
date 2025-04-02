//! Tests for modules without mod.rs like [crate::lexer]

// Lexer
// Asserts that the next token is a certain token

#[test]
fn lexer_test_basic() {
    macro_rules! next_eq {
        ($lexer:expr, $tok:expr) => {
            assert_eq!($lexer.next().unwrap().unwrap(), $tok);
        };
    }
    use crate::common::prelude::*;
    use lasso::ThreadedRodeo;
    use logos::Logos;
    use std::sync::Arc;
    let src = r#"
        paction eaction gaction
        control callf callp
        ifplayer ifentity ifgame ifvar
        repeat select var
        pevent eevent proc func
        end not else
        , : () {} [] <>
    "#;

    let rodeo = Arc::new(ThreadedRodeo::new());
    let mut lex = Token::lexer_with_extras(src, rodeo);

    next_eq!(lex, Token::PlayerAction);
    next_eq!(lex, Token::EntityAction);
    next_eq!(lex, Token::GameAction);

    next_eq!(lex, Token::Control);
    next_eq!(lex, Token::CallFunction);
    next_eq!(lex, Token::CallProcess);

    next_eq!(lex, Token::IfPlayer);
    next_eq!(lex, Token::IfEntity);
    next_eq!(lex, Token::IfGame);
    next_eq!(lex, Token::IfVar);

    next_eq!(lex, Token::Repeat);
    next_eq!(lex, Token::Select);
    next_eq!(lex, Token::SetVar);

    next_eq!(lex, Token::PlayerEvent);
    next_eq!(lex, Token::EntityEvent);
    next_eq!(lex, Token::ProcDef);
    next_eq!(lex, Token::FuncDef);

    next_eq!(lex, Token::End);
    next_eq!(lex, Token::Not);
    next_eq!(lex, Token::Else);

    next_eq!(lex, Token::Comma);
    next_eq!(lex, Token::Colon);

    next_eq!(lex, Token::OpenParen);
    next_eq!(lex, Token::CloseParen);
    next_eq!(lex, Token::OpenBrace);
    next_eq!(lex, Token::CloseBrace);
    next_eq!(lex, Token::OpenBracket);
    next_eq!(lex, Token::CloseBracket);
    next_eq!(lex, Token::OpenComp);
    next_eq!(lex, Token::CloseComp);
}

#[test]
fn tok_eq() {
    use crate::lexer::Token;
    use lasso::ThreadedRodeo;

    let rodeo = ThreadedRodeo::new();
    let a1 = Token::Iden(Some(rodeo.get_or_intern("tok a")));
    let a2 = Token::Iden(Some(rodeo.get_or_intern("tok a")));
    let b1 = Token::Iden(Some(rodeo.get_or_intern("tok b")));
    assert_eq!(a1, a2);
    assert_eq!(a1, b1);
}

#[test]
fn lexer_iden() {
    use lasso::ThreadedRodeo;
    use logos::Logos;
    use std::sync::Arc;

    use crate::lexer::Token;

    let rodeo = Arc::new(ThreadedRodeo::new());
    let src = "'hello'";
    let lexer = Token::lexer_with_extras(src, rodeo.clone());
    let _ = lexer.collect::<Vec<_>>(); // consume
    assert!(rodeo.contains("hello"));
}

#[test]
fn lexer_string() {
    use lasso::ThreadedRodeo;
    use logos::Logos;
    use std::sync::Arc;

    use crate::lexer::Token;

    let rodeo = Arc::new(ThreadedRodeo::new());
    let src = "\"world\"";
    let lexer = Token::lexer_with_extras(src, rodeo.clone());
    let _ = lexer.collect::<Vec<_>>(); // consume
    assert!(rodeo.contains("world"));
}

#[test]
fn lexer_multiline() {
    use crate::common::prelude::*;
    use lasso::ThreadedRodeo;
    use logos::Logos;
    use std::sync::Arc;
    // Yes, it should be formatted like that
    let src = r#"
        paction Join
            paction SendMessage ("=====
Welcome to my Plot!
=====")
        end
    "#;
    let rodeo = Arc::new(ThreadedRodeo::new());
    let lexer = Token::lexer_with_extras(src, rodeo.clone());
    let _ = lexer.collect::<Vec<_>>(); // consume
    assert!(rodeo.contains(
        r#"=====
Welcome to my Plot!
====="#
    ));
}

#[test]
fn lexer_styled() {
    use crate::common::prelude::*;
    use lasso::ThreadedRodeo;
    use logos::Logos;
    use std::sync::Arc;

    let src = r#"
        paction Join
            paction SendMessage ($"<white>balls")
        end
    "#;
    let rodeo = Arc::new(ThreadedRodeo::new());
    let lexer = Token::lexer_with_extras(src, rodeo.clone());
    let _ = lexer.collect::<Vec<_>>(); // consume
    assert!(rodeo.contains("<white>balls")); // This is better than &fballs fight me
}
