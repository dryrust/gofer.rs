// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};

/// See: https://en.wikipedia.org/wiki/Secure_copy_protocol
/// See: https://man.openbsd.org/scp
pub fn open(url: &Url) -> Result<Box<dyn Read>> {
    todo!() // TODO
}
