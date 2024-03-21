use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    New {
        name: String,
    },
    Init,
    Build {
        #[clap(default_value = "raw")]
        target: String,
    },
    Send {
        target: String,
        #[arg(long)]
        all: bool,
    },
    Single {
        file: PathBuf,
        #[arg(short, long)]
        out: Option<PathBuf>,
        #[arg(long)]
        tree: bool,
    },
    #[command(name = "interactive")]
    Interactive
}
