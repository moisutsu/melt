use anyhow::Result;
use clap::Clap;
use melt::{decompress, Opts};

fn main() -> Result<()> {
    let opts = Opts::parse();

    for file_path in opts.file_paths {
        decompress(&file_path)?;
    }

    Ok(())
}
