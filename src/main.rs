use anyhow::Result;
use clap::Parser;
use melt::{decompress, Opts};

fn main() -> Result<()> {
    let opts = Opts::parse();

    for file_path in opts.file_paths {
        decompress(&file_path)?;
    }

    Ok(())
}
