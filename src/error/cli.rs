use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Cannot read source file `{file}` with code `{code}`")]
    CannotReadSource { file: PathBuf, code: io::Error },
    #[error("Cannot read action dump file `{file}` with code `{code}`")]
    CannotReadActionDump { file: PathBuf, code: io::Error },
    #[error("Malformed actiondump file `{file}` with error `{error}`")]
    MalformedActionDump {
        file: PathBuf,
        error: serde_json::Error,
    },
}
