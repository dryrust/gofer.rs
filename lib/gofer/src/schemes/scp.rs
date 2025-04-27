// This is free and unencumbered software released into the public domain.

use crate::{Read, Result, Url};

/// See: https://en.wikipedia.org/wiki/Secure_copy_protocol
/// See: https://man.openbsd.org/scp
pub fn open<'a, 'b>(_url: &'a Url<'b>) -> Result<'b, Box<dyn Read>> {
    todo!() // TODO
}
