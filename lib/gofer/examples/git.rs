// This is free and unencumbered software released into the public domain.

use std::{boxed::Box, error::Error, io::stdout, result::Result};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let mut input = gofer::open("git://github.com/dryrust/gofer.rs/blob/master/VERSION")?;
    std::io::copy(&mut input, &mut output)?;

    Ok(())
}
