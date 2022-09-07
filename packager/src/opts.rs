use anyhow::{bail, Context, Result};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::path::Path;

pub fn get_pkg_name(default: Option<&str>) -> Result<String> {
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

pub fn get_description() -> Result<Option<String>> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter package description")
        .allow_empty(true)
        .interact_text()
        .map(|input| if input.is_empty() { None } else { Some(input) })
        .context("Could not get description")
}

pub fn get_version() -> Result<String> {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter version")
        .default("0.1.0".to_owned())
        .validate_with(|input: &String| semver::Version::parse(input).map(|_| ()))
        .interact_text()
        .context("Could not get version")
}

pub fn get_image_url() -> Result<Option<String>> {
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

pub fn get_exe_path(base_dir: &Path) -> Result<Option<String>> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the relative path of the executable within this package")
        .allow_empty(true)
        .validate_with(|input: &String| {
            let path = base_dir.join(input);
            path.is_file()
                .then_some(())
                .ok_or("executable specified does not exist")
        })
        .interact_text()
        .map(|input| if input.is_empty() { None } else { Some(input) })
        .context("Could not get executable path")
}

pub fn add_to_path() -> Result<bool> {
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

pub fn has_installer(dir: &Path) -> Result<bool> {
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
        .context("Could not get choice for install script")?;
    let script_path = dir.join("install.sh");

    if selection && !script_path.is_file() {
        bail!("Could not find install script at {script_path:?}")
    }
    Ok(selection)
}
