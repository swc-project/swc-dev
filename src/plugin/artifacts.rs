use anyhow::Error;
use structopt::StructOpt;

/// Patch package.json to include `optionalDependencies`
#[derive(Debug, StructOpt)]
pub struct ArtifactsCommand {}

impl ArtifactsCommand {
    pub fn run(&self) -> Result<(), Error> {
        todo!()
    }
}
