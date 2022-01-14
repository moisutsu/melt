use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Opts {
    #[clap(help = "Files to decompress", parse(from_os_str))]
    pub file_paths: Vec<PathBuf>,
}
