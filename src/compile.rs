//! Set of functions to show how to compile

use std::sync::Arc;

use lasso::ThreadedRodeo;
use logos::Logos;

use crate::{
    codegen::CodeGenerator,
    dump::ActionDump,
    lexer::Token,
    parser::Parser,
    semantic::Analyzer,
};

pub async fn compile(files: Vec<SourceFile>) -> () {
    let mut handles = Vec::with_capacity(files.len());
    let rodeo = Arc::new(ThreadedRodeo::new());

    for file in files {
        let src = file.source.clone();
        let file = file.file.clone();

        let rodeo = rodeo.clone();
        let handle = tokio::spawn(async move {
            let lexer = Token::lexer_with_extras(&src, rodeo);
            let parser = Parser::new(lexer, file.clone());
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
}

pub async fn compile_single(file: SourceFile, dump: ActionDump) -> String {
    let rodeo = Arc::new(ThreadedRodeo::new());
    let lexer = Token::lexer_with_extras(&file.source, rodeo);
    let parser = Parser::new(lexer, file.file);
    let ast = parser.parse();

    let dump = Arc::new(dump);
    let checker = Analyzer::new(dump.clone(), &ast.program);

    let codegen = CodeGenerator::new(dump, &ast.program);
    codegen.generate()
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
