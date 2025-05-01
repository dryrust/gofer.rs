// This is free and unencumbered software released into the public domain.

use crate::{Read, Result, Url};
use reqwest::{blocking::ClientBuilder, redirect};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// See: https://en.wikipedia.org/wiki/HTTP
/// See: https://en.wikipedia.org/wiki/HTTPS
pub fn open<'a, 'b>(url: &'a Url<'b>, secure: bool) -> Result<Box<dyn Read>> {
    // See: https://docs.rs/reqwest/latest/reqwest/blocking/struct.ClientBuilder.html
    let mut client = ClientBuilder::new()
        .user_agent(USER_AGENT)
        .redirect(redirect::Policy::default());

    if secure {
        client = client.https_only(true);
    }

    // See: https://docs.rs/reqwest/latest/reqwest/blocking/struct.Client.html#method.get
    // See: https://docs.rs/reqwest/latest/reqwest/blocking/struct.RequestBuilder.html
    let response = client.build()?.get(url.as_str()).send()?;

    Ok(Box::new(response))
}
