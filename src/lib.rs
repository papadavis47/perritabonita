//! Perritabonita: a virtual Golden Retriever companion for your terminal.
//!
//! The binary (`src/main.rs`) is intentionally tiny — it just parses arguments
//! and calls [`run`]. All the interesting logic lives here in the library so it
//! can be unit- and integration-tested.
//!
//! # Example
//!
//! ```
//! use perritabonita::speech::barkify;
//! assert_eq!(barkify("good girl"), "GOOD GIRL!");
//! ```

// Module declarations. Each corresponds to a `src/<name>.rs` file.
pub mod art;
pub mod cli;
pub mod data;
pub mod dog;
pub mod error;
pub mod speech;
pub mod state;
pub mod theme;

use colored::Colorize;

use crate::cli::{Cli, Command};
use crate::dog::Perritabonita;
use crate::error::Result;
use crate::theme::Theme;

/// Entry point for the application logic.
///
/// Loads Perritabonita's remembered stats, dispatches on the chosen command,
/// and renders the result. Returns a [`Result`] so `main` can print any error
/// nicely.
pub fn run(cli: Cli) -> Result<()> {
    let mut perritabonita = Perritabonita::load()?;

    // No subcommand? Just show her.
    let command = cli.command.unwrap_or(Command::Show);

    // Breathing room between the shell prompt and Perritabonita's output.
    println!();

    match command {
        Command::Say { words } => {
            say(&perritabonita, &words.join(" "));
        }
        Command::Pet => {
            perritabonita.pet();
            perritabonita.save()?;
            let theme = Theme::for_mood(perritabonita.mood());
            println!(
                "{}",
                "You give Perritabonita a good pet. *thump thump thump*".color(theme.accent)
            );
            println!();
            show(&perritabonita)?;
        }
        Command::Feed => {
            perritabonita.feed();
            perritabonita.save()?;
            let theme = Theme::for_mood(perritabonita.mood());
            println!(
                "{}",
                "Perritabonita gobbles up the treat! *happy crunching*".color(theme.accent)
            );
            println!();
            show(&perritabonita)?;
        }
        Command::Trick => {
            trick(&perritabonita)?;
        }
        Command::Fortune => {
            fortune(&perritabonita)?;
        }
        Command::Show => {
            show(&perritabonita)?;
        }
        Command::Status => {
            status(&perritabonita);
        }
    }

    Ok(())
}

/// Render Perritabonita saying something inside a speech bubble, above her
/// portrait.
fn say(perritabonita: &Perritabonita, text: &str) {
    let mood = perritabonita.mood();
    let theme = Theme::for_mood(mood);
    let spoken = speech::barkify(text);
    print!("{}", speech::bubble(&spoken).color(theme.accent));
    println!();
    println!("{}", art::portrait(mood).color(theme.art));
}

/// Show Perritabonita's portrait plus her mood.
fn show(perritabonita: &Perritabonita) -> Result<()> {
    let mood = perritabonita.mood();
    let theme = Theme::for_mood(mood);

    println!("{}", art::portrait(mood).color(theme.art));
    println!(
        "{}",
        format!("{} is feeling {}.", perritabonita.name, mood.label()).color(theme.accent)
    );
    println!("{}", mood.greeting().color(theme.accent));
    Ok(())
}

/// Pick and print a random trick.
fn trick(perritabonita: &Perritabonita) -> Result<()> {
    let tricks = data::tricks()?;
    let theme = Theme::for_mood(perritabonita.mood());
    match speech::pick(&tricks) {
        Some(t) => println!("{}", format!("Perritabonita {t}.").color(theme.accent)),
        None => println!("Perritabonita doesn't know any tricks yet!"),
    }
    println!();
    println!("{}", art::portrait(perritabonita.mood()).color(theme.art));
    Ok(())
}

/// Pick and print a random fortune.
fn fortune(perritabonita: &Perritabonita) -> Result<()> {
    let fortunes = data::fortunes()?;
    let theme = Theme::for_mood(perritabonita.mood());
    match speech::pick(&fortunes) {
        Some(f) => println!("{}", format!("\u{1F4AD} {f}").color(theme.accent)),
        None => println!("Perritabonita is out of wisdom for now."),
    }
    println!();
    println!("{}", art::portrait(perritabonita.mood()).color(theme.art));
    Ok(())
}

/// Print a full status readout with little meters.
fn status(perritabonita: &Perritabonita) {
    let s = &perritabonita.stats;
    let mood = perritabonita.mood();
    let theme = Theme::for_mood(mood);

    println!(
        "{}",
        format!("\u{1F43E} {}'s status", perritabonita.name)
            .color(theme.accent)
            .bold()
    );
    println!("  Mood:         {}", mood.label());
    println!("  Happiness:    {}", meter(s.happiness));
    println!("  Energy:       {}", meter(s.energy));
    println!("  Hunger:       {}", meter(s.hunger));
    println!("  Times petted: {}", s.times_petted);
    println!("  Treats eaten: {}", s.treats_eaten);
}

/// Render a 0–100 value as a 20-cell bar, e.g. `[██████··············] 30`.
fn meter(value: u8) -> String {
    let filled = ((value as usize) / 5).min(20);
    let empty = 20 - filled;
    format!(
        "[{}{}] {}",
        "\u{2588}".repeat(filled),
        "\u{00B7}".repeat(empty),
        value
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meter_is_full_at_100() {
        assert_eq!(meter(100), "[████████████████████] 100");
    }

    #[test]
    fn meter_is_empty_at_0() {
        assert_eq!(meter(0), "[····················] 0");
    }
}
