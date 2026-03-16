# Abaco — AI-Native Calculator & Unit Converter

> Italian/Spanish: abacus

[![License](https://img.shields.io/badge/license-GPLv3-blue)](LICENSE)
[![Status](https://img.shields.io/badge/status-development-yellow)]()

**Abaco** is an AI-powered calculator and unit converter for [AGNOS](https://github.com/MacCracken/agnosticos). It provides natural language math, unit conversion, currency conversion, and scientific calculation.

## Features

- **Expression evaluation** — arithmetic, functions (sqrt, sin, cos, tan, log, ln, abs, ceil, floor, round), variables, parentheses, power, modulo
- **Unit conversion** — 60+ units across 8 categories (length, mass, temperature, time, data, speed, area, volume)
- **Natural language** — "what is 15% of 230", "convert 5 km to miles", "100 usd to eur"
- **Currency conversion** — 24 currency codes with live rates via hoosh (planned)
- **Calculation history** — track and recall previous results
- **Interactive REPL** — eval-print loop for quick calculations
- **MCP tools** — 5 native tools for agent-driven queries

## Architecture

```
abaco
├── abaco-core    — Value types, Unit definitions, Currency
├── abaco-eval    — Expression tokenizer and recursive descent evaluator
├── abaco-units   — Unit conversion engine with 60+ built-in units
└── abaco-ai      — Natural language parsing, history
```

## Usage

```bash
# Evaluate an expression
abaco eval "2 + 3 * 4"
# => 14

# Unit conversion
abaco convert "5 km to miles"
# => 5 km = 3.10686 mi

# Interactive REPL (no arguments)
abaco
abaco> sqrt(144)
12
abaco> what is 15% of 230
34.5
abaco> convert 100 celsius to fahrenheit
100 C = 212 F
abaco> history
abaco> quit
```

## AGNOS Integration

Abaco integrates with AGNOS through:

- **hoosh API** (port 8088) — live currency exchange rates
- **MCP tools** — `abaco_eval`, `abaco_convert`, `abaco_currency`, `abaco_history`, `abaco_units`
- **agnoshi intents** — "calculate X", "convert X to Y", "how much is X in Y"

## License

GPL-3.0
