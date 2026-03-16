# Architecture

## Crate Structure

```
abaco (binary)
├── abaco-core     — Shared types
│   ├── Value      — Integer, Float, Fraction, Complex, Text
│   ├── Unit       — Name, symbol, category, conversion factors
│   ├── UnitCategory — Length, Mass, Temperature, Time, DataSize, Speed, Area, Volume, Energy, Pressure
│   ├── ConversionResult
│   └── Currency   — Code, name, symbol
│
├── abaco-eval     — Expression evaluation
│   ├── Token      — Lexer tokens
│   ├── tokenize() — Expression tokenizer
│   └── Evaluator  — Recursive descent parser with variables
│       ├── parse_expr → parse_term → parse_power → parse_unary → parse_primary
│       └── Built-in functions: sqrt, sin, cos, tan, log, ln, abs, ceil, floor, round, exp
│
├── abaco-units    — Unit conversion
│   └── UnitRegistry
│       ├── 60+ built-in units across 8 categories
│       ├── convert(value, from, to) — with temperature offset support
│       ├── find_unit() — case-insensitive, plural-aware lookup
│       └── list_units(category)
│
└── abaco-ai       — Natural language interface
    ├── NlParser   — NL to structured query
    │   ├── ParsedQuery::Calculation
    │   ├── ParsedQuery::Conversion
    │   └── ParsedQuery::CurrencyConversion
    └── CalculationHistory — capped result history
```

## Data Flow

1. User input (CLI arg or REPL line)
2. `NlParser::parse_natural()` classifies the query
3. Route to `Evaluator::eval()` or `UnitRegistry::convert()`
4. Result formatted and displayed
5. Entry added to `CalculationHistory`

## Dependencies

- `abaco-eval` depends on `abaco-core`
- `abaco-units` depends on `abaco-core`
- `abaco-ai` depends on `abaco-core`, `abaco-eval`, `abaco-units`
- Binary depends on all four crates
