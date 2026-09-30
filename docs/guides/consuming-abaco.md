# Consuming abaco

How to depend on abaco from another Cyrius project (hisab, dhvani, the Abacus
desktop app, …). abaco ships a single self-contained bundle, `dist/abaco.cyr` —
that file *is* the public surface (see
[ADR 0001](../adr/0001-dist-bundle-distribution.md)).

## 1. Declare the dependency

In your project's `cyrius.cyml`:

```toml
[deps.abaco]
git = "https://github.com/MacCracken/abaco.git"
tag = "2.4.8"                 # pin to a released tag, never a branch
modules = ["dist/abaco.cyr"]  # the bundle is the only file you name
```

`cyrius deps` resolves it into your `lib/`.

## 2. Provide the stdlib surface

The bundle carries **no** `include "lib/…"` lines — abaco does not dictate your
stdlib set. List the stdlib modules abaco's code needs in your own `[deps]`:

```toml
[deps]
stdlib = ["string", "fmt", "alloc", "vec", "str", "syscalls", "tagged",
          "hashmap", "fnptr", "math", "ganita", "io", "net", "http", "bayan"]
```

(Drop `net` / `http` only if you never touch the currency-cache path — `bayan`
is still needed for `bayan_u64_powmod` in `ntheory`.)

> **Changed at 6.2.x** — the standalone `json` and `u128` modules were folded
> into **`bayan`**, and the extended transcendentals abaco calls
> (`f64_asinh`, `f64_atan2`, `fibonacci`, `binomial`, …) moved from `math` into
> **`ganita`**. If you copied the old `[…, "json", "u128"]` list from a
> pre-2.2.5 README, `cyrius deps` will fail to resolve those two names and the
> `ganita` symbols will come up undefined at link time.

## 3. Use the API

A consumer includes the bundle (or, in-repo, the `src/` modules) and calls the
public functions directly.

```cyr
fn main() {
    alloc_init();

    # Expression evaluation
    var e = Evaluator_new();
    Evaluator_set_variable(e, "x", f64_from(7));
    var v = Evaluator_eval(e, "2 * x + 1");        # -> 15.0

    # Unit conversion
    var r = UnitRegistry_new();
    var km = UnitRegistry_convert(r, f64_from(5), "mi", "km");  # 5 mi -> km

    # Number theory
    var p = is_prime(1000000007);                  # -> 1

    # DSP math
    var db = amplitude_to_db(f64_from(1));         # -> 0.0 dBFS
    return 0;
}
```

## Surface map

| Domain | Entry points | Module |
|--------|-------------|--------|
| Expressions | `Evaluator_new`, `Evaluator_set_variable`, `Evaluator_eval`, `Evaluator_eval_partial` | `eval` |
| Units | `UnitRegistry_new`, `UnitRegistry_convert`, `UnitRegistry_find`, `UnitRegistry_list` | `units` |
| Number theory | `is_prime`, `next_prime`, `prev_prime`, `factor`, `totient` | `ntheory` |
| DSP | windows, `amplitude_to_db`, MIDI↔freq, interpolation, chromagram, batch ops | `dsp` |
| Values | `Value_*`, `Unit`, `ConversionResult` | `core` |
| NL / history / currency | `nl_parse`, `CalcHistory_*`, `CurrencyCache_*` | `ai` |

## Upgrading

Bump the `tag` and run `cyrius deps`. abaco follows SemVer (post-1.0): patch and
minor bumps are source-compatible; a major bump documents breaking changes in
[`CHANGELOG.md`](../../CHANGELOG.md) with a migration section.

### 2.4.8 — pitch classes move; two functions now report errors

- **`freq_to_pitch_class` / `freq_to_octave` answer differently for off-centre
  tones** — correctly now. The C0 reference was encoded as 16.703125 Hz rather
  than 16.3516 Hz, so every result was 0.368 semitone flat and a tone more than
  ~0.13 semitone flat of a note came back as the note below (435 Hz as G#, not
  A). In-tune notes are unchanged. If you compensated for the old skew — a
  tolerance, an offset — remove it. dhvani in particular should re-vendor.
