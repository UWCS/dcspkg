use anyhow::{bail, Context, Result};
use dcspkg_common::Package;
use reqwest::{blocking::get, IntoUrl, StatusCode};

pub fn list<U: IntoUrl>(url: U) -> Result<Vec<Package>> {
    //craft URL
    let url: reqwest::Url = url
        .into_url()
        .map_err(anyhow::Error::from)
        .and_then(|url| url.join(crate::LIST_ENDPOINT).map_err(|e| e.into()))
        .context("Could not parse URL")?;

    log::info!("Downloading package list from {url}...");

    //fetch the list
    let response = get(url.as_ref()).context("Request failed")?;
    log::info!("Got reponse from {url}");
    if response.status() != StatusCode::OK {
        bail!(
            "Response was not okay (got code {})",
            response.status().as_u16()
        )
    }
    let list: Vec<Package> = response.json().context("Could not parse JSON response")?;

    log::debug!("Package list: {list:?}");

    Ok(list)
}
