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
| [`ntheory`](src/ntheory.cyr) | `is_prime` (Miller–Rabin), `next_prime` / `prev_prime`, `factor`, `totient`, `fibonacci`, `binomial` |
| [`dsp`](src/dsp.cyr)       | dB ↔ amplitude, MIDI ↔ frequency, envelope time constants, PolyBLEP, panning, crossfade, Hann / Hamming / Blackman / Kaiser windows, cubic / sinc interpolation, chromagram helpers, SIMD batch ops, samples ↔ ms, BPM ↔ Hz |
| [`eval`](src/eval.cyr)     | Tokenizer + recursive-descent parser, 43+ functions, variables, implicit multiplication, `%` operator, scientific notation, `eval_partial` for live-as-you-type feedback |
| [`units`](src/units.cyr)   | 112 built-in units in 19 categories, 80+ aliases, O(1) hashmap lookup, reciprocal units (L/100km), pitch (semitones / cents / octaves), BPM via frequency |
| [`ai`](src/ai.cyr)         | Natural-language parsing (`"convert 5 km to miles"`, `"what is 15% of 230"`), bounded calculation history, currency cache + `http_get`-driven fetch for live rates via hoosh |

## Quick start

Requires Cyrius `4.8.x`.

```bash
# Build the library (currently ships as a module set; link into your
# program via `include "src/*.cyr"` and a cyrius.toml with the same
# stdlib deps).
cyrius build src/main.cyr build/abaco

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
# your-project/cyrius.toml
[deps]
stdlib = ["string", "fmt", "alloc", "vec", "str", "syscalls", "tagged",
          "hashmap", "fnptr", "math", "io", "net", "http", "json"]

[deps.abaco]
path = "../abaco"
tag  = "2.0.0"
modules = ["src/core.cyr", "src/ntheory.cyr", "src/dsp.cyr",
           "src/eval.cyr", "src/units.cyr", "src/ai.cyr"]
```

Then use the public API directly:

```cyr
include "src/core.cyr"
include "src/eval.cyr"
include "src/units.cyr"

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

Case-insensitive lookup. Plural forms (`meters`, `kilograms`). Multi-
word aliases (`"square kilometers"`, `"miles per gallon"`). Symbol
case preserved (`mW` vs `MW`). Reciprocal units (L/100km). Tempo
(BPM) routes through frequency so `120 bpm -> 2 Hz` works.

## Ecosystem

Abaco sits at the base of a stack:

- **abacus** — desktop calculator / CLI / REPL GUI (rebuilding; consumes abaco)
- **hisab** — physics + symbolic algebra + high math (consumes abaco)
- **dhvani** — audio DSP pipelines (consumes `abaco::dsp`)
- **abaco** — math primitives (this repo)
- **cyrius** — language + stdlib

Abaco does not bundle a GUI, a CLI, or an MCP server. Those live in
consumer projects and call into abaco. This keeps the engine
headless, stable, and reusable.

## Status

- **v2.0.0** — Rust → Cyrius port complete (breaking: not a Rust crate anymore)
- **381 tests** passing (`cyrius test`)
- **3 fuzz harnesses** (eval, ntheory, units) — run clean at 10k+ iters
- **56 benchmarks** — CSV history tracked in `bench-history.csv`
- **100% of the public API** is documented (`cyrius doc --check`)

Known gaps live in [ROADMAP.md](ROADMAP.md).

## License

AGPL-3.0-only. See [LICENSE](LICENSE).
