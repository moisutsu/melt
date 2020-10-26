use crate::{get_extention, Ext::*};
use anyhow::{anyhow, Result};
use std::process::Command;

pub fn decompress(file_name: String) -> Result<()> {
    if let Some(file_extention) = get_extention(&file_name) {
        match file_extention {
            Zip => decompress_zip(&file_name),
            Tar => decompress_tar(&file_name),
            TarGz => decompress_tar_gz(&file_name),
            TarBz2 => decompress_tar_bz2(&file_name),
            TarXz => decompress_tar_xz(&file_name),
            Other(ext) => Err(anyhow!(format!(
                "The extension `{}` is not supported.",
                ext
            ))),
        }
    } else {
        Err(anyhow!("The file must have an extension."))
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
    let output_to_stdout = String::from_utf8(output.stdout)?;
    let output_to_stderr = String::from_utf8(output.stderr)?;
    if !output_to_stdout.is_empty() {
        println!("{}", output_to_stdout);
    }
    if !output_to_stderr.is_empty() {
        println!("{}", output_to_stderr);
    }
    Ok(())
}
