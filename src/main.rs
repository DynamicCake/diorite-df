#![deny(
    // This compiler uses a lot of lifetimes so this is really important
    elided_lifetimes_in_paths,
)]

mod args;
pub mod codegen;
pub mod compile;
pub mod diagnostics;
pub mod dump;
pub mod error;
pub mod flow;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod span;
pub mod test;
pub mod tree;

use std::{env::set_var, error::Error};

use args::Args;
use clap::Parser;
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args = Args::parse();
    set_var("RUST_BACKTRACE", "1");
    if let Err(err) = flow::handle(args).await {
        eprintln!("{}{}", "Error: ".red(), err.to_string().red());
    };

    println!("Ran!");
    Ok(())
}
