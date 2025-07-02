// This is free and unencumbered software released into the public domain.

use ureq::{Agent, tls::{TlsConfig, TlsProvider}};
use crate::{Read, RequestConfig, Result};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Creates a new ureq Agent with a specified user agent and optional TLS configuration.
///
/// # Arguments
/// * `use_tls` - Whether to enable TLS with Rustls as the provider.
/// * `tls_provider` - Optional TLS provider; defaults to Rustls if `use_tls` is true.
pub(crate) fn new_agent(use_tls: bool, tls_provider: Option<TlsProvider>) -> Agent {
    let mut builder = Agent::config_builder()
        .user_agent(USER_AGENT);

    if use_tls {
        let provider = tls_provider.unwrap_or(TlsProvider::Rustls);
        builder = builder.tls_config(
            TlsConfig::builder()
                .provider(provider)
                .build()
        );
    }

    builder.build().into()
}

/// Performs an HTTP GET request and returns the response body as a readable stream.
///
/// # Arguments
/// * `agent` - The ureq Agent to use for the request.
/// * `url` - The URL to fetch.
pub(crate) fn fetch(agent: &Agent, url: &str) -> Result<Box<dyn Read>> {
    let response = agent.get(url).call()?;
    let (_headers, body) = response.into_parts();
    Ok(Box::new(body.into_reader()))
}

/// Performs an HTTP GET request with custom configuration and returns the response body as a readable stream.
///
/// # Arguments
/// * `agent` - The ureq Agent to use for the request.
/// * `url` - The URL to fetch.
/// * `config` - Configuration containing custom headers.
pub(crate) fn fetch_with_config(agent: &Agent, url: &str, config: &RequestConfig) -> Result<Box<dyn Read>> {
    let mut request = agent.get(url);
    for (key, value) in &config.headers {
        request = request.header(key, value);
    }
    let response = request.call()?;
    let (_headers, body) = response.into_parts();
    Ok(Box::new(body.into_reader()))
}