use crate::commands::*;
use crate::config::DcspkgConfig;
use crate::util::*;
use anyhow::Context;
use clap::{Parser, Subcommand};
use std::os::unix::process::CommandExt;

//clap stuff

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
        json: bool,
    },
    /// Install a package
    Install { package: String },
    ///Show all installed packages and their versions
    Installed {
        #[clap(long, short, action)]
        json: bool,
    },
    ///Run the executable from the package specified
    Run { package: String },
}

//where the cli opts are dispatched to functions
impl Command {
    pub fn run(&self, config: DcspkgConfig) -> anyhow::Result<()> {
        use Command::*;
        match &self {
            //list all the packages to stdout
            List { json } => {
                let packages = list(config.server.url)?;
                print_package_list(&packages, *json).context("Cannot format package list")
            }
            //install a package
            Install { package } => install(
                package,
                config.server.url,
                config.registry.install_dir,
                config.registry.bin_dir,
                config.registry.registry_file,
            ),

            //list what we have installed
            Installed { json } => {
                let packages = get_registry(&config.registry.registry_file)?;
                print_package_list(&packages, *json).context("Cannot format package list")
            }

            //run an executable from a package
            Run { package } => {
                let package_data = get_registry(&config.registry.registry_file)?
                    .into_iter()
                    .find(|pkg| pkg.pkgname == *package)
                    .context(format!(
                        "Could not find a package with the name {} in {:?}",
                        package, config.registry.registry_file
                    ))?;

                let exe_path = config.registry.install_dir.join(package_data.pkgname).join(
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
