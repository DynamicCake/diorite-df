//! Module for handling different ways to compile this language
//! Probably will be moved to dioget and therefore would not be documented

use arrayvec::ArrayVec;
use core::panic;
use eyre::eyre;
use std::{
    fs::{self, File},
    io::{self, stdin, stdout, BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Instant,
};

use ariadne::Source;
use futures::future;
use lasso::{Resolver, ThreadedRodeo};

use crate::project::{
    ActionDumpReadError, Project, ProjectCreationError, ProjectFile, ProjectFileCreationError,
};

use super::{args::Args, diagnostics};

pub async fn handle(args: Args) -> eyre::Result<String> {
    multi(args.files, args.dump, args.output).await
}

async fn multi(src_paths: Vec<PathBuf>, actiondump: PathBuf, out: PathBuf) -> eyre::Result<String> {
    let rodeo = Arc::new(ThreadedRodeo::new());

    // Start io stuff that can fail easily
    let mut handles = src_paths.into_iter().map(|path| {
        let rodeo = rodeo.clone();
        tokio::spawn(async move {
            let src_path = path.canonicalize().unwrap();
            if !src_path.is_file() {
                return Err(eyre!("{:?} is not a file", src_path));
            }
            let file_name = rodeo.get_or_intern(src_path.file_name().unwrap().to_str().unwrap());

            let mut dir = src_path;
            dir.pop();
            let root = if let Some(it) = dir.to_str() {
                it
            } else {
                return Err(eyre!("The file path to {:?} is not utf-8", dir));
            };

            let file_name = dir.join(rodeo.resolve(&file_name));
            let file = match ProjectFile::read(
                Path::new(&file_name),
                rodeo.get_or_intern(root),
                rodeo.clone(),
            )
            .await
            {
                Ok(it) => it,
                Err(err) => {
                    return Err(eyre!(
                        "Cannot use file '{:?}' due to io error '{}'",
                        &file_name,
                        err
                    ))
                }
            };
            Ok(file)
        })
    });
    let mut files: Vec<_> = Vec::with_capacity(handles.len());
    for result in future::join_all(handles).await {
        match result.expect("join shouldn't fail") {
            Ok(it) => files.push(it),
            Err(err) => return Err(eyre!("File init error: {}", err)),
        }
    }

    let project = match Project::create_project(
        Arc::try_unwrap(rodeo).expect("rodeo arc escaped scope"),
        files,
        actiondump.into(),
    )
    .await
    {
        Ok(it) => it,
        Err(err) => {
            return Err(match err {
                ProjectCreationError::ActionDump(e) => eyre!("Actiondump error: {}", e),
                ProjectCreationError::NoFilesInputed => eyre!("No files provided"),
                ProjectCreationError::RootsDoNotMatch { root, file } => todo!(),
            })
        }
    };
    println!("{:#?}", project.files.parsed);

    // TODO: Create project analysis and codegen
    todo!()
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
