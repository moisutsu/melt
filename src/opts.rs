use clap::{crate_version, Clap};

#[derive(Clap)]
#[clap(version = crate_version!(), author = "moisutsu <moisutsu@gmail.com>")]
pub struct Opts {
    pub input_file: String,
}
