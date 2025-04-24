// This is free and unencumbered software released into the public domain.

use std::{boxed::Box, env::args, error::Error, io::stdout, result::Result};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    for url in args().skip(1) {
        let mut input = gofer::open(url)?;
        std::io::copy(&mut input, &mut output)?;
    }
    Ok(())
}
