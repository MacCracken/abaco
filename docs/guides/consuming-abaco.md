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
tag = "2.4.10"                 # pin to a released tag, never a branch
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
| DSP | windows, `amplitude_to_db`, MIDI↔freq, interpolation, chromagram, batch ops — every argument an **f64 bit pattern** (`time_constant(f64_from(10), f64_from(48000))`, not `48000`) | `dsp` |
| Values | `Value_*`, `Unit`, `ConversionResult` | `core` |
| NL / history / currency | `nl_parse`, `CalcHistory_*`, `CurrencyCache_*` | `ai` |

## Upgrading

Bump the `tag` and run `cyrius deps`. abaco follows SemVer (post-1.0): patch and
minor bumps are source-compatible; a major bump documents breaking changes in
[`CHANGELOG.md`](../../CHANGELOG.md) with a migration section.

### 2.4.10 — one new error code, and a few more correct last bits

- **`ABACO_ERR_ARITY` (7)** is new: a known builtin with the wrong number of
  arguments (`sin(1, 2)`, `max(1)`, `mean()`). It used to be
  `ABACO_ERR_UNKNOWN_FN`. If you map error codes to messages, add one; every
  other code keeps its value.
- **Below absolute zero is refused** with `UERR_CONVERT` (−300 C, −1 K, −500 F),
  including a conversion to the same unit.
- **Results that move in the last bits**, all toward correct: `tan` (now
  ≤ 1 ulp), `factorial` / `n!` (now exact for n ≤ 170), `mean` (now correctly
  rounded, and no overflow), `stddev` (no overflow or underflow), subnormal
  and > 18-digit literals, and the window functions (now bit-exact symmetric,
  values within a few ulp of before). dhvani and jalwa call no window function
  through abaco, so nothing there moves.
- New units `short_ton` and `long_ton`; bare `ton` is still the metric tonne.

### 2.4.9 — a full audit: re-vendor, then check the rows that apply to you

2.4.9 fixes what a project-wide audit found (`docs/audit/2026-09-30-audit.md`).
Re-vendor in any case: **every `batch_*` op wrote one f64 past `dst` (and read
past `src`) when `n` was odd** — heap corruption with no error — and
`batch_scale` / `batch_mac` leaked `n * 8` bytes per call. Then:

**dhvani (and jalwa through it).**
- `time_constant` takes the sample rate as an f64: pass `f64_from(sr)`. dhvani's
  compressor and limiter pass the integer, which reads as a subnormal ~1e-319 Hz,
  so every attack and release collapsed to the one-sample `exp(-1)` = 0.368. From
  2.4.9 that mistake answers **NaN** instead of a plausible coefficient, so fix
  the call sites (`compressor.cyr`, `limiter.cyr`) before you re-vendor.
- `amplitude_to_db(NaN)` is NaN, not -Inf. A buffer with a NaN in it no longer
  reports silence from its RMS (`dynamics.cyr`).
- The dB constants are correctly rounded: `amplitude_to_db(1000)` is 60, not
  59.999999999999986. Other dB results move by an ulp or two.
- `f64_round_half_away(0.49999999999999994)` is 0 (was 1) and odd integers
  ≥ 2^52 stay put; no dhvani call site reaches either input.
- (The 2.4.8 note told dhvani to re-vendor for `DSP_C0_FREQ`; dhvani does not
  call the pitch-class functions, so that line did not apply to it.)

**Units** — results and lookups change:
- Factors are exact to the definition (1 lb = 0.45359237 kg, US gallon =
  3.785411784 L, …): US/imperial results move in the 7th significant digit,
  `cup → tbsp` is 16 and `torr` and `mmHg` differ.
- **Symbols match only as written.** `MW` / `mW`, `kN` / `kn`, `Mm` / `mm` are
  different units, and a symbol abaco does not register is `UERR_UNKNOWN` rather
  than its case-folded neighbour: `mHz`, `Mb`, `Gb`, `kb`, `b` (bits), `Cal`,
  `ML`. Lowercase `kb` / `mb` / `gb` no longer mean bytes — use `kB` / `MB` /
  `GB` or the names. A fixed list of lowercase spellings still works (`hz`,
  `khz`, `ghz`, `ml`, `l`, `kw`, `kwh`, `kpa`, `c`, `f`, `k`), as do names and
  aliases in any case (`Kilometers`, `LBS`, `MPH`).
