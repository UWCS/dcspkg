use anyhow::Context;
use clap::Parser;
use env_logger::Env;

mod cli;
mod commands;
mod config;
mod util;

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
    let config = crate::config::DcspkgConfig::get()?;

    //create the dcspkg directory and registry file
    std::fs::create_dir_all(&*crate::config::DCSPKG_DIR)?;
    std::fs::write(&config.registry.registry_file, "[]")
        .context("Could not create empty package registry file")?;

    cli.command.run(config)
}

const DATA_ENDPOINT: &str = "/pkgdata";
const FILE_ENDPOINT: &str = "/download";
const LIST_ENDPOINT: &str = "/list";
