use self::cargo::BaseCargoBuildCommand;
use crate::util::cargo::{get_default_cargo_target, swc_build_dir};
use anyhow::Error;
use indexmap::IndexSet;
use rayon::prelude::*;
use structopt::StructOpt;
use tracing::error;

mod cargo;

/// Build plugin packages.
#[derive(Debug, StructOpt)]
pub struct BuildCommand {
    #[structopt(flatten)]
    pub cargo: BaseCargoBuildCommand,
}

impl BuildCommand {
    pub fn run(self) -> Result<(), Error> {
        let build_dir = swc_build_dir()?;

        let platform = match self.cargo.target.clone() {
            Some(v) => v,
            None => get_default_cargo_target()?,
        };

        let libs = self.cargo.run()?;

        let crate_names = libs
            .iter()
            .map(|v| v.crate_name.clone())
            .collect::<IndexSet<_, ahash::RandomState>>();

        let pkgs_dir = build_dir.join("pkgs");

        crate_names.par_iter().for_each(|crate_name| {
            let res =
                super::package::create_package_for_platform(&pkgs_dir, &crate_name, &platform);

            if res.is_err() {
                error!("failed to build package for {}", crate_name)
            }
        });

        Ok(())
    }
}
