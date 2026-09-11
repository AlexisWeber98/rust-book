# The Rust Programming Language — Exercises

My working notes and code while reading [The Rust Programming Language](https://doc.rust-lang.org/book/) ("the book").

Each directory is a standalone exercise or project from the chapters.

## Contents

| Path | Chapter | What it covers |
| --- | --- | --- |
| `mainfile/` | 1.2 | `rustc` compiled "Hello, world!" without Cargo |
| `hello_cargo/` | 1.3 | First Cargo project: `cargo new`, `build`, `run`, `check` |
| `projects/guessing_game/` | 2 | Guessing game: `std::io`, the `rand` crate, `match`, `loop`, shadowing |

## Running

Cargo projects:

```bash
cd hello_cargo
cargo run
```

Plain `rustc` files:

```bash
cd mainfile
rustc main.rs && ./main
```

## Toolchain

Rust 2024 edition. Install via [rustup](https://rustup.rs/).
