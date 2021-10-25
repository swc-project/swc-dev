use self::cargo::BaseCargoBuildCommand;
use crate::util::{
    cargo::{get_default_cargo_target, swc_output_dir},
    node::create_npm_package,
};
use anyhow::{bail, Context, Error};
use rayon::prelude::*;
use std::fs::{copy, create_dir_all};
use structopt::StructOpt;
use swc_node_arch::PlatformDetail;
use tracing::{debug, error, info};

mod cargo;

/// Build plugin packages.
///
/// THis command generates a file named `plugin-name.platform.swc-pkg.tar.gz`.
#[derive(Debug, StructOpt)]
pub struct BuildCommand {
    #[structopt(flatten)]
    pub cargo: BaseCargoBuildCommand,
}

impl BuildCommand {
    pub fn run(self) -> Result<(), Error> {
        let output_base = swc_output_dir()?;

        let platform = match self.cargo.target.clone() {
            Some(v) => v,
            None => get_default_cargo_target()?,
        };
        let p: PlatformDetail = platform.parse().context("failed to parse platform")?;

        let libs = self.cargo.run()?;

        let build_dir = output_base.join("build");
        create_dir_all(&build_dir)?;

        let results = libs
            .into_par_iter()
            .map(|lib| -> Result<_, Error> {
                let cdylib_ext = lib
                    .cdylib_path
                    .extension()
                    .expect("cdylib should have extension");
                let name = format!(
                    "{}.{}.{}",
                    lib.crate_name,
                    p.platform_arch_abi,
                    cdylib_ext.to_string_lossy()
                );
                let copied_path = build_dir.join(&name);

                copy(&lib.cdylib_path, &copied_path).context("failed to copy file")?;

                debug!(
                    "Copying {} to {}",
                    lib.cdylib_path.display(),
                    copied_path.display()
                );

                Ok(())
            })
            .collect::<Vec<_>>();

        let mut error = false;
        for result in results {
            match result {
                Ok(..) => {}
                Err(err) => {
                    error = true;
                    error!("failed to copy plugin: {:?}", err);
                }
            }
        }
        if error {
            bail!("failed to copy plugin");
        }

        info!("Built files are copied to {}", build_dir.display());

        create_npm_package(&build_dir).context("npm package failed")?;
        info!("Built files are copied to {}", build_dir.display());

        Ok(())
    }
}
