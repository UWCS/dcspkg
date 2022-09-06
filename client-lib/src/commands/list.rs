use anyhow::{Context, Result};
use reqwest::{blocking::Client, IntoUrl};

pub fn list(url: impl IntoUrl) -> Result<Vec<String>> {
    //craft URL
    let url: reqwest::Url = url
        .into_url()
        .map_err(anyhow::Error::from)
        .and_then(|url| url.join(crate::LIST_ENDPOINT).map_err(|e| e.into()))
        .context("Could not parse URL")?;

    log::info!("Downloading package list from {url}...");

    //fetch the list
    let list: Vec<String> = {
        let response = Client::new()
            .get(url.clone())
            .send()
            .context("Request failed")?;

        response.json().context("Could not parse json response")?
    };

    log::info!("Got reponse from {url}");
    log::debug!("Package list: {list:?}");

    Ok(list)
}
