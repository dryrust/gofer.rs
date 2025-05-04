// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};
use std::fs::File;

/// See: https://en.wikipedia.org/wiki/File_URI_scheme
/// See: https://www.rfc-editor.org/rfc/rfc8089.html
pub fn open<'a, 'b>(url: &'a Url<'b>) -> Result<Box<dyn Read>> {
    // See: https://docs.rs/dogma/latest/dogma/enums/enum.Iri.html#method.to_path
    let path = url
        .to_path()
        .ok_or_else(|| Error::InvalidFileUrl(url.to_string()))?;

    // See: https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.open
    // See: https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html#method.open
    let file = File::open(path)?;

    Ok(Box::new(file))
}
