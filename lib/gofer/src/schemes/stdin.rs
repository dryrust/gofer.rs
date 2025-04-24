// This is free and unencumbered software released into the public domain.

use crate::{Read, Result, Url};

/// See: https://en.wikipedia.org/wiki/Standard_streams#Standard_input_(stdin)
pub fn open(_url: &Url) -> Result<Box<dyn Read>> {
    Ok(Box::new(std::io::stdin().lock()))
}
