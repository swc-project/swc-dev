use self::{cargo::BaseCargoBuildCommand, package_json::PackageJsonForBin};
use crate::util::{cargo::swc_build_dir, node::platform::all_node_platforms};
use anyhow::{Context, Error};
use indexmap::IndexSet;
use std::path::Path;
use structopt::StructOpt;
use swc_node_arch::PlatformDetail;
use tracing::debug;

mod cargo;
mod package_json;

/// Build plugin packages.
#[derive(Debug, StructOpt)]
pub struct BuildCommand {
    #[structopt(flatten)]
    pub cargo: BaseCargoBuildCommand,

    /// If specified, the package will contains binaries only for the specified platforms.
    ///
    ///
    #[structopt(long)]
    pub only_platforms: Option<Vec<String>>,
}

impl BuildCommand {
    pub async fn run(self) -> Result<(), Error> {
        let build_dir = swc_build_dir().await?;

        let platforms = if let Some(only) = &self.only_platforms {
            only.clone()
        } else {
            all_node_platforms()
                .into_iter()
                .map(|v| v.to_string())
                .collect()
        };

        let libs = self.cargo.run().await?;

        let crate_names = libs
            .iter()
            .map(|v| v.crate_name.clone())
            .collect::<IndexSet<_, ahash::RandomState>>();

        let pkgs_dir = build_dir.join("pkgs");

        for platform in platforms {
            for crate_name in &crate_names {
                create_package_for_platform(&pkgs_dir, &crate_name, &platform)?;
            }
        }

        dbg!(&libs);

        Ok(())
    }
}

#[tracing::instrument(name = "build_node_package", skip(pkgs_dir))]
fn create_package_for_platform(
    pkgs_dir: &Path,
    crate_name: &str,
    platform: &str,
) -> Result<(), Error> {
    debug!("Creating a package for a platform");

    let pkg_dir = pkgs_dir.join(format!("{}-{}", crate_name, platform));
    let platform_detail: PlatformDetail = platform.parse().context("invalid platform")?;

    // let package_json = PackageJsonForBin {
    //     name: crate_name.to_string(),
    // };

    dbg!(&pkg_dir);

    Ok(())
}
