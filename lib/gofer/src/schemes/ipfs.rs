// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, RequestConfig, Result, Url};
use crate::schemes::request;

static GATEWAY: &str = "https://ipfs.io";

/// Fetches a file from the InterPlanetary File System (IPFS) using a gateway.
///
/// The function converts an IPFS URL (e.g., `ipfs://<content-id>`) to a gateway URL
/// and performs an HTTPS GET request to retrieve the file content as a readable stream.
///
/// # Arguments
/// * `url` - The IPFS URL specifying the content ID (e.g., `ipfs://Qm...`).
///
/// # Returns
/// A `Result` containing a boxed readable stream (`Box<dyn Read>`) on success, or an `Error` if the URL is invalid.
///
/// # References
/// - [InterPlanetary File System](https://en.wikipedia.org/wiki/InterPlanetary_File_System)
pub fn open<'a, 'b>(url: &'a Url<'b>) -> Result<Box<dyn Read>> {
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.Agent.html
    let agent = request::new_agent(true, None);
    let url = map_ipfs_url_to_gateway_url(url.as_str())?;

    // See: https://docs.rs/ureq/3.0.12/ureq/struct.Agent.html#method.get
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.RequestBuilder.html#method.call
    request::fetch(&agent, &url)
}

/// Fetches a file from the InterPlanetary File System (IPFS) using a gateway with custom request configuration.
///
/// The function converts an IPFS URL (e.g., `ipfs://<content-id>`) to a gateway URL
/// and performs an HTTPS GET request with the specified configuration (e.g., custom headers)
/// to retrieve the file content as a readable stream.
///
/// # Arguments
/// * `url` - The IPFS URL specifying the content ID (e.g., `ipfs://Qm...`).
/// * `config` - Custom request configuration, such as HTTP headers.
///
/// # Returns
/// A `Result` containing a boxed readable stream (`Box<dyn Read>`) on success, or an `Error` if the URL is invalid.
///
/// # References
/// - [InterPlanetary File System](https://en.wikipedia.org/wiki/InterPlanetary_File_System)
pub fn open_with_config<'a, 'b>(url: &'a Url<'b>, config: RequestConfig) -> Result<Box<dyn Read>> {
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.Agent.html
    let agent = request::new_agent(true, None);
    let gateway_url = map_ipfs_url_to_gateway_url(url.as_str())?;

    // See: https://docs.rs/ureq/3.0.12/ureq/struct.Agent.html#method.get
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.RequestBuilder.html#method.call
    request::fetch_with_config(&agent, &gateway_url, &config)
}

/// Maps an IPFS URL to a gateway URL for file retrieval.
///
/// Converts URLs like `ipfs://<content-id>` to `https://ipfs.io/ipfs/<content-id>`.
///
/// # Arguments
/// * `url_str` - The IPFS URL to map.
///
/// # Returns
/// A `Result` containing the gateway URL as a `String` on success, or an `Error` if the URL is invalid.
fn map_ipfs_url_to_gateway_url(url_str: &str) -> Result<String> {
    url_str
        .strip_prefix("ipfs://")
        .ok_or_else(|| Error::InvalidIpfsUrl(url_str.to_string()))
        .map(|id| format!("{}/ipfs/{}", GATEWAY, id))
}
