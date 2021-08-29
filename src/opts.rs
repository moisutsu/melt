use std::path::PathBuf;

use clap::{crate_authors, crate_description, crate_version, Clap};

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!(), about = crate_description!())]
pub struct Opts {
    #[clap(about = "Files to decompress", parse(from_os_str))]
    pub file_paths: Vec<PathBuf>,
}
