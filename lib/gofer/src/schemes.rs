// This is free and unencumbered software released into the public domain.

#[cfg(feature = "data")]
pub mod data;

#[cfg(feature = "file")]
pub mod file;

#[cfg(any(feature = "http", feature = "https"))]
pub mod http;
