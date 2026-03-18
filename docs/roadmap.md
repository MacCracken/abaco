# Roadmap

## V1 — Core Calculator, Units & GUI (done)

- Expression evaluator with recursive descent parser
- 95+ built-in units across 14 categories (length, mass, temperature, time, data, speed, area, volume, energy, pressure, angle, frequency, force, power)
- Temperature conversion with offset support
- Natural language parsing for math and conversions (GUI + CLI)
- CLI with eval, convert, list, REPL, and `--gui` flag
- Calculation history (in-memory, capped)
- Multi-arg functions: min, max, log2, pow, atan2
- Hyperbolic trig: sinh, cosh, tanh, asinh, acosh, atanh
- Degree/radian helpers: deg(), rad()
- Utility functions: sign, trunc, fract
- Scientific notation support (1e3, 1.5e-3, 2E+6)
- Percentage shorthand in evaluator (e.g. `15%` as `0.15`)
- Variable assignment in REPL and GUI (`x = 5`, then use `x`)
- `list` subcommand to browse available units by category
- MCP tool server (`abaco_eval`, `abaco_convert`, `abaco_currency`, `abaco_history`, `abaco_units`)
- egui desktop GUI with calculator, unit converter, history, and function plotter
- AGNOS dark theme
- 90+ tests, 0 warnings

## Post-V1 — Live Currency & Persistence

- Live currency exchange rates via hoosh (port 8088)
- Rate caching and offline fallback
- Conversion history persistence (JSON file)
- History search and filtering
- Session persistence (variables, plot state)
- Settings/preferences panel
- Binary vs SI data size distinction (KiB/MiB vs kB/MB)

## Future — Advanced Math & NL

- Symbolic algebra (simplify, expand, factor)
- Equation solving (linear, quadratic)
- Matrix operations
- Statistical functions (mean, median, stddev)
- Factorial, gcd, lcm
- Advanced NL via hoosh LLM ("solve x^2 + 3x - 4 = 0")
- Function graphing with interactive zoom
- LaTeX output
- Fuel economy units (mpg, L/100km, km/L)
- Density, luminosity, viscosity units
- Implicit multiplication (e.g. `2(3 + 4)`)
- Live-as-you-type evaluation
