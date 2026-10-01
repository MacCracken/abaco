# Abaco — Math Engine

> Italian / Spanish: *abacus*

**Abaco** is the math engine for the AGNOS ecosystem. Expression
evaluation, unit conversion, DSP primitives, number theory, and
natural-language parsing — all headless, all in [Cyrius][cyrius].

Abaco is a **backend**. Frontends (desktop calculator, CLI, REPL,
MCP bridges) live in other projects and consume abaco. If you want a
math engine you can wrap any way you like, this is it.

[cyrius]: https://github.com/MacCracken/cyrius

## Modules

| Module | What it does |
|--------|--------------|
| [`core`](src/core.cyr)     | `Value` (Integer / Float / Fraction / Complex / Text), `Unit`, `UnitCategory` (19 categories), `Currency`, `ConversionResult` |
| [`ntheory`](src/ntheory.cyr) | `is_prime` (deterministic Miller–Rabin), `next_prime` / `prev_prime`, `factor`, `totient`, `mod_pow`, `abaco_binomial` (exact whenever the result fits in i64) |
| [`dsp`](src/dsp.cyr)       | dB ↔ amplitude, MIDI ↔ frequency, envelope time constants, PolyBLEP, panning, crossfade, Hann / Hamming / Blackman / Kaiser windows (symmetric, and periodic / DFT-even for STFT), cubic / sinc interpolation, chromagram helpers, SIMD batch ops, samples ↔ ms, BPM ↔ Hz |
| [`eval`](src/eval.cyr)     | Tokenizer + recursive-descent parser, 43+ functions, variables, implicit multiplication, `%` operator, scientific notation, `eval_partial` for live-as-you-type feedback |
| [`units`](src/units.cyr)   | Built-in units in 19 categories with factors exact to the definition (NIST SP 811), O(1) hashmap lookup, reciprocal units (L/100km), pitch (semitones / cents / octaves), BPM via frequency |
| [`ai`](src/ai.cyr)         | Natural-language parsing (`"convert 5 km to miles"`, `"what is 15% of 230"`), bounded calculation history with JSON save/load, currency cache + a plaintext fetch from a loopback rates server (no TLS yet) |

## Quick start

Requires the Cyrius toolchain pinned in `cyrius.cyml` (`[package].cyrius`).

```bash
# Vendor the pinned stdlib into lib/ (a gitignored build artifact)
cyrius deps

# Build the smoke binary
cyrius build src/main.cyr build/abaco

# Regenerate the consumer bundle dist/abaco.cyr
cyrius distlib

# Run the demo
cyrius run programs/basic.cyr

# Run tests (auto-discovers tests/*.tcyr)
cyrius test

# Run benchmarks
cyrius bench benches/bench.bcyr

# Run all fuzz harnesses
./fuzz/run.sh 10000
```

## Consuming abaco from your own Cyrius project

```
# your-project/cyrius.cyml
[deps]
stdlib = ["string", "fmt", "alloc", "vec", "str", "syscalls", "tagged",
          "hashmap", "fnptr", "math", "ganita", "io", "net", "http", "bayan"]

[deps.abaco]
git = "https://github.com/MacCracken/abaco.git"
tag = "2.4.11"
modules = ["dist/abaco.cyr"]   # self-contained library bundle
```

`cyrius deps` vendors the bundle and reads `dist/abaco.deps` beside it — the
stdlib modules the bundle needs — and vendors those too, so from 2.4.11 the
`[deps] stdlib` line above is optional (repeating it is harmless: each module
is vendored once). Keep it for a tag before 2.4.11, or if you include
`dist/abaco.cyr` without `cyrius deps`. There is nothing to `include` from
abaco's `src/`. Then call the public API directly:

```cyr
fn main() {
    alloc_init();
    var e = Evaluator_new();

    # Evaluate expressions.
    var r = Evaluator_eval(e, "2 + 3 * 4");        # -> 14 (f64 bits)
    var r2 = Evaluator_eval(e, "sqrt(144) + sin(pi/2)");  # -> 13

    # Variables.
    Evaluator_set_variable(e, "x", f64_from(42));
    var r3 = Evaluator_eval(e, "x^2 + 1");         # -> 1765

    # Unit conversion.
    var reg = UnitRegistry_new();
    var r4 = UnitRegistry_convert(reg, f64_from(100), "C", "F");  # -> 212
    var r5 = UnitRegistry_convert(reg, f64_from(120), "bpm", "Hz"); # -> 2
    return 0;
}
```

See [`programs/basic.cyr`](programs/basic.cyr) for a runnable end-to-end
example.

## Supported functions (43+)

**Basic** `sqrt`, `abs`, `exp`, `sign`/`sgn`, `deg`, `rad`, `min`, `max`, `pow`, `gcd`, `lcm`, `factorial`, `n!`
**Trig** `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`
**Hyperbolic** `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`
**Logs** `log` (log10), `ln`, `log2`
**Rounding** `ceil`, `floor`, `round`, `trunc`, `fract`
**Statistical** `mean` / `avg`, `median`, `stddev` / `stdev`
**Number theory** `isprime`, `nextprime`, `prevprime`, `totient`, `fibonacci` / `fib`, `binomial` / `choose`

## Unit categories (19)

Length, Mass, Temperature, Time, Data Size (SI + IEC), Speed, Area,
Volume, Energy, Pressure, Angle, Frequency, Force, Power, Fuel Economy,
Density, Luminosity, Viscosity, Pitch.

Names and aliases match in any case (`Kilometers`, `FEET`), with plurals
(`meters`, `tonnes`) and multi-word aliases (`"square kilometers"`,
`"miles per gallon"`). Symbols match **exactly as written**, because SI and
IEC symbols are case-sensitive: `MW` is megawatt and `mW` milliwatt, and an
unregistered symbol such as `mHz` or `Mb` is unknown rather than a near miss.
Non-SI abbreviations (`mi`, `lb`, `gal`, `MPH`) match in any case, and common
lowercase spellings (`hz`, `ml`, `kw`, `c`, `f`) are accepted. Reciprocal
units (L/100km). Tempo (BPM) routes through frequency so `120 bpm -> 2 Hz`
works.

## Ecosystem

Abaco sits at the base of a stack:

- **abacus** — desktop calculator / CLI / REPL GUI (rebuilding; consumes abaco)
- **hisab** — physics + symbolic algebra + high math (*sibling* library, distinct
  domain — not a consumer)
- **dhvani** — audio DSP pipelines (consumes `abaco::dsp`)
- **abaco** — math primitives (this repo)
- **cyrius** — language + stdlib

Abaco does not bundle a GUI, a CLI, or an MCP server. Those live in
consumer projects and call into abaco. This keeps the engine
headless, stable, and reusable.

## Status

- **v2.0.0** — Rust → Cyrius port complete (breaking: not a Rust crate anymore)
- 7 test suites, 4 fuzz harnesses (eval, ntheory, units, ai) and a benchmark
  CSV trail (`bench-history.csv`). Current counts, version, binary size and
  consumers are in [`docs/development/state.md`](docs/development/state.md).

Planned work lives in [`docs/development/roadmap.md`](docs/development/roadmap.md).

## License

GPL-3.0-only. See [LICENSE](LICENSE).
