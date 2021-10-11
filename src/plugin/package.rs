use crate::util::{cargo::swc_build_dir, node::platform::all_node_platforms};

use super::build::BaseCargoBuildCommand;
use anyhow::Error;
use indexmap::IndexSet;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct PackageCommand {
    #[structopt(flatten)]
    pub cargo: BaseCargoBuildCommand,

    /// If specified, the package will contains binaries only for the specified platforms.
    ///
    ///
    #[structopt(long)]
    pub only_platforms: Option<Vec<String>>,
}

impl PackageCommand {
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

        let plugin_names = libs
            .iter()
            .map(|v| v.crate_name.clone())
            .collect::<IndexSet<_, ahash::RandomState>>();

        let pkgs_dir = build_dir.join("pkgs");

        for platform in platforms {
            for name in &plugin_names {
                let pkg_dir = pkgs_dir.join(format!("{}-{}", name, platform));

                dbg!(&pkg_dir);
            }
        }

        dbg!(&libs);

        Ok(())
    }
}
