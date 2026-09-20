//! Loading Perritabonita's data-driven content (tricks & fortunes) from TOML.
//!
//! Keeping this content in `assets/*.toml` means you can add new tricks or
//! fortunes by editing a text file — no Rust changes, no recompiling the logic.
//! (They're embedded with `include_str!`, so a rebuild picks up edits.)

use serde::Deserialize;

use crate::error::Result;

const TRICKS_TOML: &str = include_str!("../assets/tricks.toml");
const FORTUNES_TOML: &str = include_str!("../assets/fortunes.toml");

/// Matches the shape of `tricks.toml`: a top-level `tricks = [...]` array.
#[derive(Debug, Deserialize)]
struct TricksFile {
    tricks: Vec<String>,
}

/// Matches the shape of `fortunes.toml`: a top-level `fortunes = [...]` array.
#[derive(Debug, Deserialize)]
struct FortunesFile {
    fortunes: Vec<String>,
}

/// All of Perritabonita's tricks.
pub fn tricks() -> Result<Vec<String>> {
    // `toml::from_str` turns text into our typed struct; `?` converts a
    // `toml::de::Error` into `PerritabonitaError::Data`.
    let parsed: TricksFile = toml::from_str(TRICKS_TOML)?;
    Ok(parsed.tricks)
}

/// All of Perritabonita's fortunes.
pub fn fortunes() -> Result<Vec<String>> {
    let parsed: FortunesFile = toml::from_str(FORTUNES_TOML)?;
    Ok(parsed.fortunes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tricks_file_parses_and_is_nonempty() {
        let tricks = tricks().expect("bundled tricks.toml should parse");
        assert!(!tricks.is_empty());
    }

    #[test]
    fn fortunes_file_parses_and_is_nonempty() {
        let fortunes = fortunes().expect("bundled fortunes.toml should parse");
        assert!(!fortunes.is_empty());
    }
}
