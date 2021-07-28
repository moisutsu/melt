use anyhow::Result;
use clap::Clap;
use melt::{decompress, Opts};

fn main() -> Result<()> {
    let opts = Opts::parse();

    for file in opts.files {
        decompress(&file)?;
    }

    Ok(())
}
