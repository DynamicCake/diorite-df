#![deny(
    // This compiler uses a lot of lifetimes so this is really important
    elided_lifetimes_in_paths,
)]
// Clogs up the errors, remove when polishing
#![allow(unused)]

pub mod ast;
pub mod cli;
pub mod codegen;
pub mod common;
pub mod dump;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod project;
pub mod semantic;
pub mod test;
pub mod tree;

use std::{env::set_var, error::Error};

use clap::Parser;
use cli::args::Args;
use colored::Colorize;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    set_var("RUST_BACKTRACE", "1");
    let args = Args::parse_from(std::env::args());
    let result = cli::handle(args).await;
    if let Err(err) = result {
        eprintln!("{}{}", "Error: ".red(), err);
    };

    Ok(())
}
