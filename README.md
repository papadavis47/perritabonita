# 🐾 Perritabonita

A virtual Golden Retriever companion for your terminal — and a hands-on Rust
learning project. Inspired by our beautiful, crazy dog. ❤️

Perritabonita has **moods**, remembers you between runs (a tiny Tamagotchi), does
**tricks**, shares **dog wisdom**, and can **wag her tail** in animated ASCII.

## Quick start

```sh
cargo run -- <command>
```

| Command                        | What it does                                       |
| ------------------------------ | -------------------------------------------------- |
| `perritabonita`                         | Show Perritabonita in her current mood (default)            |
| `perritabonita say "who's a good girl"` | Bark your words back in a speech bubble            |
| `perritabonita pet`                     | Pet her — raises happiness, and she remembers it   |
| `perritabonita feed`                    | Give her a treat — lowers hunger                   |
| `perritabonita trick`                   | Perform a random trick (from `assets/tricks.toml`) |
| `perritabonita fortune`                 | Share a random piece of dog wisdom                 |
| `perritabonita show --animate`          | Animated tail wag, then settle into a portrait     |
| `perritabonita status`                  | Show her stats and mood with little meters         |

Her memory lives at `~/.config/perritabonita/state.json` (path varies by OS). Delete that
file to reset her to a fresh puppy.

## How it's built

The project is split into a **library** (`src/lib.rs`, all the logic) and a
tiny **binary** (`src/main.rs`, just argument parsing) so the logic is reusable
and testable.

```
src/
├── main.rs    # thin entry point: parse args, call run(), handle errors
├── lib.rs     # run() dispatch + rendering helpers
├── cli.rs     # clap command/argument definitions
├── dog.rs     # Perritabonita struct + Mood enum (the core model)
├── state.rs   # persistent stats (serde_json + filesystem)
├── data.rs    # load tricks/fortunes from TOML (serde)
├── speech.rs  # barkify(), speech bubbles, random picker
├── art.rs     # ASCII art loading + animation (crossterm)
├── theme.rs   # mood -> colors
└── error.rs   # one custom error type for the whole crate
assets/
├── art/*.txt  # ASCII portraits + animation frames
├── tricks.toml
└── fortunes.toml
tests/
└── integration.rs  # tests the public API end-to-end
```

## Where to learn each Rust concept

| Concept                                   | Look at                                                       |
| ----------------------------------------- | ------------------------------------------------------------- |
| Library + binary split                    | `Cargo.toml`, `main.rs`, `lib.rs`                             |
| Modules & visibility (`pub`)              | `lib.rs` module declarations                                  |
| Structs & methods (`&self` / `&mut self`) | `dog.rs` (`Perritabonita`)                                             |
| Enums & exhaustive `match`                | `dog.rs` (`Mood`), `art.rs`                                   |
| Custom errors, `?`, `From` via `#[from]`  | `error.rs`, used everywhere                                   |
| Traits via derive                         | `Serialize`/`Deserialize` in `state.rs`, `Parser` in `cli.rs` |
| Generics & lifetimes                      | `speech.rs` (`pick`)                                          |
| Iterators & closures                      | `speech.rs` (`bubble`), `lib.rs` (`meter`)                    |
| `Option` handling                         | `state.rs`, `speech.rs`                                       |
| Filesystem I/O                            | `state.rs`                                                    |
| Compile-time embedding                    | `include_str!` in `art.rs`, `data.rs`                         |
| Serialization                             | `state.rs` (JSON), `data.rs` (TOML)                           |
| Unit + integration + doc tests            | `#[cfg(test)]` blocks, `tests/`, `lib.rs` doc                 |

## Development

```sh
cargo test            # run unit, integration, and doc tests
cargo clippy          # lint (currently clean)
cargo run -- status   # try it out
```

## Stretch goals (exercises for you)

Ideas to keep learning, roughly easy → hard:

1. **More content**: add tricks/fortunes to the TOML files. (No Rust needed.)
2. **A new mood**: add `Mood::Playful`. The compiler will walk you through every
   `match` you must update — a great tour of exhaustiveness.
3. **Time decay**: store a "last interacted" timestamp in `Stats`; make Perritabonita get
   hungrier/sleepier the longer since you last visited (learn `SystemTime`, and
   the `chrono` or `time` crate).
4. **Config file**: let users rename Perritabonita or pick a favorite color via a TOML
   config loaded at startup.
5. **`perritabonita play` mini-game**: a guessing or fetch game using `rand` and a loop.
6. **Richer animation**: add more frames or a blinking idle animation.
7. **CLI polish**: add shell completions with `clap_complete`.
8. **Errors with context**: swap `thiserror` for `anyhow` in the binary layer and
   compare the ergonomics.
