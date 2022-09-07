use anyhow::{anyhow, Context, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The directory to package up
    #[clap(validator = dir_exists)]
    directory: String,
    #[clap(short, long, value_parser, validator=file_exists)]
    #[clap(default_value = "packagedb.sqlite")]
    db: String,
    #[clap(short, long, value_parser, validator=dir_exists)]
    #[clap(default_value = "packages")]
    pkg_dir: String,
}

fn dir_exists(f: &str) -> Result<(), &'static str> {
    Path::new(f)
        .is_dir()
        .then_some(())
        .ok_or("Directory does not exist")
}

fn file_exists(f: &str) -> Result<(), &'static str> {
    Path::new(f)
        .is_file()
        .then_some(())
        .ok_or("File does not exist")
}

fn main() -> anyhow::Result<()> {
    let c = Cli::parse();
    dbg!(&c);
    let directory = PathBuf::from(c.directory);
    println!("Creating new dcspkg from {directory:?}");
    println!("Please specify package options (skip to use defaults)");

    let pkg_name = get_pkg_name(directory.file_name().and_then(|s| s.to_str()))?;
    let version = get_version()?;

    validate_name_and_version(&pkg_name, &version)?;

    let description = get_description()?;
    let url = get_image_url()?;
    let exe_path = get_exe_path()?;
    let add_to_path = add_to_path()?;
    let has_installer = has_installer(&directory)?;

    Ok(())
}

fn get_pkg_name(default: Option<&str>) -> Result<String> {
    if let Some(default) = default {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter package name")
            .default(default.to_string())
            .show_default(true)
            .interact_text()
    } else {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter package name")
            .show_default(true)
            .interact_text()
    }
    .context("Could not get package name")
}

fn get_description() -> Result<Option<String>> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter package description")
        .allow_empty(true)
        .interact_text()
        .map(|input| if input.is_empty() { None } else { Some(input) })
        .context("Could not get description")
}

fn get_version() -> Result<String> {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter version")
        .default("0.1.0".to_owned())
        .validate_with(|input: &String| semver::Version::parse(input).map(|_| ()))
        .interact_text()
        .context("Could not get version")
}

fn get_image_url() -> Result<Option<String>> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter URL for image")
        .allow_empty(true)
        .validate_with(|input: &String| {
            if input.is_empty() {
                Ok(())
            } else {
                url::Url::parse(input).map(|_| ())
            }
        })
        .interact_text()
        .map(|input| if input.is_empty() { None } else { Some(input) })
        .context("Could not get image URL")
}

fn get_exe_path() -> Result<Option<String>> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the path to the executable within this package, if one exists")
        .allow_empty(true)
        .validate_with(|input: &String| {
            let path = Path::new(input);
            path.is_file()
                .then_some(())
                .ok_or("executable specified does not exist")
        })
        .interact_text()
        .map(|input| if input.is_empty() { None } else { Some(input) })
        .context("Could not get executable path")
}

fn add_to_path() -> Result<bool> {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Do you wish for this executable to be added to the user's path on installation?",
        )
        .items(&["yes", "no"])
        .default(1)
        .interact()
        .map(|selection| match selection {
            0 => true,
            1 => false,
            _ => unreachable!(),
        })
        .context("Could not get choice for adding executable to path")
}

fn has_installer(dir: &Path) -> Result<bool> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Does this executable have an install.sh script?")
        .items(&["yes", "no"])
        .default(1)
        .interact()
        .map(|selection| match selection {
            0 => true,
            1 => false,
            _ => unreachable!(),
        })
        .context("Could not get choice for install script");
    let script_path = dir.join("install.sh");

    if script_path.is_file() {
        selection
    } else {
        Err(anyhow!("Could not find install script at {script_path:?}"))
    }
}

fn validate_name_and_version(pkg_name: &str, version: &str) -> Result<()> {
    Ok(())
}
