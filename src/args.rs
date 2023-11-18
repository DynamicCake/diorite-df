use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Arguments {
    #[arg(short, long)]
    pub file: Option<PathBuf>
}