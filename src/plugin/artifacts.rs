use crate::util::cargo::get_all_crates;
use anyhow::Error;
use structopt::StructOpt;

/// Patch package.json to include `optionalDependencies`
#[derive(Debug, StructOpt)]
pub struct ArtifactsCommand {}

impl ArtifactsCommand {
    pub fn run(&self) -> Result<(), Error> {
        let crates = get_all_crates()?;

        for (crate_name, manifest_dir) in crates {}

        todo!()
    }
}
