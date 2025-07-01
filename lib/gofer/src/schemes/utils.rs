// This is free and unencumbered software released into the public domain.

use crate::Result;
use ureq::{Agent, RequestBuilder, typestate::WithoutBody, tls::{TlsConfig, TlsProvider}};
use std::time::Duration;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Creates a configured ureq RequestBuilder for GET requests with user agent,
/// optional HTTPS enforcement, and Accept header for JSON-LD and any media type.
///
/// If `secure` is true, enforces HTTPS using rustls. Sets a default timeout of 30 seconds.
pub fn create_get_request(secure: bool, url: &str) -> Result<RequestBuilder<WithoutBody>> {
    let mut builder = Agent::config_builder()
        .user_agent(USER_AGENT)
        .timeout_global(Some(Duration::from_secs(30)));

    if secure {
        builder = builder.tls_config(
            TlsConfig::builder()
                .provider(TlsProvider::Rustls)
                .build()
        );
    }

    let agent: Agent = builder.build().into();
    Ok(agent.get(url).header("Accept", "application/ld+json,*/*"))
}