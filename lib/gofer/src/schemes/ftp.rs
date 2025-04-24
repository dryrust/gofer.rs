// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};

/// See: https://en.wikipedia.org/wiki/FTP
/// See: https://en.wikipedia.org/wiki/FTPS
pub fn open(url: &Url, https: bool) -> Result<Box<dyn Read>> {
    todo!() // TODO
}
