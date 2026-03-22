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

## 0.23 — Extended Units & Currency

- Live currency exchange rates via hoosh (port 8088)
- Rate caching and offline fallback
- Fuel economy units (mpg, L/100km, km/L)
- Density, luminosity, viscosity units
- Unit aliases and abbreviation normalization

## 0.24 — Expression Engine Enhancements

- Implicit multiplication (`2(3 + 4)`, `2pi`)
- Factorial, gcd, lcm functions
- Statistical functions (mean, median, stddev) for list expressions
- Conversion history persistence (JSON)
- LaTeX output for expressions
- Live-as-you-type evaluation support (partial parse, error recovery)

## 1.0.0 — Stable Public API

All of the above complete, plus:

- Stable public API for `Evaluator`, `UnitRegistry`, `Value`, `NlParser`
- Comprehensive documentation with examples for all public types
- No breaking changes from this point forward
- Published to crates.io

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
