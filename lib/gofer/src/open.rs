// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url, UrlScheme};

pub fn open(url: impl AsRef<str>) -> Result<Box<dyn Read>> {
    let url = url.as_ref().parse::<Url>()?;

    // TODO: look up the scheme in a protocol handler registry

    match url.scheme() {
        #[cfg(feature = "data")]
        UrlScheme::Data => crate::schemes::data::open(&url),

        #[cfg(feature = "file")]
        UrlScheme::File => crate::schemes::file::open(&url),

        #[cfg(feature = "ftp")]
        UrlScheme::Ftp => crate::schemes::ftp::open(&url, false),

        #[cfg(feature = "ftps")]
        UrlScheme::Ftps => crate::schemes::ftp::open(&url, true),

        #[cfg(feature = "git")]
        UrlScheme::Git => crate::schemes::git::open(&url),

        #[cfg(feature = "http")]
        UrlScheme::Http => crate::schemes::http::open(&url, false),

        #[cfg(feature = "https")]
        UrlScheme::Https => crate::schemes::http::open(&url, true),

        #[cfg(feature = "scp")]
        UrlScheme::Scp => crate::schemes::scp::open(&url),

        #[cfg(feature = "stdin")]
        UrlScheme::Stdin => crate::schemes::stdin::open(&url),

        _ => Err(Error::UnknownScheme(url.scheme_str().to_string())),
    }
}

#[cfg(feature = "std")]
pub fn open_buffered(url: impl AsRef<str>) -> Result<std::io::BufReader<Box<dyn Read>>> {
    Ok(std::io::BufReader::new(open(url)?))
}
