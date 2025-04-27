// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result};
use dogma::{Iri, IriScheme};

pub fn open<'a>(url: impl AsRef<str>) -> Result<'a, Box<dyn Read>> {
    let url = url.as_ref().parse::<Iri>()?;

    // TODO: look up the scheme in a protocol handler registry

    match url.scheme() {
        #[cfg(feature = "data")]
        IriScheme::Data => crate::schemes::data::open(&url),

        #[cfg(feature = "file")]
        IriScheme::File => crate::schemes::file::open(&url),

        #[cfg(feature = "ftp")]
        IriScheme::Ftp => crate::schemes::ftp::open(&url, false),

        #[cfg(feature = "ftps")]
        IriScheme::Ftps => crate::schemes::ftp::open(&url, true),

        #[cfg(feature = "git")]
        IriScheme::Git => crate::schemes::git::open(&url),

        #[cfg(feature = "http")]
        IriScheme::Http => crate::schemes::http::open(&url, false),

        #[cfg(feature = "https")]
        IriScheme::Https => crate::schemes::http::open(&url, true),

        #[cfg(feature = "scp")]
        IriScheme::Scp => crate::schemes::scp::open(&url),

        #[cfg(feature = "stdin")]
        IriScheme::Stdin => crate::schemes::stdin::open(&url),

        _ => Err(Error::UnknownScheme(url.scheme_str().to_string())),
    }
}

#[cfg(feature = "std")]
pub fn open_buffered<'a>(url: impl AsRef<str>) -> Result<'a, std::io::BufReader<Box<dyn Read>>> {
    Ok(std::io::BufReader::new(open(url)?))
}
