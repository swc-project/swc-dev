use super::build::BaseCargoBuildCommand;
use anyhow::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct PackageCommand {
    #[structopt(flatten)]
    pub cargo: BaseCargoBuildCommand,
}

impl PackageCommand {
    pub async fn run(self) -> Result<(), Error> {
        let libs = self.cargo.run().await?;

        Ok(())
    }
}
