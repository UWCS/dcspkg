use std::path::PathBuf;

use serde::{Deserialize, Serialize};

mod commands;
pub mod config;
mod util;

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

/// Returns a vector containing a list of packages that are available
/// for installation from the dcspkg server.
pub fn list_all_packages(server_url: String) -> anyhow::Result<Vec<Package>> {
    list(server_url)
}

/// Returns a vector containing a list of packages that are currently
/// installed.
pub fn list_installed_packages(registry_file: &PathBuf) -> anyhow::Result<Vec<Package>> {
    get_registry(registry_file)
}

/// Installs the specified package locally.
pub fn install_package(
    server_url: String,
    install_dir: PathBuf,
    bin_dir: PathBuf,
    registry_file: PathBuf,
    package: &String,
) -> anyhow::Result<()> {
    install(package, server_url, install_dir, bin_dir, registry_file)
}

/// Launches the specified package. This exits the current process
/// and launches the package in its place.
pub fn run_package(
    registry_file: &PathBuf,
    install_dir: PathBuf,
    package: &String,
) -> anyhow::Result<()> {
    run(registry_file, install_dir, package)
}

const DATA_ENDPOINT: &str = "/pkgdata";
const FILE_ENDPOINT: &str = "/download";
const LIST_ENDPOINT: &str = "/list";
