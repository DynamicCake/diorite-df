#![deny(
    // This compiler uses a lot of lifetimes so this is really important
    elided_lifetimes_in_paths
)]

mod args;
pub mod codegen;
pub mod compile;
pub mod diagnostics;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod span;
pub mod test;
pub mod tree;
pub mod flow;

use std::{
    env::set_var,
    error::Error,
};

use args::Args;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args = Args::parse();
    set_var("RUST_BACKTRACE", "1");
    flow::handle(args).await;

    println!("Ran!");
    Ok(())
}

