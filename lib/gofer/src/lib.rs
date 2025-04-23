// This is free and unencumbered software released into the public domain.

//! This crate provides a protocol-independent `open()`.
//!
//! ```edition2021
//! # use gofer::*;
//! ```

#![deny(unsafe_code)]
#![allow(unused)]

mod features;
pub use features::*;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
