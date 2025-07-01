// This is free and unencumbered software released into the public domain.

use crate::{Read, Result, Url};
use ureq::{Agent, tls::{TlsConfig, TlsProvider}};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// See: https://en.wikipedia.org/wiki/HTTP
/// See: https://en.wikipedia.org/wiki/HTTPS
pub fn open<'a, 'b>(url: &'a Url<'b>, secure: bool) -> Result<Box<dyn Read>> {
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.Agent.html
    let mut agent_builder = Agent::config_builder()
        .user_agent(USER_AGENT);

    if secure {
        agent_builder = agent_builder.tls_config(
            TlsConfig::builder()
                .provider(TlsProvider::Rustls)
                .build()
        );
    }

    let agent: ureq::Agent = agent_builder.build().into();

    // See: https://docs.rs/ureq/3.0.12/ureq/struct.Agent.html#method.get
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.RequestBuilder.html#method.call
    let response = agent.get(url.as_str()).call()?;
    let (_headers, body) = response.into_parts();

    Ok(Box::new(body.into_reader()))
}
