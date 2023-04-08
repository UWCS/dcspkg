use serde::{Deserialize, Serialize};



pub mod commands;
pub mod cli;
pub mod config;
pub mod util;

use crate::config::DcspkgConfig;
use crate::commands::*;
use crate::util::*;

/// Represents a package, and contains all the metadata assoicated with it.
#[derive(Deserialize, Default, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Package {
    /// The package's name, ie "gcc"
    /// This is the primary key
    pub pkgname: String,
    /// The game/app's full name/title, ie "The GNU Compiler Collection, Version 4.3"
    pub fullname: String,
    /// A short description of the package
    pub description: Option<String>,
    /// A URL pointing to an image for the package
    pub image_url: Option<String>,
    /// The relative path of the executable within the tarball
    pub executable_path: Option<String>,
    /// The package's CRC checksum
    pub crc: u32,
    /// Does the package have an install script that needs running?
    pub has_installer: bool,
    /// Does the package want to be added to path on the machine it was installed on?
    pub add_to_path: bool,
}

// Simplify an API to expose here for when this crate
// is used as a library. This also provides calls 
// for cli.rs when used from main.rs.
pub fn list_all_packages(config: DcspkgConfig) -> anyhow::Result<Vec<Package>> {
    list(config.server.url)
}

pub fn list_installed_packages(config: DcspkgConfig) -> anyhow::Result<Vec<Package>> {
    get_registry(&config.registry.registry_file)
}

pub fn install_package(config: DcspkgConfig, package: &String) -> anyhow::Result<()> {
    install(
        package,
        config.server.url,
        config.registry.install_dir,
        config.registry.bin_dir,
        config.registry.registry_file,
    )
}

pub fn run_package(config: DcspkgConfig, package: &String) -> anyhow::Result<()> {
    run(config, package)
}

const DATA_ENDPOINT: &str = "/pkgdata";
const FILE_ENDPOINT: &str = "/download";
const LIST_ENDPOINT: &str = "/list";