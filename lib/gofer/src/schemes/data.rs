// This is free and unencumbered software released into the public domain.

use crate::{Cursor, Error, Read, Result, Url};
use data_url::DataUrl;

/// See: https://en.wikipedia.org/wiki/Data_URI_scheme
/// See: https://fetch.spec.whatwg.org/#data-urls
/// See: https://www.rfc-editor.org/rfc/rfc2397
pub fn open(url: &Url) -> Result<Box<dyn Read>> {
    let url = DataUrl::process(url.as_str())?;

    let (body, _) = url.decode_to_vec()?;

    Ok(Box::new(Cursor::new(body)) as Box<dyn Read>)
}
