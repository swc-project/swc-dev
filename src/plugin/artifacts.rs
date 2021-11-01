use crate::util::{cargo::get_all_crates, node::publish_tarball_to_npm, AHashMap};
use anyhow::{bail, Context, Result};
use serde_json::Value;
use std::{
    env::current_dir,
    fs::{self, read_dir, read_to_string},
    path::{Path, PathBuf},
    str::FromStr,
};
use structopt::StructOpt;
use tracing::info;

/// Publish package for each platforms and patch package.json to include
/// `optionalDependencies`
#[derive(Debug, StructOpt)]
pub struct PublishDepsCommand {
    /// Defaults whole crates.
    #[structopt(long)]
    pub crates: Vec<String>,

    /// Defaults to current working directory.
    #[structopt(long, default_value)]
    pub artifacts_dir: String,

    #[structopt(long)]
    pub access: Option<String>,
}

impl PublishDepsCommand {
    pub fn run(self) -> Result<()> {
        let artifacts_dir = if self.artifacts_dir.is_empty() {
            current_dir().context("failed to get current directory")?
        } else {
            PathBuf::from(self.artifacts_dir)
        };

        let enabled_crates = self.crates;
        let access = self.access;

        let crates = get_all_crates()?
            .into_iter()
            .filter(|(name, _)| enabled_crates.is_empty() || enabled_crates.contains(&name))
            .collect::<AHashMap<_, _>>();

        let all_pkg_platforms =
            get_swc_pkg_files(&artifacts_dir).context("failed to get swc package files")?;

        info!("Using {:?}", all_pkg_platforms);

        for (crate_name, manifest_dir) in crates {
            let base_package_json_path = manifest_dir.join("package.json");
            assert!(
                base_package_json_path.is_file(),
                "package.json for crate `{}` is not found",
                crate_name
            );

            let base_package_json_str = read_to_string(&base_package_json_path).context(
                format!("failed to read `{}`", base_package_json_path.display()),
            )?;

            let mut base_package_json = Value::from_str(&base_package_json_str)?;
            if !base_package_json.is_object() {
                bail!("package.json is not an object")
            }

            let pkg_platforms = all_pkg_platforms.get(&crate_name).with_context(|| {
                format!("failed to get package files for crate `{}`", crate_name)
            })?;

            {
                let pkg_json_obj = base_package_json.as_object_mut().unwrap();

                if !pkg_json_obj.contains_key("optionalDependencies") {
                    pkg_json_obj.insert(
                        "optionalDependencies".to_string(),
                        Value::Object(Default::default()),
                    );
                }

                let pkg_name = pkg_json_obj
                    .get("name")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();
                let pkg_version = pkg_json_obj
                    .get("version")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();

                let opt_deps = pkg_json_obj
                    .get_mut("optionalDependencies")
                    .unwrap()
                    .as_object_mut()
                    .unwrap();

                for platform in pkg_platforms.iter() {
                    let dep_name = format!("{}-{}", pkg_name, platform);
                    if !opt_deps.contains_key(&dep_name) {
                        opt_deps.insert(dep_name, Value::String(pkg_version.clone()));
                    }
                }
            }
            let pkg_json_str = serde_json::to_string_pretty(&base_package_json)?;

            fs::write(&base_package_json_path, &pkg_json_str).context(format!(
                "failed to write to `{}`",
                base_package_json_path.display()
            ))?;

            for platform in pkg_platforms.iter() {
                let platform_pkg_filename = format!("{}.{}.swc-pkg.tgz", crate_name, platform);

                publish_tarball_to_npm(
                    &artifacts_dir.join(&platform_pkg_filename),
                    access.as_deref(),
                )
                .with_context(|| {
                    format!(
                        "failed to publish platform package for `{}` (target = {})",
                        crate_name, platform
                    )
                })?;
            }
        }


        Ok(())
    }
}

/// Key is crate name and values are platforms
fn get_swc_pkg_files(artifacts_dir: &Path) -> Result<AHashMap<String, Vec<String>>> {
    let entries = read_dir(&artifacts_dir)?;

    let mut buf = AHashMap::<_, Vec<_>>::default();

    for e in entries {
        let e = e?;
        let path = e.path();

        if path.to_string_lossy().ends_with(".swc-pkg.tgz") {
            let file_name = path.file_name().unwrap().to_string_lossy();

            assert_eq!(
                file_name.split('.').count(),
                4,
                "The plugin artifact should be named `<crate_name>.<platform_name>.swc-pkg.tgz`"
            );
            let crate_name = file_name.split('.').next().unwrap().to_string();
            let platform = file_name.split('.').nth(1).unwrap().to_string();

            buf.entry(crate_name).or_default().push(platform);
        }
    }

    Ok(buf)
}
