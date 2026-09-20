//! The `perritabonita` binary: a thin shell around the `perritabonita` library.
//!
//! It parses command-line arguments and hands off to [`perritabonita::run`]. All the
//! real logic lives in the library so it can be tested independently.

use std::process::ExitCode;

use clap::Parser;
use colored::Colorize;

use perritabonita::cli::Cli;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match perritabonita::run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            // Print to stderr so it doesn't pollute piped output, and return a
            // non-zero exit code so scripts can detect the failure.
            eprintln!("{} {err}", "error:".red().bold());
            ExitCode::FAILURE
        }
    }
}
