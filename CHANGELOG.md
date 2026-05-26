# Changelog

All notable changes to Abaco will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.2.4] — 2026-05-26

**Closeout of the 2.2.x modernization arc** (last patch before 2.3.0). A
full closeout pass — clean build, full suite, dead-code audit, downstream
consumability check, security re-scan, doc sync, version verify. Suite green at
**472 asserts, 0 failures**; clean-from-scratch DCE build passes.

### Verified

- **Downstream consumability** — built a synthetic consumer that depends on
  abaco purely via `[deps.abaco] modules = ["dist/abaco.cyr"]` + the stdlib list
  (no manual includes), exercising `Evaluator_eval`, `UnitRegistry_convert`, and
  `is_prime`. Confirms the bundle contract ([ADR 0001](docs/adr/0001-dist-bundle-distribution.md))
  is genuinely consumable end-to-end.
- **Clean build** — `rm -rf build && cyrius deps && CYRIUS_DCE=1 cyrius build`
  passes; smoke binary 246,968 B.
- **Security re-scan** — no `exec`/`fork`/`sys_system`, no hardcoded system
  paths, no large stack buffers (CI parity).

### Changed

- **Dead-code audit.** Removed the unused private helper `_nl_split` in
  `src/ai.cyr` (superseded by `_nl_split_ws`). Floor: the smoke `main()`
  exercises no library surface, so DCE NOPs the whole library (709 fns /
  ~185 KB) — this is expected for a library and not a source-cleanliness
  signal. Of the public surface, ~40 functions (`Value_*`, `ConversionResult_*`,
  `batch_*`, `CalcHistory_save/load_from_file`, …) are exercised by consumers
  rather than abaco's own tests — retained as product surface. A small set of
  semi-public-but-uncalled helpers (`mod_mul`, `reg_alias_exact`,
  `eval_has_more`) is flagged for a possible trim at the **2.3.0** boundary,
  where an API change is appropriate; kept here to avoid a breaking change in a
  patch (DCE strips them from binaries regardless).
