use crate::{get_extention, Ext::*};
use anyhow::{anyhow, Result};
use std::process::Command;

pub fn decompress(file_name: String) -> Result<()> {
    match get_extention(&file_name) {
        Zip => Ok(decompress_zip(&file_name)?),
        Tar => Ok(decompress_tar(&file_name)?),
        TarGz => Ok(decompress_tar_gz(&file_name)?),
        TarBz2 => Ok(decompress_tar_bz2(&file_name)?),
        TarXz => Ok(decompress_tar_xz(&file_name)?),
        Other(ext) => Err(anyhow!(format!("The extension {} is not supported.", ext))),
    }
}

fn decompress_zip(file_name: &String) -> Result<()> {
    let output = Command::new("unzip").arg(file_name).output()?;
    print_output(output)?;
    Ok(())
}

fn decompress_tar(file_name: &String) -> Result<()> {
    let output = Command::new("tar").arg("-xvf").arg(file_name).output()?;
    print_output(output)?;
    Ok(())
}

fn decompress_tar_gz(file_name: &String) -> Result<()> {
    let output = Command::new("tar").arg("-zxvf").arg(file_name).output()?;
    print_output(output)?;
    Ok(())
}

fn decompress_tar_bz2(file_name: &String) -> Result<()> {
    let output = Command::new("tar").arg("-jxvf").arg(file_name).output()?;
    print_output(output)?;
    Ok(())
}

fn decompress_tar_xz(file_name: &String) -> Result<()> {
    let output = Command::new("tar").arg("-Jxvf").arg(file_name).output()?;
    print_output(output)?;
    Ok(())
}

fn print_output(output: std::process::Output) -> Result<()> {
    println!("{}", String::from_utf8(output.stdout)?);
    println!("{}", String::from_utf8(output.stderr)?);
    Ok(())
}
