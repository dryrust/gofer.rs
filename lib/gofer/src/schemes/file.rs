// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};
use std::fs::File;

/// See: https://en.wikipedia.org/wiki/File_URI_scheme
/// See: https://www.rfc-editor.org/rfc/rfc8089.html
pub fn open(url: &Url) -> Result<Box<dyn Read>> {
    // See: https://docs.rs/url/latest/url/struct.Url.html#method.to_file_path
    let path = url
        .to_file_path()
        .map_err(|_| Error::InvalidFileUrl(url.clone()))?;

    // See: https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.open
    // See: https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html#method.open
    let file = File::open(path)?;

    Ok(Box::new(file) as Box<dyn Read>)
}
