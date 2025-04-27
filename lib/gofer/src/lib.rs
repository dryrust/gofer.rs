// This is free and unencumbered software released into the public domain.

//! This crate provides a universal, protocol-independent `open()`.
//!
//! ```edition2021
//! # use gofer::*;
//! ```

#![deny(unsafe_code)]
//#![allow(unused)]

#[cfg(feature = "std")]
pub use std::io::{Cursor, Read};

#[cfg(not(feature = "std"))]
todo!("the 'std' feature is currently required"); // TODO

pub use dogma::Uri as Url;
pub use dogma::UriError as UrlError;
pub use dogma::UriScheme as UrlScheme;

mod features;
pub use features::*;

mod error;
pub use error::*;

mod open;
pub use open::*;

mod schemes;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
