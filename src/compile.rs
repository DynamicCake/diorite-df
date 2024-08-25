//! Set of functions for different ways of compiling

use std::{path::Path, sync::Arc};

use lasso::ThreadedRodeo;
use logos::Logos;

use crate::{
    codegen::CodeGenerator, dump::ActionDump, lexer::Token, parser::Parser, project::ProjectFile, semantic::Analyzer
};

pub async fn compile(files: Vec<ProjectFile>) -> () {
    let mut handles = Vec::with_capacity(files.len());
    let rodeo = Arc::new(ThreadedRodeo::new());

    for file in files {
        let src = file.source.clone();
        let file = file.file.clone();

        let rodeo = rodeo.clone();
        let handle = tokio::spawn(async move {
            let lexer = Token::lexer_with_extras(&src, rodeo);
            let ast = Parser::parse(lexer, file.clone());
            ast
        });
        handles.push(handle);
    }

    // Bruh async clojures are unstable
    let mut trees = Vec::new();
    for handle in handles {
        trees.push(handle.await.expect("Thread failed to execute"));
    }

    let _rodeo = Arc::try_unwrap(rodeo).expect("The Arc `rodeo` has escaped this scope");
    // gonna need this for codegen
    // let rodeo = rodeo.into_resolver();
}

pub async fn compile_single(file: ProjectFile, dump: ActionDump) -> String {
    let rodeo = Arc::new(ThreadedRodeo::new());
    let lexer = Token::lexer_with_extras(&file.source, rodeo);
    let ast = Parser::parse(lexer, file.file);

    let checker = Analyzer::resolve(&dump, &ast.program);

    let codegen = CodeGenerator::new(&dump, &ast.program);
    codegen.generate()
}

