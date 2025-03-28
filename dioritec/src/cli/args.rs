use std::{env, path::PathBuf};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub files: Vec<PathBuf>,
    /// Path to the actiondump
    #[arg(short,long)]
    pub dump: PathBuf,

    /// File to output hypercube project to
    #[arg(short, long, default_value = "project.hcp")]
    pub output: PathBuf,
    #[arg(short, long, default_value_os_t = env::current_dir().expect("Cannot get current dir"))]
    pub base_dir: PathBuf,
    /// Project Name 
    #[arg(short, long, default_value = "untitled")]
    pub name: String,
}
