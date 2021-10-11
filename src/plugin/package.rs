use crate::util::{cargo::swc_build_dir, node::platform::all_node_platforms};

use super::build::BaseCargoBuildCommand;
use anyhow::Error;
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
        let libs = self.cargo.run().await?;

        let platforms = if let Some(only) = &self.only_platforms {
            only.clone()
        } else {
            all_node_platforms()
                .into_iter()
                .map(|v| v.to_string())
                .collect()
        };

        let pkgs_dir = build_dir.join("pkgs");

        for platform in platforms {
            let pkg_dir = pkgs_dir.join(&platform);

            dbg!(&pkg_dir);
        }

        dbg!(&libs);

        Ok(())
    }
}
