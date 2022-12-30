use anyhow::Context;
use config::{Config, Environment, File};
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

lazy_static::lazy_static! {
    pub static ref DCSPKG_DIR: PathBuf = home_dir().expect("Could not find your home directory!").join(".dcspkg");
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DcspkgConfig {
    pub server: Server,
    pub registry: Registry,
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

impl DcspkgConfig {
    pub fn get() -> anyhow::Result<Self> {
        let config_file_path = DCSPKG_DIR.join("config.toml");

        if !config_file_path.exists() {
            log::info!("Config file does not exist");
            create_default_config_file(&config_file_path)
                .context("Could not create config file")?;
        }

        let config = Config::builder()
            .add_source(
                File::with_name(
                    DCSPKG_DIR
                        .join("config.toml")
                        .to_str()
                        .expect("Could not build config file name"),
                )
                .required(true),
            )
            .add_source(Environment::with_prefix("DCSPKG").separator("_"))
            .build()?;

        log::info!("Loaded config from file and environment");

        config.try_deserialize().map_err(Into::into)
    }
}

fn create_default_config_file(path: &Path) -> anyhow::Result<()> {
    let default_contents = toml::to_string_pretty(&DcspkgConfig::default())
        .context("Error in serializing default config struct")?;
    std::fs::write(path, default_contents).context("Could not write to file")?;
    log::info!("Created new config file from defaults");
    Ok(())
}
