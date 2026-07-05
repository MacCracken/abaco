# Abaco — State Snapshot

> Volatile state, refreshed every release. Durable rules live in
> [`../../CLAUDE.md`](../../CLAUDE.md); forward plan in [`roadmap.md`](roadmap.md);
> per-tag history in [`../../CHANGELOG.md`](../../CHANGELOG.md).

**Last updated:** 2026-07-05 (2.3.2 — dB constant fix: `DB_SCALE`/`DB_EXP`/`DB_GAIN_EXP` re-encoded; dist regenerated, no longer byte-identical to 2.3.0/2.3.1)

## Versions

| What | Value |
|------|-------|
| abaco | **2.3.2** |
| Cyrius toolchain pin | **6.3.10** |
| License | GPL-3.0-only |

## Stdlib (6.2.x batching — unchanged in 6.3.x)

The 6.2.x stdlib re-batched several modules; abaco's `[deps].stdlib` list
adjusted accordingly at 2.2.5. **6.3.x (the 2.3.1 bump) keeps the same layout** —
no re-batch, so the dependency list and source symbols are unchanged; the 6.3.x
bundles only grew internally. As of 2.3.0 abaco calls the **canonical `bayan_*`
API directly** — no longer the deprecated back-compat aliases:

| Was (6.0.x) | Now (6.2.x) | Canonical symbols abaco uses |
|-------------|-------------|------------------------------|
| `json`, `u128` | `bayan` (batched bundle) | `bayan_json_{parse,key,value}`, `bayan_u64_powmod` |
| extended `math` | `ganita` (math bundle) | `f64_{a,}sinh/cosh/tanh`, `f64_asin/acos`, `f64_atan2`, `fibonacci`, `binomial` |
| `math` (slim) | `math` | basics: `f64_sin/cos/sqrt/log`, `f64_pow`, constants |

## Artifacts

| Artifact | Size | Notes |
|----------|------|-------|
| `build/abaco` | ~358 KB | DCE smoke binary (`src/main.cyr`) — x86_64 ELF (357,736 B at 2.3.1; ~356 KB at 2.2.5/2.3.0). The batched `bayan`/`ganita`/`net` bundles carry more code that DCE NOPs (1072 fns, 271,671 B) but leaves resident |
| `dist/abaco.cyr` | ~112 KB (~3.16k lines) | Committed consumer bundle. 6.2.x distlib is profile-based: `cyrius distlib abaco` → `dist/abaco-abaco.cyr`, renamed to `dist/abaco.cyr` |

## Tests

**479 asserts, 0 failures** across 7 `.tcyr` files:

| Suite | Asserts |
|-------|---------|
| `test_ai` | 96 |
| `test_dsp` | 102 |
| `test_eval` | 103 |
| `test_integration` | 27 |
| `test_ntheory` | 107 |
| `test_simd` | 10 |
| `test_units` | 34 |

- Fuzz harnesses: 3 (`fuzz/fuzz_{eval,ntheory,units}.fcyr`) — smoke-clean
- Benchmarks: 3 (`bench`, `bench_eval`, `bench_units`)
- fmt / lint / vet: clean

## Library surface

6 modules (bundled into `dist/abaco.cyr` in this order):
`core` → `ntheory` → `dsp` → `eval` → `units` → `ai`. `src/main.cyr` is the
smoke entry, excluded from the bundle.

## Consumers

**No live bundle consumer today.** The `dist/abaco.cyr` contract is verified
consumable (2.2.4 closeout), but wiring real consumers is the 2.3.x ecosystem
rollout. Intended:

| Consumer | Domain | Status |
|----------|--------|--------|
| Abacus | Desktop calculator app | planned — will import `dist/abaco.cyr` (user-confirmed) |
| dhvani | Audio DSP | planned — abaco's DSP was ported *from* dhvani; rollout audit pending |

**Not a consumer:** hisab is a *sibling* higher-math library (linear algebra,
geometry, calculus, numerical methods) — distinct domain, no abaco dependency.

## In flight

- **2.2.x modernization arc CLOSED** (2.2.1–2.2.4); **2.2.5** tracked the
  Cyrius 6.2.11 toolchain bump + stdlib re-batch. **2.3.0** opened the
  ecosystem-rollout minor and landed the two deferred API cleanups:
  - ✅ Migrated off the deprecated `bayan` back-compat aliases to canonical
    `bayan_*` (`bayan_u64_powmod`, `bayan_json_{parse,key,value}`).
  - ✅ Trimmed the semi-public uncalled helpers `mod_mul`, `reg_alias_exact`,
    `eval_has_more` (the 2.2.4 dead-code audit flag).
- **2.3.1** tracked the **Cyrius 6.3.10 toolchain bump** — pin 6.2.11 → 6.3.10,
  stdlib re-vendored. No re-batch, no source change, no functional change;
  `dist/abaco.cyr` byte-identical to 2.3.0 bar the version header. (6.3.x stdlib
  now ships `lib/tls.cyr` + `tls_native_*` — relevant to the open currency-cache
  TLS item below, but not wired in that patch.)
- **2.3.2** (this release) fixes the **DSP dB conversion constants**. `DB_SCALE`
  (20/ln10), `DB_EXP` (ln10/20), and `DB_GAIN_EXP` (ln10/40) had been encoded
  from slightly-wrong f64 bit patterns (shared corrupted mantissa
  `…764D5B4BCDB5`); `DB_SCALE * DB_EXP` = 0.99919, so the
  `amplitude_to_db`/`db_to_amplitude` round trip drifted ~0.081% of |db| and
  blew a 0.01 dB tolerance for |db| ≥ 20. Re-encoded to the correctly-rounded
  values; added `test_db_roundtrip` (|db| up to 60). **`dist/abaco.cyr` is no
  longer byte-identical to 2.3.0/2.3.1** — consumers (dhvani) must re-vendor.
  A Cyrius-port encoding regression, not an f32→f64 issue.

- **2.3.x still open** (external — needs consumer repos, not actionable from
  abaco alone): wire the first real consumers (Abacus, dhvani) to
  `dist/abaco.cyr`; audit consumers for duplicated math that should use
  `abaco::dsp`; `lib/tls.cyr` for the currency cache once the stdlib TLS API
  stabilizes. See [`roadmap.md`](roadmap.md).
