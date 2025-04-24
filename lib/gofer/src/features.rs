// This is free and unencumbered software released into the public domain.

/// The set of features that are enabled in this build of the crate.
pub static FEATURES: &[&str] = &[
    #[cfg(feature = "data")]
    "data",
    #[cfg(feature = "file")]
    "file",
    #[cfg(feature = "http")]
    "http",
    #[cfg(feature = "https")]
    "https",
    #[cfg(feature = "unstable")]
    "unstable",
];
