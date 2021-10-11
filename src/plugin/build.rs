use self::cargo::BaseCargoBuildCommand;
use crate::util::cargo::swc_build_dir;
use anyhow::Error;
use indexmap::IndexSet;
use structopt::StructOpt;

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

        let libs = self.cargo.run()?;

        let crate_names = libs
            .iter()
            .map(|v| v.crate_name.clone())
            .collect::<IndexSet<_, ahash::RandomState>>();

        let pkgs_dir = build_dir.join("pkgs");

        dbg!(&libs);

        Ok(())
    }
}
