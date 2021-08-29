use crate::{get_extention, Ext::*};
use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn decompress(file_path: &Path) -> Result<()> {
    if !file_path.exists() {
        return Err(anyhow!(format!(
            "The file '{}' does not exist",
            file_path.to_str().unwrap_or_default()
        )));
    }

    match get_extention(file_path) {
        Zip => decompress_zip(file_path),
        Gz => decompress_gz(file_path),
        Z => decompress_z(file_path),
        Bz2 => decompress_bz2(file_path),
        Tar => decompress_tar(file_path),
        TarGz => decompress_tar_gz(file_path),
        TarZ => decompress_tar_z(file_path),
        TarBz2 => decompress_tar_bz2(file_path),
        TarXz => decompress_tar_xz(file_path),
        Other(extention) => Err(anyhow!(format!(
            "The extension '{}' is not supported",
            extention
        ))),
    }
}

fn decompress_zip(file_path: &Path) -> Result<()> {
    spawn_command("unzip", &[file_path.to_str().unwrap_or_default()])?;
    Ok(())
}

fn decompress_gz(file_path: &Path) -> Result<()> {
    spawn_command("gunzip", &[file_path.to_str().unwrap_or_default()])?;
    Ok(())
}

fn decompress_z(file_path: &Path) -> Result<()> {
    spawn_command("uncompress", &[file_path.to_str().unwrap_or_default()])?;
    Ok(())
}

fn decompress_bz2(file_path: &Path) -> Result<()> {
    spawn_command("bunzip2", &[file_path.to_str().unwrap_or_default()])?;
    Ok(())
}

fn decompress_tar(file_path: &Path) -> Result<()> {
    spawn_command("tar", &["-xvf", file_path.to_str().unwrap_or_default()])?;
    Ok(())
}

fn decompress_tar_gz(file_path: &Path) -> Result<()> {
    spawn_command("tar", &["-zxvf", file_path.to_str().unwrap_or_default()])?;
    Ok(())
}

fn decompress_tar_z(file_path: &Path) -> Result<()> {
    spawn_command("tar", &["-Zxvf", file_path.to_str().unwrap_or_default()])?;
    Ok(())
}

fn decompress_tar_bz2(file_path: &Path) -> Result<()> {
    spawn_command("tar", &["-jxvf", file_path.to_str().unwrap_or_default()])?;
    Ok(())
}

fn decompress_tar_xz(file_path: &Path) -> Result<()> {
    spawn_command("tar", &["-Jxvf", file_path.to_str().unwrap_or_default()])?;
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
