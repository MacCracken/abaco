# Changelog

All notable changes to Abaco will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.0] - 2026-03-22

### Changed — Flatten to shared math crate

- **Refactored** from multi-crate workspace to single flat library crate
- **Extracted** GUI (abaco-gui) and binary (main.rs, MCP server, REPL) to [abacus](https://github.com/MacCracken/abacus)
- **Switched** to SemVer 0.D.M versioning for crates.io publication
- **Feature-gated** AI module behind `ai` feature flag
- **Added** rustls-tls to reqwest (was missing)
- **Removed** binary deps (clap, anyhow, tracing-subscriber) — library only

### Modules (unchanged)

- `core` — Value types (Integer, Float, Fraction, Complex, Text), Unit/UnitCategory (14 categories), Currency
- `eval` — Tokenizer, recursive descent parser, evaluator with 28+ functions, variables, scientific notation, percentage shorthand
- `units` — Unit registry with 95+ built-in units across 14 categories, conversion engine
- `ai` — Natural language math parsing, calculation history (feature-gated)

## [2026.3.18] - 2026-03-18

### Added — First Release

- **abaco-core**: Value types (Integer, Float, Fraction, Complex, Text), Unit/UnitCategory definitions (14 categories), Currency type
- **abaco-eval**: Expression tokenizer and recursive descent evaluator with 28 functions (sqrt, sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, log, ln, log2, abs, ceil, floor, round, trunc, fract, sign, exp, deg, rad, min, max, pow, atan2), variables, percentage shorthand, scientific notation, power operator
- **abaco-units**: Unit conversion engine with 95+ built-in units across 14 categories (length, mass, temperature, time, data, speed, area, volume, energy, pressure, angle, frequency, force, power)
- **abaco-ai**: Natural language math parsing ("what is 15% of 230", "convert 5 km to miles"), calculation history
- **abaco-gui**: egui/eframe desktop GUI with calculator (NL + math), unit converter with category browser, history view, function plotter (f(x) graphing), AGNOS dark theme
- **CLI**: `eval`, `convert`, `list`, interactive REPL, and `--gui` flag for desktop mode
- **MCP**: JSON-RPC tool server with `abaco_eval`, `abaco_convert`, `abaco_currency`, `abaco_history`, `abaco_units` tools
- **CI/CD**: GitHub Actions with check, lint, security audit, test, build, release (amd64 + arm64)
- **90+ tests** across all crates, 0 warnings
