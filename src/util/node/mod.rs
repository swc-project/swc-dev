use anyhow::{Context, Error};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub mod platform;

/// Returned path is path to the built (and compressed) npm package file.
pub fn create_npm_package(cwd: &Path) -> Result<PathBuf, Error> {
    let mut cmd = Command::new("npm");
    cmd.current_dir(&cwd);
    cmd.arg("pack");

    let status = cmd.status().context("failed to spawn `npm package`")?;

    panic!("Not implemented yet")
}
