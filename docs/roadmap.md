# Roadmap

> **Abaco** is the shared math library crate. GUI/CLI/MCP moved to [abacus](https://github.com/MacCracken/abacus).
> Higher math (linear algebra, calculus, geometry, numerical methods) lives in [ganit](https://github.com/MacCracken/ganit).

## Scope

Abaco owns **expression evaluation** and **unit conversion** — the primitives that every math-consuming app needs. It does NOT own:

- **Linear algebra, matrices, quaternions** → ganit-core (wraps glam)
- **Geometry, spatial queries, intersections** → ganit-geo
- **Calculus, integration, Bezier curves** → ganit-calc
- **Root finding, solvers, FFT** → ganit-num
- **Physics simulation** → impetus (wraps rapier)
- **GUI, CLI, MCP tools** → abacus (consumes abaco)

## V1 — Core Expression Engine & Units (done)

- Recursive descent expression evaluator
- 95+ built-in units across 14 categories
- 28+ math functions (trig, hyperbolic, log, rounding, etc.)
- Temperature conversion with offset support
- Scientific notation, percentage shorthand
- Variable assignment and recall
- Natural language math parsing (feature-gated `ai`)
- 90+ tests, 0 warnings

## V1.1 — Performance (done, 2026-03-22)

- Bytes-based tokenizer: 43-62% faster expression evaluation
- HashMap unit index: 94-98% faster lookups (by-name, case-insensitive, plural)
- One-time registry creation cost (24us) for amortized lookup wins
- 27 criterion benchmarks

## V2 — Extended Units & Currency

- Live currency exchange rates via hoosh (port 8088)
- Rate caching and offline fallback
- Binary vs SI data size distinction (KiB/MiB vs kB/MB)
- Fuel economy units (mpg, L/100km, km/L)
- Density, luminosity, viscosity units
- Unit aliases and abbreviation normalization

## V3 — Expression Engine Enhancements

- Implicit multiplication (`2(3 + 4)`, `2pi`)
- Factorial, gcd, lcm functions
- Statistical functions (mean, median, stddev) for list expressions
- Conversion history persistence (JSON)
- LaTeX output for expressions
- Live-as-you-type evaluation support (partial parse, error recovery)

## Boundary with Ganit

When a feature involves **evaluating user-typed math expressions or converting units**, it belongs in abaco. When it involves **programmatic math operations on typed vectors/matrices/curves**, it belongs in ganit.

| Feature | abaco | ganit |
|---------|-------|-------|
| `eval("sin(pi/4)")` | Yes | — |
| `convert(100, "km", "miles")` | Yes | — |
| `Vec3::cross(a, b)` | — | ganit-core |
| `ray_sphere_intersection()` | — | ganit-geo |
| `integral_simpson(f, 0, 1, 100)` | — | ganit-calc |
| `newton_raphson(f, df, x0)` | — | ganit-num |
| `eval("solve x^2 + 3x - 4")` | Parse expression | ganit-num solves it |

Abaco may depend on ganit in the future for advanced expression evaluation (e.g. symbolic simplification), but ganit should never depend on abaco.
