#![deny(
    // This compiler uses a lot of lifetimes so this is really important
    elided_lifetimes_in_paths
)]
// This is in place until the program is somewhat stable
#![allow(unused_imports, unused_import_braces, dead_code)]

mod args;
pub mod ast;
pub mod lexer;
pub mod parser;
pub mod test;

use std::{
    error::Error,
    fs::File,
    io::{self, stdin, stdout, BufRead, BufReader, Read, Write},
    path::PathBuf,
};

use args::Arguments;
use ast::{top::TopLevel, Program};
use clap::Parser;
use lexer::Token;
use logos::Logos;
use parser::error::{CompilerResult, UnexpectedToken};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let _args = Arguments::parse();

    /*
    let src = if let Some(path) = args.file {
        compile_file(path)
    } else {
        compile_prompt()
    }?;
    */

    let src = "pevent Join end";
    let res = compile(src);

    

    println!("Somehow: {:#?}", res);

    Ok(())
}

fn compile(
    src: &str,
) -> CompilerResult<'_, Program<'_>, Vec<UnexpectedToken<'_>>> {
    let lexer = Token::lexer(src);
    let print_lexer: Vec<_> = lexer.clone().collect();
    println!("{:?}", print_lexer);
    let mut parser = parser::Parser::new(lexer);
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
