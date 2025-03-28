#![deny(
    // This compiler uses a lot of lifetimes so this is really important
    elided_lifetimes_in_paths,
)]
// Clogs up the errors, remove when polishing
#![allow(unused)]

pub mod ast;
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
