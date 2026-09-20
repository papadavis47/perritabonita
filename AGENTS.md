# AGENTS.md

Guidance for AI coding agents working in this repository.

## Project overview

`perritabonita` is a small Rust CLI: a virtual Golden Retriever companion for the
terminal. It is primarily a **learning project**, so clarity and idiomatic,
well-explained code matter as much as functionality. Prefer straightforward,
readable solutions over clever ones, and keep the teaching-oriented doc comments
intact.

It is structured as a **library + thin binary**:

- `src/main.rs` — tiny entry point: parse args, call `perritabonita::run`, handle errors.
- `src/lib.rs` — `run()` dispatch and rendering helpers; declares all modules.
- All real logic lives in the library so it can be unit- and integration-tested.

## Module map

| File                   | Responsibility                                           |
| ---------------------- | -------------------------------------------------------- |
| `src/cli.rs`           | `clap` command/argument definitions                      |
| `src/dog.rs`           | `Perritabonita` struct + `Mood` enum (core model)                 |
| `src/state.rs`         | Persistent stats via `serde_json` + filesystem           |
| `src/data.rs`          | Load tricks/fortunes from bundled TOML                   |
| `src/speech.rs`        | `barkify()`, speech bubbles, generic `pick()`            |
| `src/art.rs`           | ASCII art (`include_str!`)                                |
| `src/theme.rs`         | Mood → colors                                            |
| `src/error.rs`         | Single crate-wide `PerritabonitaError` + `Result<T>` alias        |
| `assets/`              | ASCII art `.txt` files + `tricks.toml` / `fortunes.toml` |
| `tests/integration.rs` | Public-API integration tests                             |

## Commands

```sh
cargo build            # build
cargo run -- <cmd>     # run, e.g. `cargo run -- status`
cargo test             # unit + integration + doc tests
cargo clippy --all-targets   # lint (keep this clean)
cargo fmt              # format
cargo fmt --check      # verify formatting in CI-style checks
```

Common subcommands: `say "<text>"`, `pet`, `feed`, `trick`, `fortune`,
`show`, `status`.

## Conventions

- **Edition 2021.** Keep `rustc`/`cargo` at the toolchain already in use.
- **Error handling:** never use `.unwrap()`/`.expect()` in library or binary
  code paths (tests are fine). Return `crate::error::Result<T>` and use `?`.
  Add new failure modes as variants on `PerritabonitaError`, using `#[from]` when
  wrapping a foreign error type.
- **Exhaustive matches:** match on enums (e.g. `Mood`) directly rather than
  stringly-typed keys, so the compiler enforces coverage when variants change.
- **Assets over hardcoding:** ASCII art goes in `assets/art/*.txt`; content
  lists (tricks, fortunes) go in `assets/*.toml`. Both are embedded with
  `include_str!`, so a rebuild picks up edits.
- **Presentation vs. logic:** keep colors in `theme.rs` and drawing in
  `art.rs`/`lib.rs`; keep `dog.rs`/`state.rs` free of ANSI/printing concerns.
- **Doc comments:** modules and public items have `//!`/`///` docs that often
  explain the _why_ and the Rust concept being demonstrated. Preserve and extend
  this style; it is intentional for the learning goal.

## Testing expectations

- Add unit tests in a `#[cfg(test)] mod tests` block within the module for pure
  logic (see `dog.rs`, `speech.rs`).
- Add integration tests in `tests/` that exercise only the public API.
- Before finishing a change, run `cargo test` and `cargo clippy --all-targets`
  and ensure both pass with no warnings, and run `cargo fmt`.

## Gotchas

- Perritabonita's saved state lives at the OS config dir (e.g.
  `~/.config/perritabonita/state.json`). Deleting it resets her to defaults. Avoid
  leaving demo state behind after manual runs; reset it if you created it.
- Art files contain Unicode braille characters — edit them as UTF-8 and do not
  "fix" the whitespace or blank-looking cells.
- Only two portraits exist (`perritabonita_happy.txt`, `perritabonita_sleepy.txt`). `art::portrait`
  maps every non-`Sleepy` mood to the happy pose; moods are told apart by
  `theme.rs` colors, not separate art.

## Scope

This is a personal, for-fun project. Do not add heavy dependencies, CI, or
large frameworks unless explicitly requested. Small, well-explained increments
are preferred. When in doubt, favor the approach that best teaches a Rust
concept.

## Stretch goals / roadmap

These are intended as learning exercises for the project owner, so **do not
implement them proactively** — only pick one up when explicitly asked. When you
do, keep each change small, well-tested, and heavy on explanatory doc comments.
The same list (with more prose) lives in `README.md`; keep the two in sync if you
edit either.

Roughly easy → hard:

1. **More content** — add tricks/fortunes to `assets/tricks.toml` /
   `assets/fortunes.toml`. No Rust changes needed.
2. **A new mood** — add a `Mood` variant (e.g. `Playful`). Let the compiler's
   exhaustiveness checks guide every `match` that must be updated
   (`dog.rs`, `art.rs`, `theme.rs`). Map it to an existing portrait in
   `art::portrait` (or add a new `assets/art/*.txt` if it deserves its own).
3. **Time decay** — store a "last interacted" timestamp in `Stats` and make
   hunger/energy drift over time (`SystemTime`, or the `time`/`chrono` crate).
4. **Config file** — let users rename Perritabonita or pick a favorite color via a TOML
   config loaded at startup.
5. **`perritabonita play` mini-game** — a guessing/fetch game using `rand` and a loop.
6. **Animation** — Perritabonita used to have a `show --animate` tail-wag animation
   (`crossterm`); it was removed for lack of good small-scale art. Re-add it
   with new frame art if you want the exercise.
7. **CLI polish** — shell completions via `clap_complete`.
8. **Errors with context** — try `anyhow` in the binary layer and compare the
   ergonomics against the current `thiserror` setup.

Remember the dependency guidance in **Scope**: prefer the standard library and
existing crates; only reach for a new dependency when the exercise calls for it.
