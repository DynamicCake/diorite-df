use std::sync::Arc;

use lasso::ThreadedRodeo;
use logos::Logos;

use crate::{
    lexer::Token,
    parser::{
        error::{ParseResult, UnexpectedToken},
        Parser,
    },
    tree::Program,
};

pub async fn compile(files: Vec<SourceFile>) -> Vec<ParseResult<Program, Vec<UnexpectedToken>>> {
    let mut handles = Vec::with_capacity(files.len());
    let rodeo = Arc::new(ThreadedRodeo::new());

    for file in files {
        let src = file.source.clone();
        let file = file.file.clone();

        let rodeo = rodeo.clone();
        let handle = tokio::spawn(async move {
            let lexer = Token::lexer_with_extras(&src, rodeo);
            let mut parser = Parser::new(lexer, file.clone());
            let ast = parser.parse();
            ast
        });
        handles.push(handle);
    }

    // Bruh async clojures are unstable
    let mut trees = Vec::new();
    for handle in handles {
        trees.push(handle.await.expect("Thread failed to execute"));
    }

    let _rodeo = Arc::try_unwrap(rodeo).expect("Somehow, the Arc has escaped this scope");
    // gonna need this for codegen
    // let rodeo = rodeo.into_resolver();

    trees
}

pub async fn compile_single(file: SourceFile) -> ParseResult<Program> {
    let rodeo = Arc::new(ThreadedRodeo::new());
    let lexer = Token::lexer_with_extras(&file.source, rodeo);
    let mut parser = Parser::new(lexer, file.file);
    let ast = parser.parse();
    ast
}

pub struct SourceFile {
    source: Arc<str>,
    file: Arc<str>,
}

impl SourceFile {
    pub fn new(source: Arc<str>, file: Arc<str>) -> Self {
        Self { source, file }
    }
}