- **Consumer docs corrected.** abaco has **no live bundle consumer today**.
  hisab is a *sibling* higher-math library (linear algebra / geometry /
  calculus) with a distinct domain — it does not depend on abaco. The intended
  consumers (the Abacus desktop app; dhvani, from which abaco's DSP was ported)
  are an ecosystem-rollout item tracked for 2.3.x. `state.md` / `CLAUDE.md`
  updated to say *planned* rather than current.

### Notes

- No functional source change beyond the `_nl_split` removal; `dist/abaco.cyr`
  differs from 2.2.3 only by that removal + the version stamp.

## [2.2.3] — 2026-05-26

Third slot of the 2.2.x arc: **P(-1) hardening + convention alignment**. A fresh
security audit under 6.0.1 closes the two deferred items from the 2026-04-14
audit. Suite green at **472 asserts, 0 failures** (was 470).

### Security

- **MED-4 — `json_parse` stack-exhaustion guard (fixed).** `lib/json.cyr`'s
  `json_parse` recurses once per nesting level with no depth cap, and abaco
  feeds it the `rates` object from a currency endpoint. A deeply-nested payload
  from a compromised / MITM'd server could exhaust the native stack before any
  value is read. Added `_ccy_json_depth_ok` — bounds nesting at
  `CCY_MAX_JSON_DEPTH = 8` before `json_parse` runs (legitimate fiat-rate maps
  are depth-1). Regression: `test_ccy_deep_nesting_rejected`. Defense-in-depth
  over the existing HTTPS guard. (+2 asserts.)
- **LOW-8 — hashmap HashDoS (documented; upstream).** `lib/hashmap.cyr` uses an
  unseeded content hash (CWE-407). abaco's residual risk is **LOW** — registry
  keys are fixed/trusted, currency keys arrive only over validated HTTPS, and
  expression variable names are consumer-supplied. No abaco-side change; a
  seeded/SipHash-class hash is recommended for the cyrius stdlib.
- Full report: [`docs/audit/2026-05-26-audit.md`](docs/audit/2026-05-26-audit.md).
  The 2026-04-14 mitigations (HIGH-1/2/3, MED-5/6/7, LOW-9a/10) were re-verified
  under 6.0.1. No new HIGH/CRITICAL findings; `src/` has no fixed-size stack
  buffers and no raw syscalls.

### Changed

- **Fuzz harnesses renamed `fuzz/fuzz_*.cyr` → `fuzz/fuzz_*.fcyr`** (the
  patra/sigil `.fcyr` extension); `fuzz/run.sh` simplified to a `*.fcyr` loop and
  CI updated to glob `fuzz/*.fcyr`.
- `src/ai.cyr` — added the MED-4 depth guard (the bundle grows accordingly).

### Added

- `docs/audit/2026-05-26-audit.md` — the P(-1) hardening audit report.
- `docs/benchmarks.md` — 3-point trend (baseline → 4.8.5-optimized → 6.0.1)
  proving no regression held across the arc; `is_prime_small` 17µs → 2µs → 1µs.
- [ADR 0004](docs/adr/0004-error-handling-defer-sakshi.md) — error handling stays
  enums-by-value; **sakshi adoption deferred on heft-vs-need** (user-ratified): a
  transitive dep every consumer must vendor, with no concrete need in abaco
  today. Recorded as an explicit, non-conforming exception with concrete
  re-examine triggers — a consumer surfacing a specific need, or sakshi being
  folded into the stdlib. Same evaluation: flat `tests/*.tcyr` layout kept.

## [2.2.2] — 2026-05-26

Second slot of the 2.2.x modernization arc: **documentation depth + light
hardening**. No source changes — `src/` is byte-identical to 2.2.1 (the bundle
differs only by version stamp); this slot adds tests, docs, and a fresh
benchmark baseline. Suite green at **470 asserts, 0 failures** (was 452).

### Added

- **`docs/architecture/`** — non-obvious invariants that can't be derived from a
  single function: token storage layout, expression depth bound (`MAX_DEPTH`),
  unit-registry two-map hashing + lookup order, Miller–Rabin witness validity
  bound. (README + 4 numbered notes.)
- **`docs/adr/`** — architecture decision records (+ template): dist-bundle
  distribution (0001), `eval_pow` vs stdlib `f64_pow` precision fast path
  (0002), deterministic Miller–Rabin over a fixed witness set (0003).
- **`docs/guides/consuming-abaco.md`** — how hisab/dhvani/Abacus wire
  `dist/abaco.cyr` as a dependency, with an API surface map.
- **LOW-9b regression tests** (closes the open 2026-04-14 audit item):
  - `test_ai::test_ccy_truncated_body` — a truncated body / lying
    Content-Length is rejected cleanly (`AI_ERR_CURRENCY`), and `_jf_get_string`
    is shown len-bounded (no over-read past the declared length). +5 asserts.
  - `test_units::test_hashmap_collisions` — a 200-entry round-trip exercises the
    cstr-keyed registry map well past its 16-slot cap (guaranteed collisions +
    grows); every key must read back. +13 asserts (with integrity check).
  - `test_units::test_registry_integrity` — per-category lookup + conversion
    integrity across the registry surface.

### Changed

- **`docs/sources.md` completeness audit** — every algorithm, formula, and
  constant cross-checked against a citation; added PolyBLEP (Välimäki &
  Huovilainen 2007), the constant-power pan/crossfade law, and the exponential
  envelope time constant.
- **`docs/README.md`** index refreshed (architecture/, adr/, guides/, sources,
  doc-health, development/); **`docs/mcp-tools.md`** read-through vs `src/ai.cyr`
  — the 5 tools mapped to their backing functions, implementation status (engine
  vs consumer-side server) clarified.
- **Benchmark baseline refreshed under 6.0.1** (`bench-latest.md` +
  `bench-history.csv`) as the early arc baseline; `is_prime_small` 2µs → 1µs.
  The full 3-point trend lands in 2.2.3.
- `docs/doc-health.md` refreshed: 30 docs, 0 stale, 0 read-through outstanding.

## [2.2.1] — 2026-05-26

Cyrius 5.7.23 → 6.0.1 upgrade and first slot of the 2.2.x modernization
arc — aligning abaco with the patra/sigil first-party reference shape.
Full suite green at **452 asserts, 0 failures** (up from 442 with SIMD
quarantined). No public API changes. Forward plan:
[`docs/development/roadmap.md`](docs/development/roadmap.md).

### Changed

- **Cyrius pin 5.7.23 → 6.0.1.** 6.0.x is internal to the toolchain
  (the `cc5→cycc` / `cyrc→cybs` rename ceremony plus two stdlib-path
  fixes) — abaco's `src/` builds clean against it with no language
  changes required.
- **`lib/` is no longer committed.** It's a build artifact regenerated
  by `cyrius deps` from `[deps].stdlib`, gitignored to match patra/sigil.
  Eliminates the "cwd ./lib/ shadows version-pinned snapshot" note and
  keeps the vendored stdlib version-matched to the pin automatically.
- **CI/release modernized to the patra/sigil shape:**
  - Toolchain installs via the upstream `scripts/install.sh`, reading the
    version straight from the `cyrius.cyml` pin (single source of truth).
  - CI now gates on `cyrius fmt --check` and `cyrius lint` (src is
    warning-clean), verifies `dist/abaco.cyr` is current, and runs the
    full `.tcyr` suite (SIMD included).
  - Release regenerates the bundle with `cyrius distlib` and ships
    `dist/abaco.cyr` alongside the source tarball + smoke binary.
  - Release tag filter narrowed to plain semver (`X.Y.Z`); the old
    `v`-prefixed style is dropped to match `git tag $(cat VERSION)`.
- Local `f64_pow` in `src/eval.cyr` renamed to **`eval_pow`** — it shadowed
  stdlib `lib/math.cyr`'s `f64_pow` (duplicate-fn warning). The evaluator
  keeps its integer-exponent fast path (exact whole powers); the rename
  just disambiguates it from the stdlib transcendental form.

### Added

- **`dist/abaco.cyr` — self-contained library bundle** generated by
  `cyrius distlib` from the new `[lib] modules` list in `cyrius.cyml`.
  This is the consumer-facing artifact: hisab, dhvani, and the Abacus
  desktop app import abaco via `[deps.abaco] modules = ["dist/abaco.cyr"]`.
- `docs/development/roadmap.md` (forward 2.2.x arc) and
  `docs/development/state.md` (volatile state snapshot), per first-party
  standards.
- `docs/sources.md` — academic/domain citations for every algorithm,
  formula, and constant (required for a math crate).

### Fixed

- **`tests/test_ntheory.tcyr` compile failure** under 6.0.1 — it compared
  `tag(r) == ERR` / `tag(r) == OK` against bare constants the stdlib
  dropped in the v5.8.x `Result` migration. `prev_prime` returns
  `Ok`/`Err`, so the asserts now use `is_ok(r)` / `is_err_result(r)` from
  `lib/result.cyr`. (107 asserts.)
- `src/core.cyr` consecutive blank lines and `src/dsp.cyr` 120-column
  overflow (cubic-interpolation coefficient split into an intermediate) —
  both cleared so the lint gate can be blocking.

### Removed

- **`tests/test_simd.tcyr` quarantine.** The `f64v_*` SIMD intrinsics
  that SIGSEGV'd under Cyrius 5.7.23 are fixed in 6.0.1 — the test passes
  (10 asserts) and `src/dsp.cyr`'s `batch_*` wrappers are sound again.
  CI no longer skips it.
- 25 committed `lib/*.cyr` stdlib files (now regenerated via `cyrius deps`).

### Migration

- Run `cyrius deps` after pulling to vendor the 6.0.1 stdlib into `lib/`.
- Consumers: `dist/abaco.cyr` is the import surface — no source-API change,
  but the bundle is new this release.

## [2.2.0] — 2026-04-28

Cyrius 4.10.2 → 5.7.23 upgrade and project modernization. No source
changes — abaco's `src/` builds clean against 5.7.23. 442 test asserts
pass; `tests/test_simd.tcyr` is quarantined (see Known issues).

### Changed

- **Cyrius bumped 4.10.2 → 5.7.23.** All `lib/` stdlib files refreshed
  via `cyrius deps`.
- **Manifest renamed `cyrius.toml` → `cyrius.cyml`** and modernized:
  - `version` now interpolates from the `VERSION` file via
    `${file:VERSION}` (single source of truth).
  - Added `repository = "https://github.com/MacCracken/abaco"`.
  - `output` is now `build/abaco` to match the standard layout.
  - `[deps].stdlib` is now multi-line for diff-friendliness.
- **CI/release workflows modernized** to match the yukti/daimon shape:
  - Toolchain version is read directly from `cyrius.cyml` — no more
    parallel `.cyrius-toolchain` pin to keep in sync.
  - New steps: `cyrius deps`, `cyrius vet`, advisory `cyrius lint`,
    DCE-enabled build, ELF verification, best-effort aarch64
    cross-build, security pattern scan, doc-presence check.
  - Release workflow accepts both `1.2.3` and `v1.2.3` tag styles,
    archives source + per-arch binaries with `SHA256SUMS`, and pulls
    the per-version body straight from `CHANGELOG.md`.

### Removed

- `.cyrius-toolchain` — superseded by reading the version from
  `cyrius.cyml` (`grep -oP '(?<=^cyrius = ")[^"]+' cyrius.cyml`).
- 40 stale `lib/*.cyr` files that were never referenced from `src/`
  (leftovers from earlier dep experiments: `audio`, `chrono`,
  `linalg`, `mabda`, `patra`, `regex`, `sakshi`, `sigil`, `thread`,
  `toml`, `ws`, `yukti`, etc.). `cyrius deps` now produces a `lib/`
  that exactly matches `[deps].stdlib`.

### Known issues

- **`tests/test_simd.tcyr` quarantined.** The `f64v_*` SIMD
  intrinsics (`f64v_add` / `f64v_sub` / `f64v_mul` / `f64v_div` /
  `f64v_sqrt` / `f64v_abs` / `f64v_fmadd`) SIGSEGV at runtime under
  Cyrius 5.7.23. This is a pre-existing compiler regression — the
  test last passed on 4.8.5; the prior CI loop only checked for
  the literal string `failed` in test output, so the crash went
  unnoticed across the 4.8.5 → 4.10.2 bump. The new CI's strict
  exit-code check exposes it. `src/dsp.cyr`'s `batch_*` thin
  wrappers (`batch_add` / `batch_sub` / `batch_mul` / `batch_div`
  / `batch_sqrt` / `batch_abs` / `batch_mac` / `batch_fmadd`) are
  affected at runtime too — DCE strips them in release builds
  today, but any caller that lands will hit the same crash. CI
  and release workflows skip `test_simd` until the intrinsics are
  fixed upstream.

## [2.1.0] — 2026-04-15

Cyrius 4.10.2 upgrade — stdlib now provides primitives that were
previously hand-rolled in abaco. Net effect: less code to maintain,
identical behaviour, and abaco gains stdlib improvements for free
(e.g. `f64_parse` handles scientific notation and NaN/Inf).

### Changed

- **Cyrius bumped 4.8.5 → 4.10.2.** All `lib/` stdlib files synced.
- **`src/dsp.cyr`** — removed 8 hand-rolled functions now in stdlib
  `math.cyr`: `f64_clamp`, `f64_max`, `f64_min`, `f64_lerp`,
  `f64_hypot`, `f64_trunc`, `f64_fract`, `f64_sign`. Call sites
  resolve to the stdlib versions transparently.
- **`src/eval.cyr`** — `CONST_PI`/`CONST_E`/`CONST_TAU` now alias
  `F64_PI`/`F64_E`/`F64_TAU` from stdlib. `gcd_int` delegates to
  stdlib `gcd()`. `lcm()` dispatch delegates to stdlib `lcm()`.
- **`src/ntheory.cyr`** — `fibonacci` and `binomial` removed; callers
  resolve to identical stdlib implementations in `math.cyr`.
- **`src/ai.cyr`** — `_nl_parse_f64` now uses stdlib `f64_parse` for
  the heavy lifting while preserving strict "entire string consumed"
  semantics (CWE-917 guard intact).

### Added (stdlib)

- **`lib/math.cyr`** gains: `f64_lerp`, `f64_hypot`, `f64_sign`,
  `f64_trunc`, `f64_fract`, `gcd`, `lcm`, `fibonacci`, `binomial`,
  `f64_parse`, `f64_parse_ok`.
- **`lib/fmt.cyr`** — `fmt_sprintf` now takes a `bufsz` parameter for
  bounds-checked formatting (breaking change in stdlib; no abaco call
  sites affected).
- **`lib/linalg.cyr`** — new stdlib module (LU, Cholesky, QR,
  determinant, inverse, least-squares). Not yet dep'd by abaco.
- **`lib/cyml.cyr`** — new stdlib module (CYML document parser).

## [2.0.0] — 2026-04-14

Major version bump: abaco is no longer a Rust crate. The entire library
has been ported to [Cyrius][cyrius] and the Rust implementation removed.
This is a breaking change for anyone who was depending on `abaco` via
`crates.io` or Cargo.

[cyrius]: https://github.com/MacCracken/cyrius

### Breaking

- **Implementation language changed from Rust to Cyrius.** `Cargo.toml`,
  `crates/*`, and all `.rs` sources are gone.
- **Distribution format changed.** abaco is now a Cyrius module set
  consumed via `[deps.abaco]` in a downstream `cyrius.toml`, not a
  crates.io dependency.
- **API shape changed.** Method-style `Evaluator::new()`/`.eval()` is now
  prefix-style `Evaluator_new()` / `Evaluator_eval(e, ...)`. See
  `docs/architecture.md` for naming conventions.
- **f64 values are bit patterns** through the public API (Cyrius convention).
- **`#[non_exhaustive]`, `Serialize`/`Deserialize`, `Display`, async
  futures** — no Rust-specific annotations apply anymore. Structured
  output goes through explicit `*_to_latex`, `*_to_json`, etc.
- **No Cargo features.** The `ai` feature is now an included module
  (`src/ai.cyr`), not feature-gated at link time. Cyrius's cross-unit
  DCE strips unused modules at build.

### Added — Cyrius port

- **`src/ai.cyr`** (520 lines) — NL parsing, `CalcHistory`,
  `CurrencyCache` with live `http_get` fetch and nested JSON extractor.
- **DSP expansions** — Hann / Hamming / Blackman / Kaiser windows,
  `window_kaiser_fill` (hoists I0(β) denom), `_bessel_i0`, `f64_cubic`
  (Catmull–Rom), `f64_sinc`, `sinc_kernel`, `freq_to_pitch_class`,
  `freq_to_octave`, `pitch_class_name`, `samples_to_ms` /
  `ms_to_samples`, `bpm_to_hz` / `hz_to_bpm`.
- **`CAT_PITCH` category** — semitone / cent / octave unit conversions.
- **BPM in `CAT_FREQUENCY`** — `registry.convert(120, "bpm", "Hz")`
  works naturally.
- **Multi-word aliases** — `"square kilometers"`, `"meters per second"`,
  `"kilometers per hour"`, `"miles per hour"`, `"miles per gallon"`.
- **`programs/basic.cyr`** — runnable end-to-end demo.
- **`fuzz/` harnesses** — `fuzz_eval`, `fuzz_ntheory`, `fuzz_units`
  with a `run.sh` runner. Clean at 50k iters each.
- **`cyrius capacity`** + `cyrius doc --check` wired into the dev loop.

### Changed

- Test count: 283 → **381 assertions** across 6 `.tcyr` files.
- Benchmarks: 56 tracked in `bench-history.csv`, last-3-runs table in
  `bench-latest.md`.
- Hyperbolic trig (`sinh`/`cosh`/`tanh`) now uses stdlib
  `lib/math.cyr::f64_sinh/cosh/tanh` instead of inlined formulas.
- Numeric constants in `src/dsp.cyr` use `_` digit separators
  (Cyrius 4.8.0): `0x4009_21FB_5444_2D18`.
- Docs rewritten for Cyrius — `README.md`, `docs/architecture.md`,
  `docs/development.md`, `CONTRIBUTING.md`, `SECURITY.md`.

### Known gaps

- **u128 is_prime perf** — Cyrius 4.8.0 `u128_mod` software long-division
  is ~40× slower than the current binary double-and-add; reverted.
  Waiting on hardware 128-bit div-mod emission.
- **`asin` / `acos` / `atan` / `atan2`** — still identity-formula stopgaps;
  filed as P1-2 in `cyrius/docs/issues/stdlib-math-recommendations-from-abaco.md`.
- **dBFS** — log-scale unit, requires special handling beyond the
  linear `to_base` factor; deferred.

## [1.1.0] - 2026-03-27

### Added

- **`ntheory` module** — number theory primitives, zero dependencies:
  - `is_prime(n)` — deterministic Miller-Rabin, correct for all u64 (Sorenson & Webster 2015 witnesses)
  - `next_prime(n)`, `prev_prime(n)` — nearest prime search
  - `factor(n)` — prime factorization via trial division, returns sorted `Vec<u64>`
  - `totient(n)` — Euler's totient function
  - `fibonacci(n)` — fast doubling algorithm, exact for n <= 93
  - `binomial(n, k)` — overflow-safe multiplicative formula
- **Evaluator functions** from ntheory: `isprime`, `nextprime`, `prevprime`, `totient`, `fibonacci`/`fib`, `binomial`/`choose`
- 8 ntheory criterion benchmarks (is_prime, factor, totient, fibonacci, binomial)
- 28 new tests (348 total + 10 doctests)
- 8 doc-tested examples in ntheory module

### Changed

- Evaluator now supports 43+ functions (was 35+)
- Roadmap updated with hisab integration plan (solver bridge, symbolic algebra, verified evaluation)

## [1.0.0] - 2026-03-27

**Abaco's first stable release.** Public API is now frozen — no breaking changes without a major version bump.

### Added

- **Implicit multiplication** — `2(3+4)`, `2pi`, `(2)(3)`, `(3)4` all work naturally
- **Factorial** — `factorial(n)` function and `n!` postfix operator (0..170)
- **GCD / LCM** — `gcd(a, b)` and `lcm(a, b)` functions
- **Statistical functions** — `mean(...)`, `avg(...)`, `median(...)`, `stddev(...)`, `stdev(...)` with variable arity
- **LaTeX output** — `Value::to_latex()` renders fractions as `\frac{n}{d}`, complex as `a + bi`, large floats in scientific notation
- **Conversion history persistence** — `CalculationHistory::to_json()`, `from_json()`, `save_to_file()`, `load_from_file()`
- **Partial parse / live evaluation** — `Evaluator::eval_partial()` for live-as-you-type feedback with error recovery
- **`Token::Bang`** variant for `!` postfix factorial
- 37 new tests (320 total + 2 doctests)

### Changed

- `lib.rs` crate docs updated to reflect full 1.0 feature set
- Expression evaluator now supports 35+ functions (was 28+)

## [0.23.0] - 2026-03-27

### Added

- **4 new unit categories** (18 total, was 14):
  - **Fuel Economy**: km/L, mpg, L/100km with reciprocal conversion support
  - **Density**: kg/m³, g/cm³, g/mL, kg/L, lb/ft³
  - **Luminosity** (Illuminance): lux, foot-candle, lm/m², phot
  - **Viscosity** (Dynamic): Pa·s, mPa·s, poise, centipoise
- **Reciprocal unit conversion** — `Unit::new_inverse()` for units where `base = factor / value` (e.g., L/100km)
- **Unit aliases and abbreviation normalization** — 80+ aliases:
  - Temperature: °C, °F, degC, degF, centigrade
  - British spellings: metres, kilometres, litres, gramme
  - Common abbreviations: kph, kmh, sec, hrs, lbs, yrs
  - Area phrases: "sq m", "sq km", "square feet"
  - Speed phrases: "meters per second", "kilometers per hour"
- **Live currency exchange rates** via hoosh service (feature-gated: `ai`)
  - `CurrencyConverter` with configurable base URL and cache TTL
  - In-memory rate caching with TTL (default: 1 hour)
  - Offline fallback: uses stale cache when service is unreachable
  - `set_rates()` for manual/test rate injection
  - Cross-rate conversion (EUR→JPY goes through base currency)
- 30 new tests (283 total, was 253), 6 new benchmarks (56 total)

### Changed

- `Unit` struct gains `to_base_inverse: bool` field for reciprocal conversions
- `UnitCategory` enum: 4 new variants (FuelEconomy, Density, Luminosity, Viscosity)
- `AiError` enum: 2 new variants (CurrencyError, HttpError)
- Registry HashMap capacities increased for 120+ units + aliases
- `serde_json` and `uuid` dependencies removed (unused)
- `chrono` moved behind `ai` feature gate (was always-on)
- Default dependency count: 3 (serde, thiserror, tracing)

### Hardened (P-1 audit, pre-0.23)

- `#[non_exhaustive]` on all 7 public enums
- `#[must_use]` on all pure functions
- `#[inline]` on hot-path functions (tokenize, eval, find_unit, convert)
- Recursion depth limit (256) in expression evaluator — prevents stack overflow
- All dependencies updated to latest compatible versions

## [0.22.4] - 2026-03-22

### Added

- `dsp` module — pure numeric DSP math primitives for audio engines
  - Decibel conversions: `amplitude_to_db`, `db_to_amplitude` (f32 and f64 variants), `db_gain_factor`
  - MIDI: `midi_to_freq`, `freq_to_midi`, constants `A4_FREQUENCY`, `A4_MIDI_NOTE`, `SEMITONES_PER_OCTAVE`
  - Envelope: `time_constant` (one-pole smoothing coefficient from ms + sample rate)
  - Waveform: `poly_blep` (anti-aliasing correction), `angular_frequency` (biquad filter design)
  - Panning: `constant_power_pan` (sin/cos law), `equal_power_crossfade`
  - Utility: `sanitize_sample` (NaN/Inf → 0.0)
- 24 tests for dsp module
- 21 DSP criterion benchmarks (scalar + batch-4096)
- ROADMAP.md

### Performance

- dB conversions use `ln`/`exp` with precomputed constants instead of `log10`/`powf` — 42-62% faster
- MIDI-to-frequency uses `exp2` instead of `powf(2.0, x)`
- Pan/crossfade use single `sin_cos()` call instead of separate `sin()` + `cos()`

### Changed

- Benchmark script outputs both CSV history and 3-point tracking Markdown table
- 50 criterion benchmarks total (was 29), 242 tests

## [0.22.3] - 2026-03-22

### Performance

- Tokenizer rewritten to byte-level iteration: 43-62% faster expression evaluation
- Unit lookup indexed with HashMaps for O(1) symbol/name resolution: 94-98% faster lookups
- Registry creation pre-allocates HashMap capacity for 100+ units
- CalculationHistory switched from Vec to VecDeque for O(1) front eviction
- Function dispatch consolidated: arity check and dispatch in single match

### Added

- IEC binary data size units: KiB, MiB, GiB, TiB, PiB (powers of 1024)
- SI decimal data sizes corrected: kB, MB, GB, TB, PB now use powers of 1000
- Cross-conversion between SI and IEC (e.g. 1 GB = 0.931 GiB)
- 29 criterion benchmarks, 218 tests (all features), 99.4% line coverage
- CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md
- codecov.yml with 90% project target
- Example: examples/basic.rs
- CI: deny, MSRV, coverage, doc, benchmark, multi-platform test jobs
- Release workflow with crates.io publish and version verification

### Changed

- License aligned to AGPL-3.0-only across Cargo.toml, LICENSE, README
- Cargo.toml: added documentation, exclude fields
- deny.toml: added version fields, Unicode-DFS-2016
- Makefile: added coverage, test-all, doc with -D warnings
- CI: 8-job pipeline (was 4), multi-platform testing
- Release: library publish workflow (was binary packaging)
- .gitignore: comprehensive (was 6 lines)

### Fixed

- Bench-history script: handles criterion's wrapped benchmark name format

## [0.1.0] - 2026-03-22

### Changed — Flatten to shared math crate

- Refactored from multi-crate workspace to single flat library crate
- Extracted GUI and binary to [abacus](https://github.com/MacCracken/abacus)
- Feature-gated AI module behind `ai` feature flag
- Added rustls-tls to reqwest
- Removed binary deps (clap, anyhow, tracing-subscriber) — library only

### Modules

- `core` — Value types (Integer, Float, Fraction, Complex, Text), Unit, UnitCategory (14 categories), Currency
- `eval` — Tokenizer, recursive descent parser, evaluator with 28+ functions, variables, scientific notation, percentage shorthand
- `units` — Unit registry with 95+ built-in units across 14 categories, conversion engine
- `ai` — Natural language math parsing, calculation history (feature-gated)

[2.1.0]: https://github.com/MacCracken/abaco/compare/2.0.0...2.1.0
[0.22.4]: https://github.com/MacCracken/abaco/compare/0.22.3...0.22.4
[0.22.3]: https://github.com/MacCracken/abaco/compare/0.1.0...0.22.3
[0.1.0]: https://github.com/MacCracken/abaco/releases/tag/0.1.0
