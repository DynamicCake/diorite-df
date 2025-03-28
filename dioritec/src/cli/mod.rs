pub mod args;
pub mod diagnostics;

use args::Args;
use eyre::eyre;
use std::{path::Path, sync::Arc};

use futures::future;
use lasso::ThreadedRodeo;

use crate::project::{Project, ProjectCreationError, ProjectFile};

pub async fn handle(args: Args) -> eyre::Result<String> {
    let src_paths = args.files;
    let actiondump = args.dump;
    let out = args.output;
    let rodeo = Arc::new(ThreadedRodeo::new());
    let handles = src_paths.into_iter().map(|path| {
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
