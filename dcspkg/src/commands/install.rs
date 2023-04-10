use crate::Package;
use anyhow::{anyhow, bail, Context, Result};
use bytes::Buf;
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
    pkg_name: &str,                    //the packages pkgname
    server_url: impl reqwest::IntoUrl, //the url of the server, from config
    package_dir: P,                    //the local package install dir, from config
    bin_dir: P,                        //the local bin install dir, from config
    registry_file: P,                  //the local json registry file, from config
) -> Result<()> {
    let server_url = server_url
        .into_url()
        .context("Could not parse server URL")?;

    let package_dir = package_dir.as_ref();
    let bin_dir = bin_dir.as_ref();

    //get package data
    let pkg =
        get_pkg_data(pkg_name, &server_url).context("Could not get package data from server")?;

    //create the install directory
    fs::create_dir_all(package_dir).context("Could not create install directory for package")?;

    let install_dir = package_dir.join(pkg_name);
    //download, checksum, and decompress into PKGDIR/bin
    download_install_file(pkg_name, pkg.crc, &server_url, &install_dir)
        .context("Could not install file")?;

    //run install.sh if exists
    if pkg.has_installer {
        run_install_script(&install_dir).context("Could not run install script for file")?;
    }

    if pkg.add_to_path {
        //the relative path from within the package
        let relative_exe_path: &Path = pkg.executable_path.as_ref().context("Package is configured to add executable to path, but is not configured with an executable path")?.as_ref();

        //the symlink to create in /bin, on path
        let bin_exe_path = bin_dir.join(
            relative_exe_path
                .file_name()
                .context("Could not get file name from executable path")?,
        );

        //the symlink target, in /packages/<pkg-name>
        let package_exe_path = install_dir.join(relative_exe_path);

        //create bin path if not already exists
        fs::create_dir_all(bin_dir).context("Could not create bin directory")?;

        let link: &Path = &bin_exe_path;
        let source: &Path = &package_exe_path;
        log::info!("Creating symlink to {package_exe_path:?} at {bin_exe_path:?}");
        symlink(source, link).context("Could not create symbolic link to package executable")?;
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
    pkg_name: &str,
    checksum: u32,
    server_url: &Url,
    install_dir: &Path,
) -> Result<()> {
    let url = server_url
        .join(format!("{}/{}.dcspkg", crate::FILE_ENDPOINT, pkg_name).as_ref())
        .context("Could not parse URL")?;

    log::info!("Downloading compressed package {pkg_name} from {url}...");

    let response = get(url.as_ref()).context("Request failed")?;
    log::info!("Got reponse from {url}");

    if response.status() != StatusCode::OK {
        bail!(
            "Response was not okay (got code {} when requesting {})",
            response.status().as_u16(),
            url
        )
    }

    //the content of the response
    let compressed = response
        .bytes()
        .context("Could not get content of response")?;

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
    fs::set_permissions(&script, Permissions::from_mode(0o764))
        .context("Failed to set file permissions for install script")?;

    log::info!("Executing install script...");

    //spawn a child process executing script
    let output = Command::new("sh")
        .arg(&script)
        .output()
        .context("Could not execute install.sh")?;

    if !output.stdout.is_empty() {
        log::info!(
            "Install script stdout: {}",
            String::from_utf8_lossy(&output.stdout)
        )
    }

    if !output.stderr.is_empty() {
        log::info!(
            "Install script stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    }

    if !output.status.success() {
        bail!(
            "Install script exited with code {}",
            output.status.to_string()
        )
    }

    log::info!("Install script finished, cleaning up...");
    fs::remove_file(&script).context("Could not remove script")?;
    Ok(())
}

fn add_to_registry(registry_file: &Path, package: Package) -> Result<()> {
    //create empty registry if not exists
    if !registry_file.exists() {
        fs::write(registry_file, "[]").context("Could not create package registry")?;
    }

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(registry_file)
        .context("Could not open registry file")?;

    let mut loaded: Vec<Package> =
        serde_json::from_reader(&file).context("Could not deserialize registry file context")?;
    log::debug!("Deserialised contents of registry file");

    loaded.push(package);
    file.rewind()
        .and_then(|_| serde_json::to_writer(&file, &loaded).map_err(Into::into))
        .context("Could not write registry back to file")?;

    log::info!("Added package to local registry");

    Ok(())
}
