use crate::config::DcspkgConfig;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    /// Set the verbosity level
    #[clap(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    ///List all packages available for install
    List,
    /// Install a package
    Install { package: String },
}

impl Command {
    pub fn run(&self, config: DcspkgConfig) -> anyhow::Result<()> {
        use Command::*;
        match &self {
            List => dcspkg_client::list(config.server.url)
                .map(|v| v.into_iter().for_each(|p| println!("{p:?}"))),
            Install { package } => dcspkg_client::install(
                package,
                config.server.url,
                config.registry.install_dir,
                config.registry.bin_dir,
                config.registry.registry_file,
            ),
        }
    }
}
