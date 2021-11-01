extern crate swc_node_base;

use anyhow::Error;
use plugin::PluginCommand;
use std::time::Instant;
use structopt::StructOpt;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod plugin;
mod util;

#[derive(Debug, StructOpt)]

pub enum Cmd {
    Plugin(PluginCommand),
}

fn main() -> Result<(), Error> {
    let logger = tracing_subscriber::FmtSubscriber::builder()
        .without_time()
        .with_target(false)
        .with_ansi(true)
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(logger)?;

    let cmd = Cmd::from_args();

    let start = Instant::now();
    match cmd {
        Cmd::Plugin(cmd) => {
            cmd.run()?;
        }
    }
    info!("Done in {:?}", start.elapsed());

    Ok(())
}
