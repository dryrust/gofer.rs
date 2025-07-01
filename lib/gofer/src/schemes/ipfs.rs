// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};
use ureq::{Agent, tls::{TlsConfig, TlsProvider}};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
static GATEWAY: &str = "https://ipfs.io";

/// See: https://en.wikipedia.org/wiki/InterPlanetary_File_System
pub fn open<'a, 'b>(url: &'a Url<'b>) -> Result<Box<dyn Read>> {
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.Agent.html
    let agent: Agent = Agent::config_builder()
        .user_agent(USER_AGENT)
        .tls_config(
            TlsConfig::builder()
                .provider(TlsProvider::Rustls)
                .build()
        )
        .build()
        .into();

    let url = url
        .as_str()
        .strip_prefix("ipfs://")
        .ok_or_else(|| Error::InvalidIpfsUrl(url.to_string()))
        .map(|id| format!("{}/ipfs/{}", GATEWAY, id))?;

    // See: https://docs.rs/ureq/3.0.12/ureq/struct.Agent.html#method.get
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.RequestBuilder.html#method.call
    let response = agent.get(&url).call()?;
    let (_headers, body) = response.into_parts();

    Ok(Box::new(body.into_reader()))
}
