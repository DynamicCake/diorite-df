use crate::{
    ast::AstRoot,
    common::prelude::*,
    dump::ActionDump,
    error::syntax::{LexerError, UnexpectedEOF, UnexpectedToken},
    parser::Parser,
    tree::TreeRoot,
};

use std::{
    hash::{Hash, Hasher},
    path::Path,
    sync::Arc,
};

use super::{raw::RawProjectFiles, FileResolution, Project, ProjectFile, ProjectFiles };
use lasso::{RodeoResolver, Spur, ThreadedRodeo};
use logos::{Lexer, Logos};
use rustc_hash::FxHasher;
use tokio::{
    fs::File,
    io::{self, AsyncReadExt},
};

impl Project<RawProjectFiles> {
    pub async fn parse(self) -> Project<ParsedProjectFiles> {
        // Async clojures are unstable
        let mut trees = Vec::new();
        let mut lex_errs = Vec::new();
        let mut parse_errs = Vec::new();
        let mut eof_errs = Vec::new();

        let rodeo = Arc::new(self.files.rodeo);
        let mut handles = Vec::new();
        for file in self.files.files {
            let rodeo = rodeo.clone();
            let handle = tokio::spawn(async move {
                let src = rodeo.resolve(&file.src);
                let lexer = Token::lexer_with_extras(src, rodeo.clone());
                let tree = Parser::parse(lexer, file.path);
                (tree, file)
            });
            handles.push(handle);
        }
        for handle in handles {
            let (mut tree, file) = handle.await.expect("Thread failed to execute");
            lex_errs.append(&mut tree.lex_errs);
            parse_errs.append(&mut tree.parse_errs);
            if let Some(eof) = tree.at_eof {
                eof_errs.push(*eof);
            }
            let file = file.to_parsed(tree.root);
            trees.push(file);
        }
        let resolver = Arc::try_unwrap(rodeo).expect("Rodeo escaped scope").into_resolver();
        let files = ParsedProjectFiles::new(trees, lex_errs, parse_errs, eof_errs, Arc::new(resolver));
        Project::<ParsedProjectFiles> {
            resources: self.resources,
            hash: self.hash,
            files,
        }
    }
}

/// A program state with a parse tree
#[derive(Debug)]
pub struct ParsedProjectFiles {
    pub lex_errs: Vec<LexerError>,
    pub parse_errs: Vec<UnexpectedToken>,
    pub eof_errs: Vec<UnexpectedEOF>,
    pub parsed: Vec<ProjectFile<TreeFile>>,
    pub resolver: Arc<RodeoResolver>,
}

impl ParsedProjectFiles {
    fn new(
        files: Vec<ProjectFile<TreeFile>>,
        lex_errs: Vec<LexerError>,
        parse_errs: Vec<UnexpectedToken>,
        eof_errs: Vec<UnexpectedEOF>,
        resolver: Arc<RodeoResolver>
    ) -> Self {
        Self {
            parsed: files,
            lex_errs,
            parse_errs,
            eof_errs,
            resolver
        }
    }
}

// Parsing
#[derive(Debug, PartialEq)]
pub struct TreeFile {
    pub root: TreeRoot,
}

/// This is named like (Parse)TreeFile because ParsedFile also refers to
/// parse tree + errors that is returned from parser
impl TreeFile {
    pub fn new(program: TreeRoot) -> Self {
        Self { root: program }
    }
}
impl FileResolution for TreeFile {}
impl ProjectFiles for ParsedProjectFiles {}

impl ProjectFile<TreeFile> {}
