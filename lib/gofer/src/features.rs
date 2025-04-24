// This is free and unencumbered software released into the public domain.

/// The set of features that are enabled in this build of the crate.
pub static FEATURES: &[&str] = &[
    #[cfg(feature = "data")]
    "data",
    #[cfg(feature = "file")]
    "file",
    #[cfg(feature = "ftp")]
    "ftp",
    #[cfg(feature = "ftps")]
    "ftps",
    #[cfg(feature = "git")]
    "git",
    #[cfg(feature = "http")]
    "http",
    #[cfg(feature = "https")]
    "https",
    #[cfg(feature = "scp")]
    "scp",
    #[cfg(feature = "stdin")]
    "stdin",
    #[cfg(feature = "unstable")]
    "unstable",
];
