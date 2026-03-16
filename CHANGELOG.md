# Changelog

All notable changes to Abaco will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [2026.3.16] - 2026-03-16

### Added — Initial Release

- **abaco-core**: Value types (Integer, Float, Fraction, Complex, Text), Unit/UnitCategory definitions, Currency type
- **abaco-eval**: Expression tokenizer and recursive descent evaluator with functions (sqrt, sin, cos, tan, log, ln, abs, ceil, floor, round, pi, e), variables, percent, power
- **abaco-units**: Unit conversion engine with 60+ built-in units across 8 categories (length, mass, temperature, time, data, speed, area, volume)
- **abaco-ai**: Natural language math parsing ("what is 15% of 230", "convert 5 km to miles"), calculation history
- **CLI**: `eval`, `convert`, and interactive REPL modes
- **CI/CD**: GitHub Actions for check, test, clippy, fmt, release (amd64 + arm64)
- **45+ tests** across all crates, 0 warnings
