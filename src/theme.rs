//! Mapping moods to colors, so Perrita Bonita *looks* how she feels.
//!
//! Keeping color choices here (instead of scattered through the logic) means
//! the rest of the code never has to think about ANSI escapes.

use colored::Color;

use crate::dog::Mood;

/// A pair of colors used to render Perrita Bonita in a given mood.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    /// Color for the ASCII art itself.
    pub art: Color,
    /// Color for headings and accents.
    pub accent: Color,
}

impl Theme {
    /// Choose a theme for the given mood.
    pub fn for_mood(mood: Mood) -> Theme {
        match mood {
            Mood::Happy => Theme {
                art: Color::Yellow,
                accent: Color::BrightYellow,
            },
            Mood::Excited => Theme {
                art: Color::BrightYellow,
                accent: Color::BrightMagenta,
            },
            Mood::Sleepy => Theme {
                art: Color::BrightBlack,
                accent: Color::BrightBlue,
            },
            Mood::Hungry => Theme {
                art: Color::BrightRed,
                accent: Color::BrightCyan,
            },
        }
    }
}
