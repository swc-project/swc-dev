use crate::{
    plugin::package::package_json::PackageJsonForBin,
    util::{
        cargo::{get_all_crates, get_cargo_manifest_path, swc_output_dir},
        node::platform::{all_node_platforms, parse_node_platform},
    },
};
use anyhow::{bail, Context, Error};
use rayon::prelude::*;
use std::{
    fs::{copy, create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
    sync::Arc,
};
use structopt::StructOpt;
use swc_node_arch::PlatformDetail;
use tracing::{debug, error, info};

mod package_json;

/// Build plugin packages.
#[derive(Debug, StructOpt)]
pub struct PackageCommand {
    /// Crates to package.
    #[structopt(long)]
    pub crates: Vec<String>,

    /// If specified, the package will contains binaries only for the specified
    /// platforms. If not specified, all platforms will be used.
    #[structopt(long)]
    pub platforms: Option<Vec<String>>,
}

impl PackageCommand {
    pub fn run(self) -> Result<(), Error> {
        let output_base = swc_output_dir()?;

        let crate_names = self.crates.clone();

        let crate_names = if crate_names.is_empty() {
            info!("Using all crates in the workspace because `--crates` is not used");
            get_all_crates()?.into_iter().map(|c| c.0).collect()
        } else {
            crate_names
        };

        let platforms = if let Some(only) = &self.platforms {
            only.iter()
                .map(|s| parse_node_platform(s).unwrap())
                .collect()
        } else {
            all_node_platforms()
        };

        let build_dir = Arc::new(output_base.join("build"));
        let pkgs_dir = Arc::new(output_base.join("pkgs"));

        let results = platforms
            .par_iter()
            .cloned()
            .flat_map(|platform| {
                let build_dir = build_dir.clone();
                let pkgs_dir = pkgs_dir.clone();
                crate_names.par_iter().map(move |crate_name| {
                    create_package_for_platform(&pkgs_dir, &build_dir, &crate_name, &platform)
                        .context("failed to create a package for platform")
                })
            })
            .collect::<Vec<_>>();

        let mut error = false;
        for result in results {
            match result {
                Ok(..) => {}
                Err(err) => {
                    error = true;
                    error!("{:?}", err);
                }
            }
        }

        if error {
            bail!("failed to create packages");
        }

        Ok(())
    }
}

/// Returns the package directory.
#[tracing::instrument(name = "build_node_package", skip(pkgs_dir, build_dir))]
pub(super) fn create_package_for_platform(
    pkgs_dir: &Path,
    build_dir: &Path,
    crate_name: &str,
    platform: &PlatformDetail,
) -> Result<PathBuf, Error> {
    info!("Creating a package for a platform");

    let pkg_dir = pkgs_dir.join(format!("{}-{}", crate_name, platform));
    let built_bin_path = build_dir.join(format!(
        "{}.{}{}",
        crate_name,
        platform,
        platform.platform.cdylib_ext()
    ));

    create_dir_all(&pkg_dir).with_context(|| {
        format!(
            "failed to create `{}` which is required to create a binary package for `{}`",
            pkg_dir.display(),
            platform
        )
    })?;

    if !built_bin_path.is_file() {
        bail!(
            "failed to find built dynamic library from `{}`",
            built_bin_path.display()
        )
    }
    debug!(
        "Using the dynamic library at `{}`",
        built_bin_path.display()
    );
    let dylib_filename = format!("lib{}", platform.platform.cdylib_ext());
    let bin_path = pkg_dir.join(&dylib_filename);
    copy(&built_bin_path, &bin_path).with_context(|| {
        format!(
            "failed to copy built binary file ({}) to package ({})",
            built_bin_path.display(),
            bin_path.display(),
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
                "failed to create the package.json file for platform package from the main \
                 package.json file at {}",
                package_json_path.display()
            )
        })?;
    let main_name = bin_pkg_json.name.clone();

    bin_pkg_json.name = format!("{}-{}", bin_pkg_json.name, platform);
    bin_pkg_json.description = format!(
        "This package is part of {}. This package will be installed only for `{}`.",
        main_name, platform
    );

    bin_pkg_json.os.push(platform.platform);
    bin_pkg_json.cpu.push(platform.arch);

    bin_pkg_json.files.push(dylib_filename.clone());
    bin_pkg_json.main = dylib_filename;

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

    Ok(pkg_dir)
}
