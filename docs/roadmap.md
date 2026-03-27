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

## 0.23 — Extended Units & Currency (shipped)

- Live currency exchange rates via hoosh (port 8088)
- Rate caching and offline fallback
- Fuel economy units (mpg, L/100km, km/L)
- Density, luminosity, viscosity units (18 categories total)
- Unit aliases and abbreviation normalization (80+ aliases)
- Reciprocal unit conversion engine (L/100km)

## 0.24 — Expression Engine Enhancements (shipped)

- Implicit multiplication (`2(3 + 4)`, `2pi`, `(2)(3)`)
- Factorial (`n!` postfix + `factorial(n)`), `gcd`, `lcm`
- Statistical functions (`mean`, `median`, `stddev`) — variable arity
- Conversion history persistence (JSON save/load)
- LaTeX output (`Value::to_latex()`)
- Live-as-you-type evaluation (`eval_partial` with error recovery)
- `Token::Bang` for postfix factorial

## 1.0.0 — Stable Public API (shipped 2026-03-27)

- Stable public API — no breaking changes without major version bump
- 320 tests + 2 doctests, 56 benchmarks
- 120+ units, 18 categories, 80+ aliases, 35+ functions
- `#[non_exhaustive]` on all public enums, `#[must_use]` on all pure functions
- Published to crates.io

---

## 1.1.0 — hisab Integration + Number Theory

Depends on hisab 1.4.0 shipping its number theory + symbolic extensions.

### Solver bridge (feature-gated: `solver`)
- [ ] Optional dependency on hisab (`dep:hisab`)
- [ ] `eval("solve x^2 - 2 = 0")` → parse equation → dispatch to hisab's `newton_raphson` / `bisection`
- [ ] `eval("factor 1234567")` → hisab's integer factorization
- [ ] `eval("isprime 104729")` → hisab's primality test (Miller-Rabin)

### Symbolic algebra bridge (feature-gated: `symbolic`)
- [ ] abaco `Value` ↔ hisab `Expr` conversion
- [ ] `eval("simplify sin(x)^2 + cos(x)^2")` → hisab symbolic simplification → `1`
- [ ] `eval("diff x^3 + sin(x)")` → hisab symbolic differentiation
- [ ] `eval("integrate x^2")` → hisab symbolic integration

### Verified evaluation (feature-gated: `interval`)
- [ ] Wrap numeric results in hisab `Interval` for guaranteed error bounds
- [ ] Display uncertainty: `"pi = 3.14159... ± 1e-15"`

### Number theory functions (native, no hisab dep) — shipped 1.1.0
- [x] `isprime(n)` — deterministic Miller-Rabin for all u64
- [x] `nextprime(n)`, `prevprime(n)`
- [x] `factor(n)` — prime factorization via trial division
- [x] `totient(n)` — Euler's totient
- [x] `fibonacci(n)` — fast doubling, `binomial(n, k)` — overflow-safe

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
