use clap::Parser;
use env_logger::Env;

mod cli;

fn main() {
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
        cli::Command::List => println!("Listing packages..."),
        cli::Command::Install { package } => println!("Installing {package}!"),
    }
}
