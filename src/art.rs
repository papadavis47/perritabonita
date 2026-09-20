//! Loading Perritabonita's ASCII portraits.
//!
//! The art lives in `assets/art/*.txt`. We pull it in with `include_str!`, which
//! reads the files *at compile time* and bakes them into the binary. That makes
//! the final program self-contained: no runtime file lookups, no dependence on
//! where you happen to run it from.

use crate::dog::Mood;

// Paths are relative to this file (`src/art.rs`), hence the `../assets/...`.
const HAPPY: &str = include_str!("../assets/art/perritabonita_happy.txt");
const SLEEPY: &str = include_str!("../assets/art/perritabonita_sleepy.txt");

/// The static portrait for a given mood.
///
/// Only two portraits exist (awake and sleepy); non-sleepy moods share the
/// awake pose and are told apart by [`crate::theme::Theme`]'s per-mood color
/// instead of separate art.
pub fn portrait(mood: Mood) -> &'static str {
    match mood {
        Mood::Sleepy => SLEEPY,
        Mood::Happy | Mood::Excited | Mood::Hungry => HAPPY,
    }
}
