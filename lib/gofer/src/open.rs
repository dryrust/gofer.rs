// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};

pub fn open(url: impl AsRef<str>) -> Result<Box<dyn Read>> {
    let url = Url::parse(url.as_ref())?;

    match url.scheme() {
        #[cfg(feature = "data")]
        "data" => crate::schemes::data::open(&url),

        #[cfg(feature = "file")]
        "file" => crate::schemes::file::open(&url),

        #[cfg(feature = "http")]
        "http" => crate::schemes::http::open(&url),

        #[cfg(feature = "https")]
        "https" => crate::schemes::http::open(&url),

        scheme => Err(Error::UnknownScheme(scheme.to_string())), // TODO: registry
    }
}
