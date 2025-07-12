// This is free and unencumbered software released into the public domain.

//! This crate provides a universal, protocol-independent `open()`.
//!
//! ```edition2021
//! # use gofer::*;
//! ```

#![deny(unsafe_code)]
//#![allow(unused)]

use std::collections::HashMap;
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
pub use schemes::*;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

#[derive(Clone, Debug)]
pub struct RequestConfig {
    pub headers: HashMap<String, String>,
}

/// Configuration for HTTP requests.
///
/// This struct allows customizing HTTP requests with headers. It is used with protocol-specific
/// `open_with_config` functions to provide additional request parameters, such as custom HTTP headers.
///
/// # Examples
/// ```rust
/// use gofer::RequestConfig;
///
/// let config = RequestConfig::new()
///     .with_header("User-Agent", "gofer/1.0")
///     .with_header("Accept", "application/json");
/// assert_eq!(config.headers.len(), 2);
/// ```
impl RequestConfig {
    /// Creates a new, empty `RequestConfig`.
    ///
    /// # Returns
    /// A `RequestConfig` instance with no headers set.
    pub fn new() -> Self {
        RequestConfig {
            headers: HashMap::new(),
        }
    }

    /// Adds a header to the configuration and returns the modified `RequestConfig`.
    ///
    /// This method allows chaining to add multiple headers fluently.
    ///
    /// # Arguments
    /// * `key` - The header name (e.g., `"User-Agent"`).
    /// * `value` - The header value (e.g., `"gofer/1.0"`).
    ///
    /// # Returns
    /// The modified `RequestConfig` with the new header added.
    ///
    /// # Examples
    /// ```rust
    /// use gofer::RequestConfig;
    ///
    /// let config = RequestConfig::new().with_header("Authorization", "Bearer token");
    /// assert_eq!(config.headers.get("Authorization"), Some(&"Bearer token".to_string()));
    /// ```
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
}
