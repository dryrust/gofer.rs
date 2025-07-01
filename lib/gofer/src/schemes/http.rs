// This is free and unencumbered software released into the public domain.

use crate::{Read, Result, Url, schemes::utils::create_get_request};

/// See: https://en.wikipedia.org/wiki/HTTP
/// See: https://en.wikipedia.org/wiki/HTTPS
pub fn open<'a, 'b>(url: &'a Url<'b>, secure: bool) -> Result<Box<dyn Read>> {
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.RequestBuilder.html#method.call
    let request = create_get_request(secure, url.as_str())?;
    let response = request.call()?;
    let (_headers, body) = response.into_parts();

    Ok(Box::new(body.into_reader()))
}
