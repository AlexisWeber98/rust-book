# The Rust Programming Language — Exercises

My working code while reading [The Rust Programming Language](https://doc.rust-lang.org/book/) ("the book").

## Layout

One directory per chapter, named `chNN-chapter-topic/`, following the naming used
by the [official book repository](https://github.com/rust-lang/book). Inside each
chapter, one subdirectory per exercise or project.

```
chNN-chapter-topic/
└── exercise_name/
    ├── Cargo.toml
    └── src/main.rs
```

## Contents

| Chapter | Exercise | What it covers |
| --- | --- | --- |
| `ch01-getting-started/` | `hello_world/` | Compiling with `rustc`, no Cargo |
| | `hello_cargo/` | First Cargo project: `new`, `build`, `run`, `check` |
| `ch02-guessing-game/` | `guessing_game/` | `std::io`, the `rand` crate, `match`, `loop`, shadowing |

## Running

Cargo projects:

```bash
cd ch01-getting-started/hello_cargo
cargo run
```

Plain `rustc` files:

```bash
cd ch01-getting-started/hello_world
rustc main.rs && ./main
```

## Adding a new exercise

Create it with the VCS disabled — otherwise Cargo initializes a nested git
repository inside this one and breaks `git add`:

```bash
cargo new --vcs none ch03-common-concepts/variables
```

## Toolchain

Rust 2024 edition. Install via [rustup](https://rustup.rs/).
