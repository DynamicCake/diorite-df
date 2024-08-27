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
use lasso::{Resolver, ThreadedRodeo};

use crate::{
    args::{Action, Args},
    diagnostics,
    error::cli::CliError,
    project::{
        ActionDumpReadError, Project, ProjectCreationError, ProjectFile, ProjectFileCreationError,
    },
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
            single(file, dump, out).await?
        }
        Action::Interactive => todo!(),
    };
    Ok(())
}

async fn single(
    src_file: PathBuf,
    actiondump: PathBuf,
    out: PathBuf,
) -> Result<String, CliError> {
    let rodeo = Arc::new(ThreadedRodeo::new());
    let src_file = src_file.canonicalize().unwrap();
    let src = rodeo.get_or_intern(src_file.file_name().unwrap().to_str().unwrap());

    let mut dir = src_file;
    dir.pop();
    let root = if let Some(it) = dir.to_str() {
        it
    } else {
        return Err(CliError::NonUtf8File(dir.into()));
    };

    let file = match ProjectFile::read(Path::new(rodeo.resolve(&src)), rodeo.get_or_intern(root), rodeo.clone()).await {
        Ok(it) => it,
        Err(err) => match err {
            ProjectFileCreationError::Io(e) => {
                return Err(CliError::CannotReadSource {
                    file: dir.into(),
                    code: e,
                })
            }
            e => {
                panic!("Unexpected error: {e}")
            }
        },
    };

    let project = match Project::create_project(
        Arc::try_unwrap(rodeo).expect("rodeo arc escaped scope"),
        vec![file],
        actiondump.into(),
    )
    .await
    {
        Ok(it) => it,
        Err(err) => match err {
            ProjectCreationError::ActionDump(e) => {
                return Err(match e {
                    ActionDumpReadError::Io(path, e) => CliError::CannotReadActionDump {
                        file: path,
                        code: e,
                    },
                    ActionDumpReadError::Parse(path, e) => CliError::MalformedActionDump {
                        file: path,
                        error: e,
                    },
                })
            }
            e => panic!("Unexpected: {e:#?}")
        },
    };

    println!("{:#?}", project.files.parsed[0].resolution);

    // TODO: Create project analysis and codegen
    todo!()
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
