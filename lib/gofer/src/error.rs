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
    #[error("invalid `data:` URL: {0}")]
    InvalidDataUrl(#[from] data_url::DataUrlError),

    #[cfg(feature = "data")]
    #[error("invalid `data:` URL body: {0}")]
    InvalidDataUrlBody(#[from] data_url::forgiving_base64::InvalidBase64),
}
