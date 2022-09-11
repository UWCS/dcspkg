//! this should eventually be replaced with some proper config struct read from a file somwhere
use config::{Config, ConfigError, Environment, File};
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

lazy_static::lazy_static! {
    pub static ref DCSPKG_DIR: PathBuf = home_dir().expect("Could not find your home directory!").join(".dcspkg");
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DcspkgConfig {
    pub server: Server,
    pub registry: Registry,
}

impl DcspkgConfig {
    pub fn get() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(
                File::with_name(
                    DCSPKG_DIR
                        .join("config")
                        .to_str()
                        .expect("Could not build config file name"),
                )
                .required(true),
            )
            .add_source(Environment::with_prefix("DCSPKG"))
            .build()
            .and_then(|c| c.try_deserialize())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub url: String,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            url: "https://dcspkg.uwcs.co.uk".to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registry {
    pub registry_file: PathBuf,
    pub install_dir: PathBuf,
    pub bin_dir: PathBuf,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            registry_file: DCSPKG_DIR.join("registry.json"),
            install_dir: DCSPKG_DIR.join("packages"),
            bin_dir: DCSPKG_DIR.join("bin"),
        }
    }
}
