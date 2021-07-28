use clap::{crate_authors, crate_description, crate_version, Clap};

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!(), about = crate_description!())]
pub struct Opts {
    #[clap(about = "Files to decompress")]
    pub files: Vec<String>,
}
