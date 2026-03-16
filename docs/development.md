# Development

## Prerequisites

- Rust 1.85+ (edition 2024)
- cargo

## Build

```bash
cargo build
cargo build --release
```

## Test

```bash
cargo test --workspace
```

## Lint

```bash
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

## Run

```bash
# Evaluate expression
cargo run -- eval "2 + 3 * 4"

# Unit conversion
cargo run -- convert "5 km to miles"

# Interactive REPL
cargo run
```

## Release

Releases are triggered by pushing a git tag:

```bash
./bump-version.sh 2026.3.17
git add -A
git commit -m "release 2026.3.17"
git tag 2026.3.17
git push origin main --tags
```

The release workflow builds binaries for x86_64 and aarch64 Linux and creates a GitHub release.

## Project Structure

```
abaco/
├── Cargo.toml          — Workspace root
├── VERSION             — Single source of truth for version
├── bump-version.sh     — Version bump script
├── src/main.rs         — CLI binary
├── crates/
│   ├── abaco-core/     — Core types
│   ├── abaco-eval/     — Expression evaluator
│   ├── abaco-units/    — Unit conversion
│   └── abaco-ai/       — NL parsing, history
├── docs/               — Documentation
└── .github/workflows/  — CI/CD
```
