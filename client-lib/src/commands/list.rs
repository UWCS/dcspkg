use anyhow::{Context, Result};
use dcspkg_server::Package;
use reqwest::{blocking::get, IntoUrl};

pub fn list(url: impl IntoUrl) -> Result<Vec<Package>> {
    //craft URL
    let url: reqwest::Url = url
        .into_url()
        .map_err(anyhow::Error::from)
        .and_then(|url| url.join(crate::LIST_ENDPOINT).map_err(|e| e.into()))
        .context("Could not parse URL")?;

    log::info!("Downloading package list from {url}...");

    //fetch the list
    let list: Vec<Package> = get(url.as_ref())
        .context("Request failed")?
        .json()
        .context("Could not parse JSON response")?;

    log::info!("Got reponse from {url}");
    log::debug!("Package list: {list:?}");

    Ok(list)
}
