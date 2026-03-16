# Roadmap

## Phase 1 — Core Calculator & Units (current)

- Expression evaluator with recursive descent parser
- 60+ built-in units across 8 categories
- Temperature conversion with offset support
- Natural language parsing for math and conversions
- CLI with eval, convert, and REPL modes
- Calculation history
- 45+ tests, 0 warnings

## Phase 2 — GUI & Live Currency

- egui desktop interface with expression input and result display
- Unit conversion picker with category browser
- Live currency exchange rates via hoosh (port 8088)
- Rate caching and offline fallback
- Conversion history with search
- Graph plotting for functions

## Phase 3 — Advanced Math & NL

- Symbolic algebra (simplify, expand, factor)
- Equation solving (linear, quadratic)
- Matrix operations
- Statistical functions (mean, median, stddev)
- Advanced NL via hoosh LLM ("solve x^2 + 3x - 4 = 0")
- Function graphing with interactive zoom
- LaTeX output
