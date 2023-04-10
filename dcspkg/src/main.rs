use anyhow::Context;
use clap::Parser;
use env_logger::Env;

use dcspkg::*;
mod cli;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    let log_level = match cli.verbose {
        0 => "error",
        1 => "warn",
        2 => "info",
        3 => "debug",
        _ => "trace",
    };

    env_logger::Builder::from_env(Env::default().default_filter_or(log_level)).init();

    //load the application config

    //create the dcspkg directory
    std::fs::create_dir_all(&*config::DCSPKG_DIR)?;

    //load config
    let config = config::DcspkgConfig::get()?;

    //create registry file if not exist
    if !config.registry.registry_file.is_file() {
        std::fs::write(&config.registry.registry_file, "[]")
            .context("Could not create empty package registry file")?;
    }

    cli.command.run(config)
}
