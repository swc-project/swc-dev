use crate::util::find_executable;
use anyhow::{anyhow, Context, Error};
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub mod platform;

/// Returned path is path to the built (and compressed) npm package file.
pub fn create_npm_package(cwd: &Path) -> Result<PathBuf, Error> {
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
        .context("failed to spawn `npm package`")?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(cwd.join(stdout.trim()))
}
