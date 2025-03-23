use std::{io, path::{Path, PathBuf}};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Cannot read source file `{file}` with code `{code}`")]
    CannotReadSource { file: Box<Path>, code: io::Error },
    #[error("Cannot read action dump file `{file}` with code `{code}`")]
    CannotReadActionDump { file: Box<Path>, code: io::Error },
    #[error("Malformed actiondump file `{file}` with error `{error}`")]
    MalformedActionDump {
        file: Box<Path>,
        error: serde_json::Error,
    },
    #[error("The file path to {0} is not utf-8")]
    NonUtf8File(Box<Path>)

}
