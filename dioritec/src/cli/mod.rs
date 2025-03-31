pub mod args;
pub mod diagnostics;

use crate::codegen::hcp::ProjectMeta;
use args::Args;
use eyre::{eyre, Context};
use std::{path::Path, sync::Arc};
use tokio::{fs::File, io::AsyncWriteExt};

use futures::future;
use lasso::ThreadedRodeo;

use crate::project::{raw::ProjectCreationError, Project, ProjectFile};

pub async fn handle(args: Args) -> eyre::Result<()> {
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

    // raw
    let project = match Project::create_project(
        Arc::try_unwrap(rodeo).expect("rodeo arc escaped scope"),
        files,
        actiondump.into(),
        ProjectMeta {
            name: "dioritec".to_string(),
            version: "0.0.1".to_string(),
            mc_version: "1.21.4".to_string(),
            description: None,
            license: "unlicensed".to_string(),
            authors: vec!["unspecified".to_string()],
        },
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
    println!("{:#?}", project.files);
    // parsed
    let project = project.parse().await;
    println!("{:#?}", project.files);
    let analyzed = project.analyze().await;
    println!("{:#?}", analyzed.files);
    let generated = analyzed.generate();
    println!("{:#?}", generated);

    let mut file = File::create(out)
        .await
        .wrap_err("Unable to create output file")?;
    let stringified = serde_json::to_string_pretty(&generated).expect("Serialization shouldn't fail");
    file.write_all(stringified.as_bytes()).await.wrap_err("Unabled to write to output file")?;

    Ok(())
}
