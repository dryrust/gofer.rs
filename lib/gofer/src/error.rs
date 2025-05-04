// This is free and unencumbered software released into the public domain.

use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
pub enum Error {
    #[error("invalid URL: {0}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(gofer::invalid_url),
            help("it seems that the URL is malformed in some way"),
            url(docsrs),
        )
    )]
    InvalidUrl(#[from] crate::UrlError),

    #[error("unknown URL scheme: {0}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(gofer::unknown_scheme),
            help("there is no protocol handler available for this URL scheme"),
            url(docsrs),
        )
    )]
    UnknownScheme(String),

    #[cfg(feature = "data")]
    #[error("invalid data URL: {0}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(gofer::invalid_data_url),
            help("it seems that the URL is malformed in some way"),
            url(docsrs),
        )
    )]
    InvalidDataUrl(#[from] data_url::DataUrlError),

    #[cfg(feature = "data")]
    #[error("invalid data URL body: {0}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(gofer::invalid_data_url_body),
            help("something went wrong when decoding the Base64-encoded data"),
            url(docsrs),
        )
    )]
    InvalidDataUrlBody(#[from] data_url::forgiving_base64::InvalidBase64),

    #[cfg(feature = "file")]
    #[error("invalid file URL: {0}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(gofer::invalid_file_url),
            help("it seems that the URL is malformed in some way"),
            url(docsrs),
        )
    )]
    InvalidFileUrl(String),

    #[cfg(feature = "file")]
    #[error("failed file I/O: {0}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(gofer::failed_file_io),
            help("something went wrong when opening or reading the file"),
            url(docsrs),
        )
    )]
    FailedFileIo(#[from] std::io::Error),

    #[cfg(any(feature = "ftp", feature = "ftps"))]
    #[error("invalid FTP URL: {0}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(gofer::invalid_ftp_url),
            help("it seems that the URL is malformed in some way"),
            url(docsrs),
        )
    )]
    InvalidFtpUrl(String),

    #[cfg(any(feature = "ftp", feature = "ftps"))]
    #[error("failed FTP request: {0}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(gofer::failed_ftp_request),
            help("something went wrong connecting to the server or requesting the file"),
            url(docsrs),
        )
    )]
    FailedFtpRequest(#[from] suppaftp::FtpError),

    #[cfg(any(feature = "http", feature = "https"))]
    #[error("failed HTTP request: {0}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(gofer::failed_http_request),
            help("something went wrong connecting to the server or requesting the file"),
            url(docsrs),
        )
    )]
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
