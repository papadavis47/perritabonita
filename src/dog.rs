//! The star of the show: [`Perritabonita`] and her [`Mood`].
//!
//! This module is where we move from "a script" to "a model": Perritabonita is
//! a value that owns her state and exposes behavior through methods.

use crate::error::Result;
use crate::state::Stats;

/// Perritabonita's current mood, derived from her stats.
///
/// Deriving `Clone, Copy` lets us pass a `Mood` around by value cheaply (it's
/// just a tag), and `PartialEq, Eq` lets us compare and `match` on it easily.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Happy,
    Excited,
    Sleepy,
    Hungry,
}

impl Mood {
    /// A short human label. `match` on `self` must cover every variant, so if
    /// you add a new mood the compiler will remind you to handle it here.
    pub fn label(self) -> &'static str {
        match self {
            Mood::Happy => "happy",
            Mood::Excited => "excited",
            Mood::Sleepy => "sleepy",
            Mood::Hungry => "hungry",
        }
    }

    /// A mood-appropriate bark she leads with.
    pub fn greeting(self) -> &'static str {
        match self {
            Mood::Happy => "Woof! *tail wags*",
            Mood::Excited => "WOOFWOOFWOOF!!",
            Mood::Sleepy => "*yawwwn* ...woof?",
            Mood::Hungry => "Woof! *stares at the treat jar*",
        }
    }
}

/// A virtual Golden Retriever with a name, memory, and feelings.
#[derive(Debug, Clone)]
pub struct Perritabonita {
    pub name: String,
    pub stats: Stats,
}

impl Perritabonita {
    /// Build Perritabonita from an explicit set of stats.
    pub fn new(name: impl Into<String>, stats: Stats) -> Self {
        Perritabonita {
            name: name.into(),
            stats,
        }
    }

    /// Load Perritabonita's remembered stats from disk (or fresh-puppy
    /// defaults).
    pub fn load() -> Result<Self> {
        Ok(Perritabonita::new("Perritabonita", Stats::load()?))
    }

    /// Persist Perritabonita's current stats to disk.
    pub fn save(&self) -> Result<()> {
        self.stats.save()
    }

    /// Give Perritabonita a pet. Takes `&mut self` because it changes her
    /// state: happier, a bit more awake, and she remembers it forever.
    pub fn pet(&mut self) {
        self.stats.times_petted = self.stats.times_petted.saturating_add(1);
        self.stats.happiness = clamp_up(self.stats.happiness, 10);
        self.stats.energy = clamp_up(self.stats.energy, 3);
    }

    /// Feed Perritabonita a treat: less hungry, happier, slightly more
    /// energetic.
    pub fn feed(&mut self) {
        self.stats.treats_eaten = self.stats.treats_eaten.saturating_add(1);
        self.stats.hunger = self.stats.hunger.saturating_sub(25);
        self.stats.happiness = clamp_up(self.stats.happiness, 5);
        self.stats.energy = clamp_up(self.stats.energy, 5);
    }

    /// Derive Perritabonita's mood from her current stats.
    ///
    /// Order matters: pressing needs (hunger, tiredness) win over general mood.
    pub fn mood(&self) -> Mood {
        let s = &self.stats;
        if s.hunger >= 70 {
            Mood::Hungry
        } else if s.energy <= 30 {
            Mood::Sleepy
        } else if s.happiness >= 85 {
            Mood::Excited
        } else {
            Mood::Happy
        }
    }
}

/// Add `amount` to `value` but never exceed 100.
fn clamp_up(value: u8, amount: u8) -> u8 {
    value.saturating_add(amount).min(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn perritabonita_with(happiness: u8, energy: u8, hunger: u8) -> Perritabonita {
        Perritabonita::new(
            "Perritabonita",
            Stats {
                times_petted: 0,
                treats_eaten: 0,
                happiness,
                energy,
                hunger,
            },
        )
    }

    #[test]
    fn hunger_dominates_mood() {
        let perritabonita = perritabonita_with(100, 100, 90);
        assert_eq!(perritabonita.mood(), Mood::Hungry);
    }

    #[test]
    fn low_energy_means_sleepy() {
        let perritabonita = perritabonita_with(100, 20, 10);
        assert_eq!(perritabonita.mood(), Mood::Sleepy);
    }

    #[test]
    fn petting_increases_happiness_and_count() {
        let mut perritabonita = perritabonita_with(50, 50, 20);
        perritabonita.pet();
        assert_eq!(perritabonita.stats.times_petted, 1);
        assert_eq!(perritabonita.stats.happiness, 60);
    }

    #[test]
    fn happiness_never_overflows_past_100() {
        let mut perritabonita = perritabonita_with(98, 50, 20);
        perritabonita.pet(); // +10 would be 108 without clamping
        assert_eq!(perritabonita.stats.happiness, 100);
    }

    #[test]
    fn feeding_reduces_hunger_without_underflow() {
        let mut perritabonita = perritabonita_with(50, 50, 10);
        perritabonita.feed(); // 10 - 25 must not underflow a u8
        assert_eq!(perritabonita.stats.hunger, 0);
    }
}
