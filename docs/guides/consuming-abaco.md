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
tag = "2.4.4"                 # pin to a released tag, never a branch
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
