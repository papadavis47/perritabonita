//! Turning your words into Perritabonita-speak, and drawing speech bubbles.

use rand::seq::SliceRandom;

/// Convert your text into an excited bark: uppercased with an exclamation.
///
/// Empty input just becomes a plain "WOOF!". This is the grown-up version of
/// the original program's `to_uppercase() + "!"`.
pub fn barkify(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return "WOOF!".to_string();
    }
    format!("{}!", trimmed.to_uppercase())
}

/// Draw a cowsay-style speech bubble around `text`, sized to its widest line.
///
/// Demonstrates iterator chains: we compute the bubble width by mapping each
/// line to its character count and taking the max.
pub fn bubble(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let width = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let mut out = String::new();
    out.push(' ');
    out.push_str(&"_".repeat(width + 2));
    out.push('\n');

    for line in &lines {
        let padding = width - line.chars().count();
        out.push_str(&format!("< {}{} >\n", line, " ".repeat(padding)));
    }

    out.push(' ');
    out.push_str(&"-".repeat(width + 2));
    out.push('\n');
    // The little tail connecting the bubble to Perritabonita.
    out.push_str("   \\\n    \\\n");
    out
}

/// Pick a random element from a slice, or `None` if it's empty.
///
/// This is generic over `T`, so it works for `&[String]` (tricks/fortunes) or
/// any other slice. The lifetime `'a` ties the borrowed result to the input.
///
/// Rust's lifetime elision would actually let us write `pick<T>(items: &[T])`
/// and infer the same thing; we spell `'a` out here to make the borrow
/// relationship visible while learning.
#[allow(clippy::needless_lifetimes)]
pub fn pick<'a, T>(items: &'a [T]) -> Option<&'a T> {
    items.choose(&mut rand::thread_rng())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn barkify_uppercases_and_adds_bang() {
        assert_eq!(barkify("who's a good girl"), "WHO'S A GOOD GIRL!");
    }

    #[test]
    fn barkify_trims_whitespace() {
        assert_eq!(barkify("  hello  "), "HELLO!");
    }

    #[test]
    fn barkify_handles_empty() {
        assert_eq!(barkify("   "), "WOOF!");
    }

    #[test]
    fn bubble_is_wide_enough_for_longest_line() {
        let out = bubble("hi\nlonger line");
        // Every rendered content line should share the same width.
        let widths: Vec<usize> = out
            .lines()
            .filter(|l| l.starts_with('<'))
            .map(|l| l.chars().count())
            .collect();
        assert!(widths.windows(2).all(|w| w[0] == w[1]));
    }

    #[test]
    fn pick_returns_none_for_empty() {
        let empty: [String; 0] = [];
        assert!(pick(&empty).is_none());
    }

    #[test]
    fn pick_returns_some_for_nonempty() {
        let items = vec!["a".to_string()];
        assert_eq!(pick(&items), Some(&"a".to_string()));
    }
}
