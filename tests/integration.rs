//! Integration tests.
//!
//! Unlike the `#[cfg(test)]` modules inside `src/`, these files are compiled as
//! a separate crate that depends on `perritabonita`, so they can only touch the
//! *public* API — exactly what a real consumer sees. This is a good way to make
//! sure the public surface is actually usable and the modules cooperate.

use perritabonita::data;
use perritabonita::dog::{Mood, Perritabonita};
use perritabonita::speech::{barkify, bubble, pick};
use perritabonita::state::Stats;

#[test]
fn a_full_day_with_perritabonita() {
    // Start from fresh-puppy defaults.
    let mut perritabonita = Perritabonita::new("Perritabonita", Stats::default());

    // Petting several times should make her demonstrably happier.
    let happiness_before = perritabonita.stats.happiness;
    for _ in 0..3 {
        perritabonita.pet();
    }
    assert!(perritabonita.stats.happiness > happiness_before);
    assert_eq!(perritabonita.stats.times_petted, 3);

    // Feeding reduces hunger and is remembered.
    let hunger_before = perritabonita.stats.hunger;
    perritabonita.feed();
    assert!(perritabonita.stats.hunger < hunger_before);
    assert_eq!(perritabonita.stats.treats_eaten, 1);
}

#[test]
fn mood_reflects_extreme_stats() {
    let hungry = Perritabonita::new(
        "Perritabonita",
        Stats {
            hunger: 95,
            ..Stats::default()
        },
    );
    assert_eq!(hungry.mood(), Mood::Hungry);

    let sleepy = Perritabonita::new(
        "Perritabonita",
        Stats {
            energy: 10,
            hunger: 0,
            ..Stats::default()
        },
    );
    assert_eq!(sleepy.mood(), Mood::Sleepy);
}

#[test]
fn speech_pipeline_produces_a_bubble() {
    let spoken = barkify("who is a good girl");
    assert_eq!(spoken, "WHO IS A GOOD GIRL!");

    let drawn = bubble(&spoken);
    assert!(drawn.contains("WHO IS A GOOD GIRL!"));
    // The bubble has a top border of underscores.
    assert!(drawn.contains('_'));
}

#[test]
fn bundled_data_loads_and_is_pickable() {
    let tricks = data::tricks().expect("tricks.toml should parse");
    let fortunes = data::fortunes().expect("fortunes.toml should parse");

    assert!(!tricks.is_empty());
    assert!(!fortunes.is_empty());

    // `pick` should return one of the actual items.
    let trick = pick(&tricks).expect("non-empty slice yields Some");
    assert!(tricks.contains(trick));
}

#[test]
fn state_survives_a_save_load_round_trip() {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "perritabonita_integration_{}.json",
        std::process::id()
    ));

    let stats = Stats {
        times_petted: 42,
        treats_eaten: 9,
        ..Stats::default()
    };

    stats.save_to(&path).expect("save");
    let loaded = Stats::load_from(&path).expect("load");
    assert_eq!(stats, loaded);

    let _ = std::fs::remove_file(&path);
}
