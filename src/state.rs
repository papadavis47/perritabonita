//! Perritabonita's persistent memory.
//!
//! Her stats are stored as JSON in the platform config directory
//! (e.g. `~/.config/perritabonita/state.json` on Linux) so she remembers you
//! between runs — a tiny Tamagotchi.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{PerritabonitaError, Result};

/// Everything Perritabonita remembers about her life with you.
///
/// `Serialize`/`Deserialize` are *derived*: serde writes the conversion code to
/// and from JSON for us based on these fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stats {
    /// How many times she's been petted, ever.
    pub times_petted: u32,
    /// How many treats she's eaten, ever.
    pub treats_eaten: u32,
    /// Current happiness, 0–100.
    pub happiness: u8,
    /// Current energy, 0–100. Low energy makes her sleepy.
    pub energy: u8,
    /// Current hunger, 0–100. High hunger makes her, well, hungry.
    pub hunger: u8,
}

impl Default for Stats {
    /// A brand-new puppy: content, rested, and a little peckish.
    fn default() -> Self {
        Stats {
            times_petted: 0,
            treats_eaten: 0,
            happiness: 70,
            energy: 80,
            hunger: 40,
        }
    }
}

impl Stats {
    /// Load Perritabonita's stats from the default location, or return fresh-puppy
    /// defaults if she has no memory yet.
    pub fn load() -> Result<Stats> {
        let path = default_path()?;
        Self::load_from(&path)
    }

    /// Save Perritabonita's stats to the default location, creating directories as needed.
    pub fn save(&self) -> Result<()> {
        let path = default_path()?;
        self.save_to(&path)
    }

    /// Load from a specific path. A missing file is *not* an error — it just
    /// means this is Perritabonita's first day, so we hand back the defaults.
    pub fn load_from(path: &Path) -> Result<Stats> {
        match fs::read_to_string(path) {
            Ok(contents) => {
                // `?` here converts a `serde_json::Error` into `PerritabonitaError::State`
                // via the `#[from]` we set up in `error.rs`.
                let stats = serde_json::from_str(&contents)?;
                Ok(stats)
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Stats::default()),
            Err(err) => Err(PerritabonitaError::Io(err)),
        }
    }

    /// Serialize to pretty JSON and write it, creating parent directories first.
    pub fn save_to(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}

/// Where Perritabonita keeps her memories on this machine.
///
/// `dirs::config_dir()` returns an `Option` because some exotic environments
/// have no config dir; we turn that `None` into a real error.
fn default_path() -> Result<PathBuf> {
    let mut path = dirs::config_dir().ok_or(PerritabonitaError::NoConfigDir)?;
    path.push("perritabonita");
    path.push("state.json");
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_yields_defaults() {
        let path = Path::new("/definitely/not/here/state.json");
        let stats = Stats::load_from(path).expect("missing file should be OK");
        assert_eq!(stats, Stats::default());
    }

    #[test]
    fn round_trips_through_disk() {
        // Write to a temp path, read it back, and confirm it matches.
        let mut path = std::env::temp_dir();
        path.push(format!("perritabonita_test_{}.json", std::process::id()));

        let stats = Stats {
            times_petted: 7,
            happiness: 99,
            ..Stats::default()
        };

        stats.save_to(&path).expect("save should succeed");
        let loaded = Stats::load_from(&path).expect("load should succeed");
        assert_eq!(stats, loaded);

        let _ = fs::remove_file(&path);
    }
}
