use crate::util::cargo::add::run_cargo_add;
use anyhow::{bail, Context, Error};
use std::process::{Command, Stdio};
use structopt::StructOpt;

/// Initializes a plugin project.
#[derive(Debug, StructOpt)]
pub struct InitCommand {}

impl InitCommand {
    pub fn run(self) -> Result<(), Error> {
        let mut c = Command::new("cargo");
        c.arg("init").stderr(Stdio::inherit());

        let status = c
            .arg("--lib")
            .status()
            .with_context(|| format!("failed to run `cargo init`"))?;

        if !status.success() {
            bail!("failed to initialize a cargo project")
        }

        run_cargo_add("abi_stable")?;
        run_cargo_add("swc_atoms")?;
        run_cargo_add("swc_common")?;
        run_cargo_add("swc_plugin")?;

        Ok(())
    }
}
