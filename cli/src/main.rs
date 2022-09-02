use clap::Parser;
use env_logger::Env;

mod cli;
mod config;

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

    match cli.command {
        cli::Command::List => dcspkg_client::list()?
            .into_iter()
            .for_each(|p| println!("{p}")),
        cli::Command::Install { package } => dcspkg_client::install(&package, config::SERVER_URL)?,
    }

    Ok(())
}
