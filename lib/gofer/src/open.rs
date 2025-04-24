// This is free and unencumbered software released into the public domain.

use crate::{Error, Result};
use std::io::Read;
use url::Url;

pub fn open(url: impl AsRef<str>) -> Result<Box<dyn Read>> {
    let url = Url::parse(url.as_ref())?;

    match url.scheme() {
        "data" => open_data(&url),
        "file" => open_file(&url),
        "http" | "https" => open_http(&url),
        scheme => Err(Error::UnknownScheme(scheme.to_string())), // TODO: registry
    }
}

fn open_data(url: &Url) -> Result<Box<dyn Read>> {
    todo!() // TODO
}

fn open_file(url: &Url) -> Result<Box<dyn Read>> {
    todo!() // TODO
}

fn open_http(url: &Url) -> Result<Box<dyn Read>> {
    todo!() // TODO
}
