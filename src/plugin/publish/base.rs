use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct BasePublishCommand {
    /// Defaults whole crates.
    #[structopt(long)]
    pub crates: Vec<String>,

    #[structopt(long)]
    pub access: Option<String>,
}
