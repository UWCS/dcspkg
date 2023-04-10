use serde::{Deserialize, Serialize};

mod commands;
pub mod config;
pub mod util;

pub use crate::commands::{list_all_packages, install_package, run_package};

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

const DATA_ENDPOINT: &str = "/pkgdata";
const FILE_ENDPOINT: &str = "/download";
const LIST_ENDPOINT: &str = "/list";
