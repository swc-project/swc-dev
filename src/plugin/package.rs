use crate::{
    plugin::package::package_json::PackageJsonForBin,
    util::{
        cargo::{get_cargo_manifest_path, swc_build_dir},
        node::platform::all_node_platforms,
    },
};
use anyhow::{bail, Context, Error};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::Path,
};
use structopt::StructOpt;
use tracing::info;

mod package_json;

/// Build plugin packages.
#[derive(Debug, StructOpt)]
pub struct PackageCommand {
    /// Crates to package.
    pub crates: Vec<String>,

    /// If specified, the package will contains binaries only for the specified
    /// platforms.
    #[structopt(long)]
    pub platforms: Option<Vec<String>>,
}

impl PackageCommand {
    pub fn run(self) -> Result<(), Error> {
        let build_dir = swc_build_dir()?;

        let crate_names = self.crates.clone();

        let platforms = if let Some(only) = &self.platforms {
            only.clone()
        } else {
            all_node_platforms()
                .into_iter()
                .map(|v| v.to_string())
                .collect()
        };

        let pkgs_dir = build_dir.join("pkgs");

        for platform in platforms {
            for crate_name in &crate_names {
                create_package_for_platform(&pkgs_dir, &crate_name, &platform)?;
            }
        }

        Ok(())
    }
}

#[tracing::instrument(name = "build_node_package", skip(pkgs_dir))]
pub fn create_package_for_platform(
    pkgs_dir: &Path,
    crate_name: &str,
    platform: &str,
) -> Result<(), Error> {
    info!("Creating a package for a platform");

    let pkg_dir = pkgs_dir.join(format!("{}-{}", crate_name, platform));
    // let platform_detail: PlatformDetail = platform.parse().context("invalid
    // platform")?;

    create_dir_all(&pkg_dir).with_context(|| {
        format!(
            "failed to create `{}` which is required to create a binary package for `{}`",
            pkg_dir.display(),
            platform
        )
    })?;

    let manifest_path = get_cargo_manifest_path(crate_name.to_string())
        .context("failed to get the path of cargo manifest")?;
    let manifest_dir = manifest_path.parent().unwrap();
    let package_json_path = manifest_dir.join("package.json");

    if !package_json_path.is_file() {
        bail!(
            "Plugin `{}` should have package.json in `{}`",
            crate_name,
            manifest_dir.display()
        )
    }

    let package_json_str = read_to_string(&package_json_path)?;

    let mut bin_pkg_json: PackageJsonForBin = serde_json::from_str(&package_json_str)
        .with_context(|| {
            format!(
                "failed to create the package.json file for platorm package from the main \
                 package.json file at {}",
                package_json_path.display()
            )
        })?;
    let main_name = bin_pkg_json.name.clone();

    bin_pkg_json.name = format!("{}-{}", bin_pkg_json.name, platform);
    bin_pkg_json.description = format!(
        "This package is part of {}. This packaged is installed only for `{}`.",
        main_name, platform
    );

    // let package_json = PackageJsonForBin {
    //     name: crate_name.to_string(),
    // };

    let bin_json_path = pkg_dir.join("package.json");
    let bin_pkg_json = serde_json::to_string_pretty(&bin_pkg_json)
        .context("failed to serialize package.json file for the binary package")?;
    write(&bin_json_path, &bin_pkg_json).with_context(|| {
        format!(
            "failed to write package.json file to `{}`",
            bin_json_path.display()
        )
    })?;

    dbg!(&pkg_dir);

    Ok(())
}
