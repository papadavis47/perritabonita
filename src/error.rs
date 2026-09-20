//! Error types for the whole crate.
//!
//! Rather than sprinkling `.unwrap()`/`.expect()` around (which *panic* and
//! abort the program), every fallible function returns a [`Result`], and the
//! `?` operator bubbles failures up to `main`, where they're printed nicely.

use std::io;
use thiserror::Error;

/// Every way Perritabonita's world can go wrong.
///
/// The `#[from]` attributes tell `thiserror` to generate `From` implementations.
/// Those `From` impls are what let the `?` operator convert, say, an
/// `std::io::Error` into a `PerritabonitaError::Io` automatically.
#[derive(Debug, Error)]
pub enum PerritabonitaError {
    /// Reading stdin or touching the filesystem failed.
    #[error("Perritabonita had an I/O hiccup: {0}")]
    Io(#[from] io::Error),

    /// Perritabonita's saved memory file couldn't be read or written as JSON.
    #[error("Perritabonita's memory file is scrambled: {0}")]
    State(#[from] serde_json::Error),

    /// A bundled data file (tricks/fortunes) couldn't be parsed as TOML.
    #[error("Perritabonita's data file couldn't be parsed: {0}")]
    Data(#[from] toml::de::Error),

    /// We couldn't figure out where to store Perritabonita's memory on this OS.
    #[error("Perritabonita couldn't find a home for her memories (no config dir)")]
    NoConfigDir,
}

/// A convenient crate-wide alias so signatures read `Result<T>`.
pub type Result<T> = std::result::Result<T, PerritabonitaError>;
