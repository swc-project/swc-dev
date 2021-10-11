use super::build::BaseCargoBuildCommand;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct PackageCommand {
    #[structopt(flatten)]
    pub cargo: BaseCargoBuildCommand,
}
