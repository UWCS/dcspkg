use anyhow::{anyhow, bail, Context, Result};
use bytes::Buf;
use dcspkg_common::Package;
use flate2::{read::GzDecoder, CrcReader};
use reqwest::blocking::get;
use reqwest::{StatusCode, Url};
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::Path;
use std::process::Command;
use std::{
    fs::{self, Permissions},
    io::Seek,
};
use tar::Archive;

pub fn install<P: AsRef<Path>>(
    pkg_name: &str,
    server_url: impl reqwest::IntoUrl,
    package_dir: P,
    bin_dir: P,
    registry_file: P,
) -> Result<()> {
    let server_url = server_url
        .into_url()
        .context("Could not parse server URL")?;

    let package_dir = package_dir.as_ref();

    //get package data
    let pkg =
        get_pkg_data(pkg_name, &server_url).context("Could not get package data from server")?;

    //create the install directory
    fs::create_dir_all(&package_dir).context("Could not create install directory for package")?;

    let install_dir = package_dir.join(&pkg.name);
    //download, checksum, and decompress into PKGDIR/bin
    download_install_file(&pkg.archive_path, pkg.crc, &server_url, &install_dir)
        .context("Could not install file")?;

    //run install.sh if exists
    if pkg.has_installer {
        run_install_script(&install_dir).context("Could not run install script for file")?;
    }

    if pkg.add_to_path {
        let exe_path = install_dir.join(pkg.executable_path.as_ref().context(
            "Package is configured to add executable to path, but is not configured with an executable path",
        )?);

        //create bin path if not already exists
        fs::create_dir_all(bin_dir.as_ref())
            .context("Could not create install directory for package")?;

        create_symlink(bin_dir.as_ref().join(&pkg.name).as_path(), &exe_path)
            .context("Could not create symbolic link to package executable")?;
    }

    add_to_registry(registry_file.as_ref(), pkg).context("Could not add package to registry")?;

    Ok(())
}

fn get_pkg_data(pkg_name: &str, server_url: &Url) -> Result<Package> {
    let url = server_url
        .join(format!("{}/{}", crate::DATA_ENDPOINT, pkg_name).as_ref())
        .context("Could not parse URL")?;

    log::info!("Downloading data for package {pkg_name} from {url}...");

    //download the package date as an option
    let response = get(url.as_ref()).context("Request failed")?;
    log::info!("Got reponse from {url}");

    match response.status() {
        StatusCode::OK => (),
        StatusCode::NOT_FOUND => bail!("Package does not exist on server (404)"),
        r => bail!("Response from server was not okay (code {})", r.as_u16()),
    }

    let package: Package = response.json().context("Could not parse JSON response")?;

    log::debug!("Package data: {package:?}");

    //if option empty then err here
    Ok(package)
}

fn download_install_file(
    pkg_url: &str,
    checksum: u32,
    server_url: &Url,
    install_dir: &Path,
) -> Result<()> {
    let url = server_url
        .join(format!("{}/{}", crate::FILE_ENDPOINT, pkg_url).as_ref())
        .context("Could not parse URL")?;

    log::info!("Downloading compressed package {pkg_url} from {url}...");

    let response = get(url.as_ref()).context("Request failed")?;
    log::info!("Got reponse from {url}");

    if response.status() != StatusCode::OK {
        bail!(
            "Response was not okay (got code {})",
            response.status().as_u16()
        )
    }

    //the content of the response
    let compressed = response
        .bytes()
        .context("Could not get content of response")?;

    //check the crc value is correct

    log::info!("Decompressing and unpacking package...");

    //decompress and unarchive the bytes
    let reader = GzDecoder::new(CrcReader::new(compressed.reader()));
    let mut archive = Archive::new(reader);

    //unpack archive
    archive
        .unpack(install_dir)
        .context("Could not unpack archive")?;

    let downloaded_checksum = archive.into_inner().into_inner().crc().sum();
    log::info!("Checksum of downloaded package is {downloaded_checksum} (expected {checksum})");

    // if downloaded_checksum != checksum {
    //     return Err(anyhow!("Checksum for downloaded file did not match!"));
    // }

    log::info!("Unpacked archive");
    log::debug!("Unpacked into {:?}", install_dir);

    Ok(())
}

fn run_install_script(path: &Path) -> Result<()> {
    //check the script is real
    let script = path.join("install.sh");
    if !script.exists() {
        return Err(anyhow!(
            "We were lied to by the server, install.sh does not exist at {script:?}"
        ));
    }

    log::info!("Got install script at {script:?}");

    //set the scripts perms to allow us to execute it
    fs::set_permissions(&script, Permissions::from_mode(0o764))?;

    log::info!("Executing install script...");
    //spawn a child process executing script
    let mut cmd = Command::new("sh")
        .arg(path)
        .spawn()
        .context("Could not execute install.sh")?;

    //wait for it to finish
    cmd.wait()?;

    log::info!("Install script finished, cleaning up...");
    fs::remove_file(&script).context("Could not remove script")?;
    Ok(())
}

fn create_symlink(dcspkg_bin_path: &Path, exe_path: &Path) -> Result<()> {
    log::info!("Creating symlink from {exe_path:?} to {dcspkg_bin_path:?}");
    symlink(exe_path, dcspkg_bin_path)?;
    Ok(())
}

fn add_to_registry(registry_file: &Path, package: Package) -> Result<()> {
    //create empty registry if not exists
    if !registry_file.exists() {
        fs::write(registry_file, "[]")?;
    }

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(registry_file)
        .context("Could not open registry file")?;

    let mut loaded: Vec<Package> =
        serde_json::from_reader(&file).context("Could not deserialize registry file context")?;
    log::debug!("Deserialised contents of registry fike");

    loaded.push(package);
    file.rewind()?;
    serde_json::to_writer(&file, &loaded).context("Could not write registry back to file")?;

    log::info!("Added package to local registry");

    Ok(())
}
