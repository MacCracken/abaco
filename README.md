# Abaco — Shared Math Engine for Rust

> Italian/Spanish: abacus

[![License](https://img.shields.io/badge/license-GPLv3-blue)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/abaco)](https://crates.io/crates/abaco)

**Abaco** is a Rust math engine providing expression evaluation, unit conversion, and numeric types. It is a shared library crate used by:

- **[Abacus](https://github.com/MacCracken/abacus)** — desktop calculator GUI and CLI for [AGNOS](https://github.com/MacCracken/agnosticos)
- **[Impetus](https://github.com/MacCracken/impetus)** — physics simulation engine (unit conversions, expression evaluation)

## Features

- **Expression evaluation** — arithmetic, 28+ math functions (sqrt, sin, cos, tan, log, ln, abs, ceil, floor, round, exp, and more), variables, scientific notation, percentage shorthand
- **Unit conversion** — 95+ built-in units across 14 categories (length, mass, temperature, time, data, speed, area, volume, energy, pressure, angle, frequency, force, power)
- **Natural language parsing** (feature-gated) — "what is 15% of 230", "convert 5 km to miles", "100 usd to eur"
- **Currency support** — 24 currency codes with live rates via hoosh (planned)
- **Calculation history** — track and recall previous results

## Architecture

```
abaco/src/
├── lib.rs    — module declarations, re-exports
├── core.rs   — Value types (Integer, Float, Fraction, Complex, Text), Unit, UnitCategory, Currency
├── eval.rs   — expression tokenizer, recursive descent parser, evaluator
├── units.rs  — unit registry with 95+ definitions, conversion engine
└── ai.rs     — NL math parsing, calculation history (feature = "ai")
```

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
abaco = "0.1"

# With natural language parsing:
# abaco = { version = "0.1", features = ["ai"] }
```

```rust
use abaco::{Evaluator, UnitRegistry};

// Evaluate expressions
let eval = Evaluator::new();
let result = eval.eval("2 + 3 * 4").unwrap();
assert_eq!(result.to_string(), "14");

// Convert units
let registry = UnitRegistry::new();
let result = registry.convert(100.0, "celsius", "fahrenheit").unwrap();
assert!((result.to_value - 212.0).abs() < 0.1);
```

## Feature Flags

| Feature | Description | Extra deps |
|---------|-------------|------------|
| `ai` | Natural language math parsing, calculation history | reqwest, tokio |
| `full` | All optional features | (same as ai) |

## AGNOS Integration

Abaco integrates with AGNOS through consumer applications:

- **Abacus** — desktop app with GUI, CLI, REPL, and MCP tools (`abaco_eval`, `abaco_convert`, `abaco_currency`, `abaco_history`, `abaco_units`)
- **hoosh API** (port 8088) — live currency exchange rates (planned)
- **agnoshi intents** — "calculate X", "convert X to Y", "how much is X in Y"

## License

GPL-3.0
