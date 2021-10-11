use anyhow::{bail, Context, Error};
use std::process::Command;

pub fn run_cargo_add(crate_name: &str) -> Result<(), Error> {
    let status = Command::new("cargo")
        .arg("add")
        .arg(crate_name)
        .status()
        .with_context(|| format!("failed to run `cargo add {}`", crate_name))?;
    if status.success() {
        Ok(())
    } else {
        bail!("failed to run `cargo add {}`", crate_name)
    }
}
