use anyhow::{anyhow, Result};
use clap::Clap;
use melt::{decompress, Opts};
use std::path::Path;

fn main() -> Result<()> {
    let opts = Opts::parse();

    for file in opts.files {
        if !Path::new(&file).exists() {
            return Err(anyhow!(format!("The file '{}' does not exist", file)));
        }
        decompress(&file)?;
    }

    Ok(())
}
