//! Loading Perritabonita's ASCII portraits and animating her.
//!
//! The art lives in `assets/art/*.txt`. We pull it in with `include_str!`, which
//! reads the files *at compile time* and bakes them into the binary. That makes
//! the final program self-contained: no runtime file lookups, no dependence on
//! where you happen to run it from.

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

use crate::dog::Mood;
use crate::error::Result;

// Paths are relative to this file (`src/art.rs`), hence the `../assets/...`.
const HAPPY: &str = include_str!("../assets/art/perritabonita_happy.txt");
const SLEEPY: &str = include_str!("../assets/art/perritabonita_sleepy.txt");
const EXCITED: &str = include_str!("../assets/art/perritabonita_excited.txt");
const HUNGRY: &str = include_str!("../assets/art/perritabonita_hungry.txt");
const WAG_01: &str = include_str!("../assets/art/perritabonita_wag_01.txt");
const WAG_02: &str = include_str!("../assets/art/perritabonita_wag_02.txt");

/// The static portrait for a given mood.
///
/// Matching on `Mood` (rather than a string key) means the compiler guarantees
/// we've covered every mood — add a variant and this won't build until you do.
pub fn portrait(mood: Mood) -> &'static str {
    match mood {
        Mood::Happy => HAPPY,
        Mood::Sleepy => SLEEPY,
        Mood::Excited => EXCITED,
        Mood::Hungry => HUNGRY,
    }
}

/// The two frames of Perritabonita's tail-wag / blink animation.
pub fn wag_frames() -> [&'static str; 2] {
    [WAG_01, WAG_02]
}

/// Play `frames` in a loop, clearing the screen between each.
///
/// `loops` is how many full cycles to run; `delay` is the pause per frame.
pub fn animate(frames: &[&str], loops: usize, delay: Duration) -> Result<()> {
    let mut stdout = io::stdout();
    for _ in 0..loops {
        for frame in frames {
            execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
            print!("{frame}");
            // Print buffers by default; flush so the frame shows immediately.
            stdout.flush()?;
            thread::sleep(delay);
        }
    }
    Ok(())
}