- `st` is the stone; the `st` → semitone alias is gone (it was dead for the
  lowercase form anyway). `ms`, `ns`, `ks`, `Ws` are unknown (were metre,
  newton, kelvin, watt). `metre`, `kilometre`, `centimetre`, `millimetre` and
  `tonne` resolve.
- `L/100km` and other reciprocal units refuse a zero, negative, non-finite or
  overflowing amount with `UERR_CONVERT`. A `NULL` unit is `UERR_UNKNOWN` (was a
  SIGSEGV). `category_from_str` accepts `category_name`'s own spellings.

**Natural language** — `nl_parse` keeps the case of unit words and
expressions (keywords still match in any case), so `"convert 5 MW to kW"` is
5000. Evaluator names are case-sensitive, so `"what is SQRT(16)"` is now an
unknown function where the old lowercasing hid it. One leading verb is stripped
before every form, so `"convert 100 usd to eur"` is a currency query; one
trailing `?` or `.` is dropped; U+2212 `−` becomes `-`. A currency query is
exactly four words, and every number is read strictly and correctly rounded
(`1.5.3`, `1e` are refused).

**Evaluator.**
- A byte outside the grammar is `ABACO_ERR_PARSE` (`?`, `[`, `²`, `×`, U+2212
  in an expression): through 2.4.8 it was skipped, so `−5+3` (U+2212) was 8
  and `[2+3]*2` was 8, with no error.
- `1.2.3` is a parse error (it was 1.23). An exponent needs a digit, so `1e`
  and `2e` are 1 and 2 times the constant e (`1e` was 1). An identifier longer
  than 4096 bytes is a parse error.
- Integer functions refuse a fraction: `3.9!`, `factorial(2.5)`, `gcd(7.5, 5)`
  are `ABACO_ERR_MATH`, and `isprime(2.9)` is 0 (they truncated).
