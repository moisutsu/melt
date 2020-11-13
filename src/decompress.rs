use crate::{get_extention, Ext::*};
use anyhow::{anyhow, Result};
use std::process::{Command, Stdio};

pub fn decompress(file_name: &str) -> Result<()> {
    if let Some(file_extention) = get_extention(file_name) {
        match file_extention {
            Zip => decompress_zip(file_name),
            Tar => decompress_tar(file_name),
            TarGz => decompress_tar_gz(file_name),
            TarBz2 => decompress_tar_bz2(file_name),
            TarXz => decompress_tar_xz(file_name),
            Other(ext) => Err(anyhow!(format!("The extension '{}' is not supported", ext))),
        }
    } else {
        Err(anyhow!(format!(
            "The file '{}' must have an extension",
            file_name
        )))
    }
}

fn decompress_zip(file_name: &str) -> Result<()> {
    spawn_command("unzip", &[file_name])?;
    Ok(())
}

fn decompress_tar(file_name: &str) -> Result<()> {
    spawn_command("tar", &["-xvf", file_name])?;
    Ok(())
}

fn decompress_tar_gz(file_name: &str) -> Result<()> {
    spawn_command("tar", &["-zxvf", file_name])?;
    Ok(())
}

fn decompress_tar_bz2(file_name: &str) -> Result<()> {
    spawn_command("tar", &["-jxvf", file_name])?;
    Ok(())
}

fn decompress_tar_xz(file_name: &str) -> Result<()> {
    spawn_command("tar", &["-Jxvf", file_name])?;
    Ok(())
}

fn spawn_command(program: &str, args: &[&str]) -> Result<()> {
    Command::new(program)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;
    Ok(())
}
