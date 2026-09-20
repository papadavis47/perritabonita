//! Command-line interface definition.
//!
//! `clap`'s derive API lets us describe the whole CLI declaratively. The macros
//! generate the argument parser, `--help`/`--version`, and friendly error
//! messages from these type definitions.

use clap::{Parser, Subcommand};

/// A virtual Golden Retriever companion for your terminal.
#[derive(Debug, Parser)]
#[command(name = "perritabonita", version, about, long_about = None)]
pub struct Cli {
    /// Which command to run. If omitted, Perritabonita just says hello.
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Everything you can ask Perritabonita to do.
///
/// Each variant is a subcommand; its fields become that subcommand's arguments.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Have Perritabonita bark your words back at you.
    Say {
        /// The words to bark (all trailing words are collected together).
        #[arg(trailing_var_arg = true)]
        words: Vec<String>,
    },
    /// Give Perritabonita a pet. She'll remember it.
    Pet,
    /// Feed Perritabonita a treat.
    Feed,
    /// Ask Perritabonita to perform a random trick.
    Trick,
    /// Receive a piece of dog wisdom.
    Fortune,
    /// Show Perritabonita in her current mood.
    Show {
        /// Animate her wagging tail instead of a static portrait.
        #[arg(long)]
        animate: bool,
    },
    /// Print Perritabonita's current stats and mood.
    Status,
}