- `binomial` / `choose` give the exact value whenever it fits in i64:
  `C(62, 31)` and `C(66, 33)` are values (2.4.8 refused them with
  `ABACO_ERR_MATH`; before that ganita's `-1` came through as a result).
- More exact answers, so last bits change: integer powers are double-double
  (`10^k` equals the literal `1ek`); `log(1000)` is 3; `%` is exact for every
  finite pair (`2^60 % 10` is 6, was 0) and `5 % Inf` is 5 (was NaN).
- NaN propagates: `sign(NaN)`, `min`/`max`/`atan2` with a NaN argument and
  `median` with any NaN argument are NaN.
- An error inside parentheses or an argument keeps its code when more follows
  it: `2*(1/0+1)` is `ABACO_ERR_DIV_ZERO`, `(171!+1)` `ABACO_ERR_MATH` and
  `(x+1)*2` `ABACO_ERR_UNKNOWN_VAR` (all were `PARSE`).
- `Evaluator_eval_partial` re-parses at most a fixed few times (once, plus a
  bounded back-off out of a call still being typed: `2+max(7` previews 2) and
  returns a semantic error (division by zero, domain) as is; it used to retry
  once per token.
- **Threads:** one `Evaluator` per thread is safe (each owns its scratch
  arena); do not share one `Evaluator` between threads at the same time.
- Memory: each evaluator reuses one scratch arena, so evaluation stops growing
  the heap (16-33 KB per call before). `Evaluator_set_variable` copies the name.

**Number theory** — `factor`, `totient` and `next_prime` return for every
i64 (near 2^63 they never did); `next_prime(n)` is 0 when no i64 prime is
above n (n ≥ 2^63 − 25); `totient(n)` is 0 for n ≤ 0; `mod_pow` honours a
negative exponent as the u64 bit pattern instead of returning 1; `is_prime`
allocates nothing.

**DSP, other** — `batch_fmadd` is unfused on every target (aarch64 was
fused, so the two disagreed); `batch_sum` is compensated (Neumaier); windows
answer 0 outside `[0, size)` and 1 for `size == 1`; Kaiser windows are right
for β above ~20; `amplitude_to_dbfs` takes `|x|`; `f64_rms2` no longer
overflows above 1e154; `sinc_kernel` validates `half_width`.

**History and currency** — `CalcHistory` is a real ring buffer: O(1) push,
`CalcHistory_get` out of range is 0, a capacity ≤ 0 stores nothing. `from_json`
refuses anything but a whole, well-formed array and then leaves the history
unchanged; it decodes every JSON escape, and `to_json` escapes every control
byte. `save_to_file` is atomic, and save and load share a 16 MiB bound (a
history over 64 KiB used to save and then fail to load). The history keeps your
string pointers — keep them alive. `CurrencyCache_convert` refuses a
non-finite rate or result, or a rate that is not > 0; the loader reads `base` from the top
level only, wants a flat `rates` object and tolerates CRLF bodies;
`CurrencyCache_new` copies the URL.

### 2.4.8 — pitch classes move; two functions now report errors

- **`freq_to_pitch_class` / `freq_to_octave` answer differently for off-centre
  tones** — correctly now. The C0 reference was encoded as 16.703125 Hz rather
  than 16.3516 Hz, so every result was 0.368 semitone flat and a tone more than
  ~0.13 semitone flat of a note came back as the note below (435 Hz as G#, not
  A). In-tune notes are unchanged. If you compensated for the old skew — a
  tolerance, an offset — remove it. (This said dhvani should re-vendor for it;
  dhvani does not call these two functions — see 2.4.9.)
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
`lcm(-1e300, 1e300)` dies with SIGFPE on x86, `(-1)^(2^63)` is -1 and
`(sqrt(-1))!` is 1. 2.4.7 is correct under either convention. On aarch64, where
`f64_to` has always saturated, these were live at every pin — except that
aarch64's integer divide does not trap, so there `lcm` returned a wrong value
(-9.22e18) with no error rather than a SIGFPE.

One behaviour change to handle: `gcd`, `lcm`, `binomial` and `choose` now set
`ABACO_ERR_MATH` for a NaN, ±Inf or out-of-range argument (|x| ≥ 2^63, which
for `gcd` / `lcm` includes -(2^63)), where they used to return garbage with no
error. In-range results are unchanged. And `CurrencyCache_fetch` refuses a base
URL whose host only *starts* with `localhost` or `127.0.0.1`
(`http://localhost.example.net`) with `AI_ERR_HTTP`.

Two further changes come from the toolchain, not the bundle, so they follow
your own pin: from Cyrius 6.6.9, `sin` / `cos` are within 1 ulp for every
argument (x86 used to answer `sin(1e300) = 1e300`), and the evaluator's `tan`,
their quotient, within ~2 ulp; below 6.6.9 the pan, crossfade and sinc helpers
inherit the old large-argument error too. From 6.6.8 unary minus on zero gives
-0, so `(-0)^(-1)` is -Inf as C99 requires. Pin the toolchain in abaco's own
`cyrius.cyml` to match the one its gate runs.

### 2.4.0 — larger expressions, correctly-rounded literals

Two user-visible changes, both improvements — nothing to migrate:

- **Expressions up to 1024 tokens** parse (was 512). Costs 16 KB per evaluation
  instead of 8 KB. Nothing that parsed before stops parsing.
- **Decimal literals are correctly rounded.** `1.7976931348623157e308` (DBL_MAX)
  now parses to DBL_MAX rather than `+Inf`, subnormals like `1e-320` come out
  right, and 99.74% of a 5000-literal corpus matches the correctly-rounded
  reference (worst case 1 ulp). If you were compensating for the old parser's
  error, stop. (Corrected at 2.4.9: not every subnormal is bit-exact — a few are
  rounded twice, and digits past the 18th are dropped without a sticky bit, so
  the residual goes both ways. It stays within 1 ulp; see the roadmap.)

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
