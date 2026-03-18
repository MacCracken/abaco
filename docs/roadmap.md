# Roadmap

## MVP — Core Calculator & Units (current)

### Done
- Expression evaluator with recursive descent parser
- 75+ built-in units across 10 categories (length, mass, temperature, time, data, speed, area, volume, energy, pressure)
- Temperature conversion with offset support
- Natural language parsing for math and conversions
- CLI with eval, convert, list, and REPL modes
- Calculation history (in-memory, capped)
- Energy units (joule, calorie, kWh, BTU, eV)
- Pressure units (pascal, bar, atm, psi, mmHg, torr)
- Multi-arg functions: min, max, log2, pow, atan2
- Variable assignment in REPL (`x = 5`, then use `x` in expressions)
- `list` subcommand to browse available units by category
- MCP tool server (`abaco_eval`, `abaco_convert`, `abaco_currency`, `abaco_history`, `abaco_units`)
- Percentage shorthand in evaluator (e.g. `15%` as `0.15`)
- 74+ tests, 0 warnings

## Post-V1 — GUI & Live Currency

- egui desktop interface with expression input and result display
- Unit conversion picker with category browser
- Live currency exchange rates via hoosh (port 8088)
- Rate caching and offline fallback
- Conversion history with search and persistence
- Graph plotting for functions

## Future — Advanced Math & NL

- Symbolic algebra (simplify, expand, factor)
- Equation solving (linear, quadratic)
- Matrix operations
- Statistical functions (mean, median, stddev)
- Advanced NL via hoosh LLM ("solve x^2 + 3x - 4 = 0")
- Function graphing with interactive zoom
- LaTeX output
