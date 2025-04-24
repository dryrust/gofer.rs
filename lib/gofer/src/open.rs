// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};

pub fn open(url: impl AsRef<str>) -> Result<Box<dyn Read>> {
    let url = Url::parse(url.as_ref())?;

    // TODO: look up the scheme in a protocol handler registry

    match url.scheme() {
        #[cfg(feature = "data")]
        "data" => crate::schemes::data::open(&url),

        #[cfg(feature = "file")]
        "file" => crate::schemes::file::open(&url),

        #[cfg(feature = "ftp")]
        "ftp" => crate::schemes::ftp::open(&url, false),

        #[cfg(feature = "ftps")]
        "ftps" => crate::schemes::ftp::open(&url, true),

        #[cfg(feature = "git")]
        "git" => crate::schemes::git::open(&url),

        #[cfg(feature = "http")]
        "http" => crate::schemes::http::open(&url, false),

        #[cfg(feature = "https")]
        "https" => crate::schemes::http::open(&url, true),

        #[cfg(feature = "scp")]
        "scp" => crate::schemes::scp::open(&url),

        #[cfg(feature = "stdin")]
        "stdin" => crate::schemes::stdin::open(&url),

        scheme => Err(Error::UnknownScheme(scheme.to_string())),
    }
}

#[cfg(feature = "std")]
pub fn open_buffered(url: impl AsRef<str>) -> Result<std::io::BufReader<Box<dyn Read>>> {
    Ok(std::io::BufReader::new(open(url)?))
}
