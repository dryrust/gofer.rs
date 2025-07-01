// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};
use crate::schemes::utils;

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
    let agent = utils::new_agent(true, None);

    let url = url
        .as_str()
        .strip_prefix("ipfs://")
        .ok_or_else(|| Error::InvalidIpfsUrl(url.to_string()))
        .map(|id| format!("{}/ipfs/{}", GATEWAY, id))?;

    // See: https://docs.rs/ureq/3.0.12/ureq/struct.Agent.html#method.get
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.RequestBuilder.html#method.call
    utils::fetch(&agent, &url)
}