- **`binomial` / `choose` and `lcm` set `ABACO_ERR_MATH`** where they used to
  return a value with no error: `binomial` for a negative argument or a result
  ganita cannot hold (it answered -1), and `lcm` for a result past i64 (it
  wrapped). ganita also refuses some results that fit — any C(n, k) above
  `i64_MAX / k`, from C(62, 31) — so those are errors too until ganita lifts
  the limit. Every other result is unchanged.

### 2.4.7 — re-vendor, whatever your Cyrius pin

The 2.4.6 bundle gives wrong answers — and can hang or trap — when you compile
it with Cyrius 6.6.8 or later, because that toolchain changed what `f64_to`
returns for NaN, ±Inf and |x| ≥ 2^63: it saturates now, where x86 used to
answer `i64_MIN` for all of them. `nextprime(1e300)` never returns,
`lcm(-1e300, 1e300)` dies with SIGFPE, `(-1)^(2^63)` is -1 and `(sqrt(-1))!` is
1. 2.4.7 is correct under either convention. On aarch64, where `f64_to` has
always saturated, these were live at every pin.

One behaviour change to handle: `gcd`, `lcm`, `binomial` and `choose` now set
`ABACO_ERR_MATH` for a NaN, ±Inf or out-of-range argument (|x| ≥ 2^63, which
for `gcd` / `lcm` includes -(2^63)), where they used to return garbage with no
error. In-range results are unchanged. And `CurrencyCache_fetch` refuses a base
URL whose host only *starts* with `localhost` or `127.0.0.1`
(`http://localhost.example.net`) with `AI_ERR_HTTP`.

Two further changes come from the toolchain, not the bundle, so they follow
your own pin: from Cyrius 6.6.9, `sin` / `cos` / `tan` are within 1 ulp for
every argument (x86 used to answer `sin(1e300) = 1e300`), and from 6.6.8 unary
minus on zero gives -0, so `(-0)^(-1)` is -Inf as C99 requires. Pin 6.6.12 to
match the toolchain abaco's own gate runs.

### 2.4.0 — larger expressions, correctly-rounded literals

Two user-visible changes, both improvements — nothing to migrate:

- **Expressions up to 1024 tokens** parse (was 512). Costs 16 KB per evaluation
  instead of 8 KB. Nothing that parsed before stops parsing.
- **Decimal literals are correctly rounded.** `1.7976931348623157e308` (DBL_MAX)
  now parses to DBL_MAX rather than `+Inf`, subnormals like `1e-320` are
  bit-exact, and 99.74% of a 5000-literal corpus matches the correctly-rounded
  reference (worst case 1 ulp). If you were compensating for the old parser's
  error, stop.

### 2.3.4 — one rename you may need to act on

2.3.4 renamed the bundle's `f64_round` to **`f64_round_half_away`**, because
Cyrius 6.5.x turned `f64_round` into a compiler builtin and a library may no
longer define it. This is the one source-visible change in an otherwise
compatible patch — it was unavoidable, and it is called out here because the
failure mode is silent rather than a compile error:

| You call | Before 2.3.4 | From 2.3.4 |
|----------|--------------|------------|
| `f64_round(x)` | abaco's — ties **away from zero** (`2.5 → 3`) | the 6.5.x builtin — ties to **even** (`2.5 → 2`) |
| `f64_round_half_away(x)` | — | abaco's — ties **away from zero** (`2.5 → 3`) |

Your code still compiles either way; only the tie-breaking changes. If you
depend on ties-away-from-zero (the evaluator's user-facing `round()` rule),
switch the call to `f64_round_half_away`. If you want the IEEE-754 default,
keep calling `f64_round` and you now get it from the toolchain.

Because consumers vendor the bundle rather than link it, this only takes effect
when you re-vendor — and 2.3.4 requires a re-vendor regardless (`dist/abaco.cyr`
is not byte-identical to 2.3.3).
