// This is free and unencumbered software released into the public domain.

use crate::{Read, Result, Url};

/// See: https://en.wikipedia.org/wiki/Git
/// See: https://git-scm.com/docs/protocol-v2
pub fn open<'a, 'b>(_url: &'a Url<'b>) -> Result<Box<dyn Read>> {
    todo!() // TODO
}
