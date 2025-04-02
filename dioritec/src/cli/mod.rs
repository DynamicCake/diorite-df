pub mod args;
pub mod diagnostics;

use crate::codegen::hcp::ProjectMeta;
use crate::error::CompilerError;
use crate::{error::diagnostic::DiagnosticsGenerator, semantic::AnalyzedFile};
use args::Args;
use ariadne::Source;
use eyre::{eyre, Context};
use std::collections::HashMap;
use std::{path::Path, sync::Arc};
use tokio::{fs::File, io::AsyncWriteExt};

use futures::future;
use lasso::{Resolver, RodeoResolver, Spur, ThreadedRodeo};

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
        // This will be in the diorite.toml file with diorite and not dioritec
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
    // language error territory
    // parsed
    let project = project.parse().await;
    let resolver = project.files.resolver.clone();
    if project.files.has_errors() {
        let files = project.files;
        let mut errors = Vec::new();
        files
            .lex_errs
            .into_iter()
            .for_each(|e| errors.push(CompilerError::Lexer(e)));
        files
            .parse_errs
            .into_iter()
            .for_each(|e| errors.push(CompilerError::Parse(e)));
        files
            .eof_errs
            .into_iter()
            .for_each(|e| errors.push(CompilerError::Eof(e)));

        run_diagnostics(errors, resolver, &project.file_map);
        return Ok(());
    }
    let analyzed = project.analyze().await;
    let (generated, errs, file_map) = analyzed.generate();
    println!("{errs:?}");
    if !errs.is_empty() {
        run_diagnostics(errs, resolver, &file_map);
        return Ok(());
    }
    println!("{:#?}", generated);

    let mut file = File::create(out)
        .await
        .wrap_err("Unable to create output file")?;
    let stringified =
        serde_json::to_string_pretty(&generated).expect("Serialization shouldn't fail");
    file.write_all(stringified.as_bytes())
        .await
        .wrap_err("Unabled to write to output file")?;

    Ok(())
}

fn run_diagnostics(
    errs: Vec<CompilerError>,
    resolver: Arc<RodeoResolver>,
    file_map: &HashMap<Spur, Spur>,
) {
    let generator = DiagnosticsGenerator::new(resolver.clone());
    generator
        .generate(errs)
        .into_iter()
        .for_each(|(err, file)| {
            err.eprint((
                resolver.resolve(&file),
                Source::from(
                    resolver.resolve(file_map.get(&file).expect("Spur always contain valid path")),
                ),
            ))
            .unwrap()
        });
}
