use crate::{get_extention, Ext::*};
use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn decompress(file_name: &str) -> Result<()> {
    if !Path::new(file_name).exists() {
        return Err(anyhow!(format!("The file '{}' does not exist", file_name)));
    }

    match get_extention(file_name) {
        Zip => decompress_zip(file_name),
        Gz => decompress_gz(file_name),
        Z => decompress_z(file_name),
        Bz2 => decompress_bz2(file_name),
        Tar => decompress_tar(file_name),
        TarGz => decompress_tar_gz(file_name),
        TarZ => decompress_tar_z(file_name),
        TarBz2 => decompress_tar_bz2(file_name),
        TarXz => decompress_tar_xz(file_name),
        Other => Err(anyhow!(format!(
            "The extension of the file '{}' is not supported",
            file_name
        ))),
    }
}

fn decompress_zip(file_name: &str) -> Result<()> {
    spawn_command("unzip", &[file_name])?;
    Ok(())
}

fn decompress_gz(file_name: &str) -> Result<()> {
    spawn_command("gunzip", &[file_name])?;
    Ok(())
}

fn decompress_z(file_name: &str) -> Result<()> {
    spawn_command("uncompress", &[file_name])?;
    Ok(())
}

fn decompress_bz2(file_name: &str) -> Result<()> {
    spawn_command("bunzip2", &[file_name])?;
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

fn decompress_tar_z(file_name: &str) -> Result<()> {
    spawn_command("tar", &["-Zxvf", file_name])?;
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
