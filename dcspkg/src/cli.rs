use crate::config::DcspkgConfig;
use crate::util::print_package_list;
use anyhow::Context;
use clap::{Parser, Subcommand};
use dcspkg_client::Package;
use std::os::unix::process::CommandExt;
use std::path::Path;

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
    List {
        #[clap(long, short, action)]
        json: bool
    },
    /// Install a package
    Install { package: String },
    ///Show all installed packages and their versions
    Installed {
        #[clap(long, short, action)]
        json: bool
    },
    ///Run the executable from the package specified
    Run { package: String },
}

impl Command {
    pub fn run(&self, config: DcspkgConfig) -> anyhow::Result<()> {
        use Command::*;
        match &self {
            List { json } => {
                let packages = dcspkg_client::list(config.server.url)?;
                print_package_list(&packages, *json).context("Cannot format package list")
            }
            Install { package } => dcspkg_client::install(
                package,
                config.server.url,
                config.registry.install_dir,
                config.registry.bin_dir,
                config.registry.registry_file,
            ),
            Installed { json } => {
                let packages = &get_registry(&config.registry.registry_file)?;
                print_package_list(packages, *json).context("Cannot format package list")
            }
            Run { package } => {
                let package_data = get_registry(&config.registry.registry_file)?
                    .into_iter()
                    .find(|pkg| pkg.name == *package)
                    .context(format!(
                        "Could not find a package with the name {} in {:?}",
                        package, config.registry.registry_file
                    ))?;

                let exe_path = config.registry.install_dir.join(package_data.name).join(
                    package_data
                        .executable_path
                        .context("No executable exists for this package")?,
                );

                //will only return if there is an error
                Err(std::process::Command::new(exe_path).exec()).map_err(Into::into)
            }
        }
    }
}

fn get_registry(path: &Path) -> anyhow::Result<Vec<Package>> {
    std::fs::File::open(path)
        .context("Could not find registry file")
        .and_then(|reader| {
            serde_json::from_reader(reader).context("Could not parse JSON from registry")
        })
}
