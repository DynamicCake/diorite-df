//! Module for handling different ways to compile this language
//! Probably will be moved to dioritec and therefore would not be documented

#![allow(unused_imports, unused_variables)]
use std::{
    fs::{self, File},
    io::{self, stdin, stdout, BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Instant,
};

use ariadne::Source;

use crate::{
    args::{Action, Args},
    compile::{self, compile_single, SourceFile},
    diagnostics,
    error::cli::CliError,
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
    file: PathBuf,
    actiondump: PathBuf,
    out: PathBuf,
    tree: bool,
) -> Result<String, CliError> {
    let src = match fs::read_to_string(&file) {
        Ok(it) => it,
        Err(code) => return Err(CliError::CannotReadSource { file, code }),
    };

    let actiondump = match fs::read_to_string(actiondump) {
        Ok(it) => it,
        Err(code) => return Err(CliError::CannotReadActionDump { file, code }),
    };

    let parsed = match serde_json::from_str(&actiondump) {
        Ok(it) => it,
        Err(error) => return Err(CliError::MalformedActionDump { file, error }),
    };

    let result = compile_single(
        SourceFile::new(
            src.into(),
            file.file_name()
                .expect("File open should have been opened before and therefore exists")
                .to_string_lossy()
                .into(),
        ),
        parsed,
    )
    .await;

    if tree {
        println!("{}", result);
    }

    Ok(result)
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
