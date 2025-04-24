// This is free and unencumbered software released into the public domain.

use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("unknown URL scheme: {0}")]
    UnknownScheme(String),
}
