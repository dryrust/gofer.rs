// This is free and unencumbered software released into the public domain.

#[cfg(feature = "data")]
pub mod data;

#[cfg(feature = "file")]
pub mod file;

#[cfg(any(feature = "ftp", feature = "ftps"))]
pub mod ftp;

#[cfg(feature = "git")]
pub mod git;

#[cfg(any(feature = "http", feature = "https"))]
pub mod http;

#[cfg(feature = "scp")]
pub mod scp;

#[cfg(feature = "stdin")]
pub mod stdin;
