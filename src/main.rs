use anyhow::{anyhow, Result};
use clap::Clap;
use melt::{decompress, Opts};
use std::path::Path;

fn main() -> Result<()> {
    let opts = Opts::parse();
    if !Path::new(&opts.input_file).exists() {
        return Err(anyhow!(format!(
            "The file {} does not exist",
            opts.input_file
        )));
    }
    decompress(&opts.input_file)?;
    Ok(())
}
