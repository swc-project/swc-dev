use crate::util::find_executable;
use anyhow::{anyhow, bail, Context, Result};
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use tracing::info;

pub mod platform;

/// Returned path is path to the built (and compressed) npm package file.
pub fn create_npm_package(cwd: &Path) -> Result<PathBuf> {
    let npm_path =
        find_executable("npm").ok_or_else(|| anyhow!("failed to find `npm` from path"))?;

    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(&npm_path);
        c
    } else {
        Command::new(npm_path)
    };

    cmd.current_dir(&cwd);
    cmd.arg("pack");

    let output = cmd
        .stderr(Stdio::inherit())
        .output()
        .context("failed to spawn `npm pack`")?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(cwd.join(stdout.trim()))
}

pub fn publish_dir_to_npm(dir: &Path, access: Option<&str>) -> Result<()> {
    info!("Publishing node package at {}", dir.display());

    let npm_path =
        find_executable("npm").ok_or_else(|| anyhow!("failed to find `npm` from path"))?;

    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(&npm_path);
        c
    } else {
        Command::new(npm_path)
    };
    cmd.current_dir(&dir);

    cmd.arg("publish");
    if let Some(access) = access {
        cmd.arg("--access").arg(access);
    }

    let output = cmd.status().context("failed to spawn `npm publish`")?;

    if !output.success() {
        bail!("`npm publish` for `{}` failed", dir.display())
    }

    Ok(())
}

pub fn publish_tarball_to_npm(path: &Path, access: Option<&str>) -> Result<()> {
    info!("Publishing platform package at {}", path.display());

    let npm_path =
        find_executable("npm").ok_or_else(|| anyhow!("failed to find `npm` from path"))?;

    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(&npm_path);
        c
    } else {
        Command::new(npm_path)
    };

    cmd.arg("publish").arg(&path);
    if let Some(access) = access {
        cmd.arg("--access").arg(access);
    }

    let output = cmd.status().context("failed to spawn `npm publish`")?;

    if !output.success() {
        bail!("`npm publish` for `{}` failed", path.display())
    }

    Ok(())
}
