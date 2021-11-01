use crate::util::cargo::{cargo_target_dir, get_default_cargo_target};
use anyhow::{bail, Error};
use cargo_metadata::Message;
use std::{
    io::BufReader,
    path::PathBuf,
    process::{Command, Stdio},
};
use structopt::StructOpt;
use tracing::{debug, error, info, warn};

/// Used for commands involving `cargo build`

#[derive(Debug, StructOpt)]
pub struct BaseCargoBuildCommand {
    /// Build for production.
    #[structopt(long)]
    pub release: bool,

    /// Build one crate
    #[structopt(long)]
    pub crate_name: Vec<String>,

    /// Build all crates in a workspace.
    #[structopt(long)]
    pub all: bool,

    /// Target triple.
    #[structopt(long)]
    pub target: Option<String>,

    /// Flags to pass to cargo.
    #[structopt(long)]
    pub cargo_flags: Option<Vec<String>>,
}

impl BaseCargoBuildCommand {
    #[tracing::instrument(name = "cargo build", skip(self))]
    fn run_sync(&self) -> Result<Vec<BuiltPlugin>, Error> {
        let mut cdylibs = vec![];
        let mut cmd = Command::new("cargo");

        cmd.stdout(Stdio::piped())
            .arg("build")
            .arg("--message-format=json-render-diagnostics");

        if self.release {
            cmd.arg("--release");
        }

        if self.all {
            cmd.arg("--workspace");
        }

        for name in &self.crate_name {
            cmd.arg("--package").arg(&name);
        }

        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        for name in &self.crate_name {
            cmd.arg("--package").arg(&name);
        }

        if let Some(flags) = &self.cargo_flags {
            cmd.args(flags);
        }

        let mut cargo = cmd.spawn().unwrap();

        let target = match self.target.clone() {
            Some(v) => v,
            None => get_default_cargo_target()?,
        };

        let reader = BufReader::new(cargo.stdout.take().unwrap());
        for message in Message::parse_stream(reader) {
            let message = message?;
            match message {
                Message::CompilerMessage(msg) => {
                    println!("{:?}", msg);
                }
                Message::CompilerArtifact(artifact) => {
                    let kinds = &*artifact.target.kind;

                    // We didn't build it.
                    if kinds.len() == 1 {
                        if kinds[0] == "lib"
                            || kinds[0] == "proc-macro"
                            || artifact.target.name == "build-script-build"
                        {
                            continue;
                        }
                    }

                    if kinds.iter().any(|s| &**s == "cdylib") {
                        let paths = artifact
                            .filenames
                            .iter()
                            .filter(|s| {
                                if let Some("rlib") = s.extension() {
                                    return false;
                                }

                                true
                            })
                            .map(|v| v.to_path_buf().into_std_path_buf());

                        cdylibs.extend(paths.map(|cdylib_path| BuiltPlugin {
                            crate_name: artifact.target.name.clone(),
                            target: target.clone(),
                            cdylib_path,
                        }));
                        continue;
                    }

                    warn!("Unhandled artifact message: {:?}", artifact);
                }
                Message::BuildScriptExecuted(..) => {}
                Message::BuildFinished(finished) => {
                    if finished.success {
                        info!("`cargo build` successed")
                    } else {
                        error!("`cargo build` failed");
                    }
                }
                _ => (), // Unknown message
            }
        }

        let output = cargo.wait().expect("Couldn't get cargo's exit status");
        if !output.success() {
            bail!("failed to build plugin using cargo")
        }

        debug!("Built {:?}", cdylibs);

        Ok(cdylibs)
    }

    pub fn run(self) -> Result<Vec<BuiltPlugin>, Error> {
        let target_dir = cargo_target_dir()?;
        let target_dir_str = target_dir.to_string_lossy();
        info!(
            target_dir = &*target_dir_str,
            "Building swc plugin using cargo"
        );

        self.run_sync()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuiltPlugin {
    pub crate_name: String,
    pub target: String,
    pub cdylib_path: PathBuf,
}
