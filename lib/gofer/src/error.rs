// This is free and unencumbered software released into the public domain.

use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("unknown URL scheme: {0}")]
    UnknownScheme(String),

    #[cfg(feature = "data")]
    #[error("invalid data URL: {0}")]
    InvalidDataUrl(#[from] data_url::DataUrlError),

    #[cfg(feature = "data")]
    #[error("invalid data URL body: {0}")]
    InvalidDataUrlBody(#[from] data_url::forgiving_base64::InvalidBase64),

    #[cfg(feature = "file")]
    #[error("invalid file URL: {0}")]
    InvalidFileUrl(url::Url),

    #[cfg(feature = "file")]
    #[error("failed file I/O: {0}")]
    FailedFileIo(#[from] std::io::Error),

    #[cfg(any(feature = "ftp", feature = "ftps"))]
    #[error("invalid FTP URL: {0}")]
    InvalidFtpUrl(url::Url),

    #[cfg(any(feature = "ftp", feature = "ftps"))]
    #[error("failed FTP request: {0}")]
    FailedFtpRequest(#[from] suppaftp::FtpError),

    #[cfg(any(feature = "http", feature = "https"))]
    #[error("failed HTTP request: {0}")]
    FailedHttpRequest(#[from] reqwest::Error),
}

#[cfg(feature = "std")]
impl Into<std::io::Error> for Error {
    fn into(self) -> std::io::Error {
        use std::io::ErrorKind;
        match self {
            Error::InvalidUrl(e) => std::io::Error::new(ErrorKind::InvalidInput, e),
            Error::UnknownScheme(s) => std::io::Error::new(ErrorKind::InvalidInput, s),

            #[cfg(feature = "data")]
            Error::InvalidDataUrl(e) => std::io::Error::new(ErrorKind::InvalidInput, e),

            #[cfg(feature = "data")]
            Error::InvalidDataUrlBody(e) => std::io::Error::new(ErrorKind::InvalidData, e),

            #[cfg(feature = "file")]
            Error::InvalidFileUrl(u) => std::io::Error::new(ErrorKind::InvalidInput, u.as_str()),

            #[cfg(feature = "file")]
            Error::FailedFileIo(e) => e,

            #[cfg(any(feature = "ftp", feature = "ftps"))]
            Error::InvalidFtpUrl(u) => std::io::Error::new(ErrorKind::InvalidInput, u.as_str()),

            #[cfg(any(feature = "ftp", feature = "ftps"))]
            Error::FailedFtpRequest(_e) => std::io::Error::from(ErrorKind::Other), // FIXME

            #[cfg(any(feature = "http", feature = "https"))]
            Error::FailedHttpRequest(_e) => std::io::Error::from(ErrorKind::Other), // FIXME
        }
    }
}

impl TryInto<url::ParseError> for Error {
    type Error = Error;

    fn try_into(self) -> Result<url::ParseError> {
        match self {
            Error::InvalidUrl(e) => Ok(e),
            _ => Err(self),
        }
    }
}

#[cfg(feature = "data")]
impl TryInto<data_url::DataUrlError> for Error {
    type Error = Error;

    fn try_into(self) -> Result<data_url::DataUrlError> {
        match self {
            Error::InvalidDataUrl(e) => Ok(e),
            _ => Err(self),
        }
    }
}

#[cfg(feature = "data")]
impl TryInto<data_url::forgiving_base64::InvalidBase64> for Error {
    type Error = Error;

    fn try_into(self) -> Result<data_url::forgiving_base64::InvalidBase64> {
        match self {
            Error::InvalidDataUrlBody(e) => Ok(e),
            _ => Err(self),
        }
    }
}

#[cfg(any(feature = "ftp", feature = "ftps"))]
impl TryInto<suppaftp::FtpError> for Error {
    type Error = Error;

    fn try_into(self) -> Result<suppaftp::FtpError> {
        match self {
            Error::FailedFtpRequest(e) => Ok(e),
            _ => Err(self),
        }
    }
}

#[cfg(any(feature = "http", feature = "https"))]
impl TryInto<reqwest::Error> for Error {
    type Error = Error;

    fn try_into(self) -> Result<reqwest::Error> {
        match self {
            Error::FailedHttpRequest(e) => Ok(e),
            _ => Err(self),
        }
    }
}
