// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};

/// See: https://en.wikipedia.org/wiki/Standard_streams#Standard_input_(stdin)
pub fn open(url: &Url) -> Result<Box<dyn Read>> {
    todo!() // TODO
}
