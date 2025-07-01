// This is free and unencumbered software released into the public domain.

use crate::{Read, Result, Url};
use crate::schemes::utils;

/// Performs an HTTP or HTTPS GET request to fetch a file from a URL.
///
/// # Arguments
/// * `url` - The URL to fetch (e.g., `http://example.com/file.txt` or `https://example.com/file.txt`).
/// * `secure` - Whether to use HTTPS (TLS) for the request.
///
/// # Returns
/// A `Result` containing a boxed readable stream (`Box<dyn Read>`) on success, or an `Error` on failure.
///
/// # References
/// - [HTTP Protocol](https://en.wikipedia.org/wiki/HTTP)
/// - [HTTPS Protocol](https://en.wikipedia.org/wiki/HTTPS)
pub fn open<'a, 'b>(url: &'a Url<'b>, secure: bool) -> Result<Box<dyn Read>> {
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.Agent.html
    let agent = utils::new_agent(secure, None);

    // See: https://docs.rs/ureq/3.0.12/ureq/struct.Agent.html#method.get
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.RequestBuilder.html#method.call
    utils::fetch(&agent, url.as_str())
}
