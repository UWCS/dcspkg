use crate::list_installed_packages;
use anyhow::Context;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};

/// Launches the specified package. This exits the current process
/// and launches the package in its place.
pub fn run_package(registry_file: &Path, install_dir: PathBuf, package: &String) -> anyhow::Result<()> {
    let package_data = list_installed_packages(registry_file)?
        .into_iter()
        .find(|pkg| pkg.pkgname == *package)
        .context(format!(
            "Could not find a package with the name {} in {:?}",
            package, registry_file
        ))?;

    let exe_path = install_dir.join(package_data.pkgname).join(
        package_data
            .executable_path
            .context("No executable exists for this package")?,
    );

    //will only return if there is an error
    Err(std::process::Command::new(exe_path).exec()).map_err(Into::into)
}
