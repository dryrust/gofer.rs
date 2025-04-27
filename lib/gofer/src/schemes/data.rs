// This is free and unencumbered software released into the public domain.

use crate::{Cursor, Read, Result, Url};
use data_url::DataUrl;

/// See: https://en.wikipedia.org/wiki/Data_URI_scheme
/// See: https://fetch.spec.whatwg.org/#data-urls
/// See: https://www.rfc-editor.org/rfc/rfc2397.html
pub fn open<'a, 'b>(url: &'a Url<'b>) -> Result<'b, Box<dyn Read>> {
    // See: https://docs.rs/data-url/latest/data_url/struct.DataUrl.html#method.process
    let url = DataUrl::process(url.as_str())?;

    // See: https://docs.rs/data-url/latest/data_url/struct.DataUrl.html#method.decode_to_vec
    let (body, _) = url.decode_to_vec()?;

    Ok(Box::new(Cursor::new(body)))
}
