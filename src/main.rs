#![deny(
    // This compiler uses a lot of lifetimes so this is really important
    elided_lifetimes_in_paths
)]

mod args;
pub mod ast;
pub mod diagnostics;
pub mod lexer;
pub mod parser;
pub mod test;

use std::{
    env::set_var,
    error::Error,
    fs::File,
    io::{self, stdin, stdout, BufRead, BufReader, Read, Write},
    path::PathBuf,
    sync::Arc,
};

use args::Arguments;
use ariadne::Source;
use ast::Program;
use clap::Parser;
use lasso::ThreadedRodeo;
use lexer::Token;
use logos::Logos;
use parser::error::{ParseResult, UnexpectedToken};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let _args = Arguments::parse();
    set_var("RUST_BACKTRACE", "1");

    /*
    let src = if let Some(path) = args.file {
        compile_file(path)
    } else {
        compile_prompt()
    }?;
    */

    let src = r#"
    pevent Join 
        paction SendMessage ('IT WORKS'("lets go", 42,),)
    end
    "#;

    let res = compile(src);
    let file: Arc<str> = "test.drt".into();
    for err in res.error {
        diagnostics::generate_syntax_error(file.clone(), err)
            .print((file.clone(), Source::from(src)))
            .unwrap();
    }

    println!("Ran!");

    Ok(())
}

fn compile(src: &str) -> ParseResult<Program, Vec<UnexpectedToken>> {
    let rodeo = Arc::new(ThreadedRodeo::new());
    let lexer = Token::lexer_with_extras(src, rodeo.clone());
    let mut parser = parser::Parser::new(lexer, rodeo);
    let ast = parser.parse();

    ast
}

fn compile_file(path: PathBuf) -> Result<String, io::Error> {
    let mut src = File::open(path)?;
    let mut buf = String::new();
    src.read_to_string(&mut buf)?;

    Ok(buf)
}

fn compile_prompt() -> Result<String, io::Error> {
    let stdin = stdin();
    let mut stdout = stdout();
    let mut input = BufReader::new(stdin);

    let mut src = Vec::new();

    loop {
        print!("> ");
        stdout.flush().unwrap();
        let mut line = String::new();
        input.read_line(&mut line)?;
        let line = line.trim().to_owned();
        if line.is_empty() {
            break;
        }
        src.push(line)
    }
    Ok(src.join("\n"))
}
