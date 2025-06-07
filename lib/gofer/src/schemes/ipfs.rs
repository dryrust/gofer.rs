// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};
use reqwest::{blocking::ClientBuilder, redirect};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
static GATEWAY: &str = "https://ipfs.io";

/// See: https://en.wikipedia.org/wiki/InterPlanetary_File_System
pub fn open<'a, 'b>(url: &'a Url<'b>) -> Result<Box<dyn Read>> {
    // See: https://docs.rs/reqwest/latest/reqwest/blocking/struct.ClientBuilder.html
    let client = ClientBuilder::new()
        .user_agent(USER_AGENT)
        .redirect(redirect::Policy::default())
        .https_only(true);

    let url = url
        .as_str()
        .strip_prefix("ipfs://")
        .ok_or_else(|| Error::InvalidIpfsUrl(url.to_string()))
        .map(|id| format!("{}/ipfs/{}", GATEWAY, id))?;

    // See: https://docs.rs/reqwest/latest/reqwest/blocking/struct.Client.html#method.get
    // See: https://docs.rs/reqwest/latest/reqwest/blocking/struct.RequestBuilder.html
    let response = client.build()?.get(url).send()?;

    Ok(Box::new(response))
}
