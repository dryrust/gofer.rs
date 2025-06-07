// This is free and unencumbered software released into the public domain.

use std::{boxed::Box, error::Error, io::stdout, result::Result};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let mut input =
        gofer::open("ipfs://bafybeifx7yeb55armcsxwwitkymga5xf53dxiarykms3ygqic223w5sk3m")?;
    std::io::copy(&mut input, &mut output)?;

    Ok(())
}
