use anyhow::Context;
use std::os::unix::process::CommandExt;
use crate::config::DcspkgConfig;
use crate::get_registry;


pub fn run(config: DcspkgConfig, package: &String) -> anyhow::Result<()> {
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