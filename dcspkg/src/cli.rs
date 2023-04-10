use crate::config::DcspkgConfig;
use crate::util::*;
use crate::{install_package, list_all_packages, run_package};
use clap::{Parser, Subcommand};

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
                let packages = list_all_packages(config.server.url)?;
                print_package_list(&packages, *json);
                Ok(())
            }

            //install a package
            Install { package } => install_package(
                package,
                config.server.url,
                config.registry.install_dir,
                config.registry.bin_dir,
                config.registry.registry_file,
            ),

            //list what we have installed
            Installed { json } => {
                let packages = list_installed_packages(&config.registry.registry_file)?;
                print_package_list(&packages, *json);
                Ok(())
            }

            //run an executable from a package
            Run { package } => run_package(
                &config.registry.registry_file,
                config.registry.install_dir,
                package,
            ),
        }
    }
}
