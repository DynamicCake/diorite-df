#![allow(unused_imports, unused_variables)]
use std::{
    fs::File,
    io::{self, stdin, stdout, BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};

use ariadne::Source;

use crate::{
    args::{Action, Args},
    compile::{self, compile_single, SourceFile},
    diagnostics,
};

pub async fn handle(args: Args) {
    match args.action {
        Action::New { name } => todo!(),
        Action::Init => todo!(),
        Action::Build { target } => todo!(),
        Action::Send { target, all } => todo!(),
        Action::Single { file, tree, out} => single(file),
        Action::Interactive => todo!(),
    };
}

async fn single(file: PathBuf) {
    let mut src = File::open(&file).unwrap();
    let mut buf = String::new();
    src.read_to_string(&mut buf).unwrap();
    let result = compile_single(SourceFile::new(
        buf.into(),
        file.file_name()
            .expect("File open should have been checked before")
            .to_string_lossy()
            .into(),
    )).await;

    println!("{:#?}", result)
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
