//! Module for handling different ways to compile this language
//! Probably will be moved to dioget and therefore would not be documented

#![allow(unused_imports, unused_variables)]
use core::panic;
use std::{
    fs::{self, File},
    io::{self, stdin, stdout, BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Instant,
};

use ariadne::Source;
use lasso::ThreadedRodeo;

use crate::{
    args::{Action, Args},
    diagnostics,
    error::cli::CliError,
    project::{Project, ProjectFile, ProjectFileCreationError},
};

pub async fn handle(args: Args) -> Result<(), CliError> {
    match args.action {
        Action::New { name } => todo!(),
        Action::Init => todo!(),
        Action::Build { target } => todo!(),
        Action::Send { target, all } => todo!(),
        Action::Single {
            file,
            tree,
            out,
            dump,
        } => {
            let out = out.unwrap_or_else(|| file.clone());
            single(file, dump, out, tree).await?
        }
        Action::Interactive => todo!(),
    };
    Ok(())
}

async fn single(
    mut src_file: PathBuf,
    actiondump: PathBuf,
    out: PathBuf,
    // Temporarily here for debugging purpouses lol
    tree: bool,
) -> Result<String, CliError> {
    let rodeo = Arc::new(ThreadedRodeo::new());
    src_file.pop();
    let root = if let Some(it) = src_file.to_str() {
        it
    } else {
        return Err(CliError::NonUtf8File(src_file.into()));
    };

    let file = match ProjectFile::new(&src_file, rodeo.get_or_intern(root), rodeo.clone()).await {
        Ok(it) => it,
        Err(err) => match err {
            ProjectFileCreationError::Io(e) => {
                return Err(CliError::CannotReadSource {
                    file: src_file.into(),
                    code: e,
                })
            }
            e => {
                panic!("Unexpected error: {e}")
            }
        },
    };

    let project = Project::create_project(
        Arc::try_unwrap(rodeo).expect("rodeo arc escaped scope"),
        vec![file],
        actiondump.into(),
    )
    .await;

    // TODO: Create project analysis and codegen
    Ok(todo!())
}

fn interactive() -> Result<String, io::Error> {
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

/*
async fn single(args: Args) {
    let (src, file) = if let Some(path) = args.file {
        (compile_file(&path), Some(path))
    } else {
        (compile_prompt(), None)
    };

    let file: Arc<str> = match file {
        Some(it) => it.to_string_lossy().into(),
        None => "<stdin>".into(),
    };

    println!("Compiling...");
    let now = Instant::now();
    let res = compile::compile(vec![SourceFile::new(src.into(), file)]).await;
    let time = now.elapsed().as_millis();

    for program in res {
        for err in program.error {
            diagnostics::generate_syntax_error(file.clone(), err)
                .print((file.clone(), Source::from(&src)))
                .unwrap();
        }
    }


}
*/
