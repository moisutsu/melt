use anyhow::Result;
use clap::Clap;
use melt::{decompress, Opts};

fn main() -> Result<()> {
    let opts = Opts::parse();
    decompress(opts.input_file)?;
    Ok(())
}
