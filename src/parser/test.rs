//! Parser tests are conducted here
//! No semantic analysis here so names do not need to be valid
//! Sometimes `gaction JankCheck ()` is added to make sure the parser is in an okay state
use super::ParsedFile;
use crate::{
    error::syntax::{LexerError, UnexpectedEOF, UnexpectedToken},
    parser::Parser,
};
use lasso::ThreadedRodeo;
use logos::Lexer;
use std::sync::Arc;

type ParseResult = Result<(), ParseFileErrors>;

#[derive(Debug)]
struct ParseFileErrors {
    pub lex_errs: Vec<LexerError>,
    pub parse_errs: Vec<UnexpectedToken>,
    pub at_eof: Option<Box<UnexpectedEOF>>,
}

/// Transform source to [ParseResult]
fn parse_string(src: &str) -> ParseResult {
    let rodeo = Arc::new(ThreadedRodeo::new());
    let lexer = Lexer::with_extras(src, rodeo);
    let file = Parser::parse(lexer, "[test]".into());
    if file.is_successful() {
        Ok(())
    } else {
        Err(ParseFileErrors {
            lex_errs: file.lex_errs,
            parse_errs: file.parse_errs,
            at_eof: file.at_eof,
        })
    }
}

#[test]
fn sanity() -> ParseResult {
    parse_string("")
}

#[test]
fn single() -> ParseResult {
    parse_string(
        r#"
        pevent Exist 
            paction SendMessage ("Everything is fine")
        end
        "#,
    )
}

#[test]
fn top_level() -> ParseResult {
    parse_string(
        r#"
        pevent Join
        end

        eevent Die
        end

        proc GameLoop
        end

        func the(name: type description, other: type here)
        end
    "#,
    )
}

#[test]
fn nest_repeat() -> ParseResult {
    parse_string(
        r#"
        pevent Join
            repeat While <IsSneaking> ()
                repeat IDunno ['Tag 2': 'Electric Boogaloo'] ()
                    paction SendMessage ("Tag! you're it!")
                end
            end
        end
    "#,
    )
}

#[test]
fn nest_if() -> ParseResult {
    parse_string(
        r#"
        pevent Join
            ifplayer not Sneaking <default> ()
                ifgame HasLigma [Strict: 'Maybe So'] ()
                    paction SendMessage ("Ligma nuts")
                end
            end
            gaction JankCheck ()
        end
        "#,
    )
}

#[test]
fn items() -> ParseResult {
    parse_string(
        r#"
        func the()
            paction GiveItem (item('{Count:1b,id:"minecraft:diamond_sword"}'))
            gaction JankCheck ()
        end
        "#,
    )
}
