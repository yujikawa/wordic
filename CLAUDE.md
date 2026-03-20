# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build --release

# Run tests
cargo test --release --all-features

# Run a single test
cargo test <test_name>

# Lint
cargo fmt --all -- --check
cargo clippy

# Install locally
cargo install --path .

# Run directly
cargo run -- <subcommand>
```

## Architecture

Wordic is a CLI tool (Rust, Edition 2021) for managing a personal word dictionary stored as JSON at `~/.wordic/wordic.json`. It follows a clean layered architecture:

**Layers:**
- `domain/` — `Dictionary` entity (key, value, description, timestamp). Pure domain logic.
- `application/` — `DictionaryApplicationService<T>` orchestrates use cases (init, register, show, get, remove). Generic over repository type `T` via trait.
- `infrastructure/` — `IDictionaryRepository` trait + `DictionaryJsonRepository` concrete implementation for JSON file I/O.
- `functions/` — `display()` helper with ANSI color output (green/red).
- `main.rs` — `clap`-based CLI entry point; parses subcommands (`Init`, `Show`, `Add`, `Get`, `Rm`) and wires up the service with the JSON repository.

**Data flow:**
```
CLI args (clap) → DictionaryApplicationService → IDictionaryRepository → ~/.wordic/wordic.json
```

The repository trait enables dependency injection, which is how the unit test in `infrastructure/repository.rs` works.

## CI/CD

- Tests run on push to `develop` or `feature/**` branches (`.github/workflows/test.yml`)
- Publishing to crates.io is triggered by git tags (`.github/workflows/publish.yml`)
