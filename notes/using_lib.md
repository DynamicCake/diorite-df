# How to use the API

```rs
async fn single(
    mut src_file: PathBuf,
    actiondump: PathBuf,
    out: PathBuf,
) -> Result<String, CliError> {
    // Create a new rodeo (string interner)
    let rodeo = Arc::new(ThreadedRodeo::new());
    src_file.pop();
    // Find root from file
    let root = if let Some(it) = src_file.to_str() {
        it
    } else {
        return Err(CliError::NonUtf8File(src_file.into()));
    };

    // Create a new project file and handle errors
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

    // Create project using project files
    let project = match Project::create_project(
        Arc::try_unwrap(rodeo).expect("rodeo arc escaped scope"),
        vec![file],
        actiondump.into(),
    )
    .await
    // Handle the errors
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

    // Things should be done parsing here and the next thing to do is analysis
    Ok(todo!())
}
```

