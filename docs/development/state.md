# Abaco — State Snapshot

> Volatile state, refreshed every release. Durable rules live in
> [`../../CLAUDE.md`](../../CLAUDE.md); forward plan in [`roadmap.md`](roadmap.md);
> per-tag history in [`../../CHANGELOG.md`](../../CHANGELOG.md).

**Last updated:** 2026-07-17 (2.3.3 — Cyrius 6.4.66 toolchain bump; `EvalErr` enum namespaced `ERR_*` → `ABACO_ERR_*` for the new 6.4.x lint gate; dist regenerated, no longer byte-identical to 2.3.2)

## Versions

| What | Value |
|------|-------|
| abaco | **2.3.3** |
| Cyrius toolchain pin | **6.4.66** |
| License | GPL-3.0-only |

## Stdlib (6.2.x batching — unchanged in 6.3.x / 6.4.x)

The 6.2.x stdlib re-batched several modules; abaco's `[deps].stdlib` list
adjusted accordingly at 2.2.5. **6.3.x (2.3.1) and 6.4.x (2.3.3) keep the same
layout** — no re-batch, so the dependency list and source symbols are unchanged;
the newer bundles only grew internally (at 6.4.66: `bayan` +1106 lines, `math`
+246, `io` +157, `syscalls` +53, `alloc` +39, `net` +22, `fnptr` +17, `ganita`
+7 vs 6.3.10; the other 10 declared deps byte-identical). As of 2.3.0 abaco calls
the **canonical `bayan_*` API directly** — no longer the deprecated back-compat
aliases:

| Was (6.0.x) | Now (6.2.x) | Canonical symbols abaco uses |
|-------------|-------------|------------------------------|
| `json`, `u128` | `bayan` (batched bundle) | `bayan_json_{parse,key,value}`, `bayan_u64_powmod` |
| extended `math` | `ganita` (math bundle) | `f64_{a,}sinh/cosh/tanh`, `f64_asin/acos`, `f64_atan2`, `fibonacci`, `binomial` |
| `math` (slim) | `math` | basics: `f64_sin/cos/sqrt/log`, `f64_pow`, constants |

## Artifacts

| Artifact | Size | Notes |
|----------|------|-------|
| `build/abaco` | ~353 KB | DCE smoke binary (`src/main.cyr`) — x86_64 ELF (353,408 B at 2.3.3/6.4.66; was 357,736 B at 2.3.1/6.3.10). The batched `bayan`/`ganita`/`net` bundles carry more code that DCE NOPs (1134 fns, 286,827 B at 6.4.66) but leaves resident; net resident binary is marginally smaller than 6.3.10 |
| `dist/abaco.cyr` | ~112 KB (~3.17k lines) | Committed consumer bundle. 6.2.x distlib is profile-based: `cyrius distlib abaco` → `dist/abaco-abaco.cyr`, renamed to `dist/abaco.cyr`. 2.3.3: 112,217 B — carries the `ABACO_ERR_*` names; **not** byte-identical to 2.3.2 (consumers must re-vendor) |

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
- **2.3.3** (this release) tracks the **Cyrius 6.4.66 toolchain bump** — pin
  6.3.10 → 6.4.66, stdlib re-vendored. **No stdlib re-batch** (module list
  unchanged; 6.4.x bundles grew internally only). Also namespaced the `EvalErr`
  error enum `ERR_*` → `ABACO_ERR_*` to clear the new 6.4.x `cyrlint`
  error-enum-namespace advisory (leaf libs must prefix `<LIB>_ERR_*` so flat
  enum constants don't collide with the sakshi base logger / sibling libs).
  Behaviour-neutral (same integer values 0–6); `dist/abaco.cyr` regenerated with
  the new names — **no longer byte-identical to 2.3.2**. Suite green (479
  asserts); fuzz 3/3; fmt/lint/vet clean; DCE build passes; benchmarks
  flat-to-faster vs 6.3.10.

- **2.3.x still open** (external — needs consumer repos, not actionable from
  abaco alone): wire the first real consumers (Abacus, dhvani) to
  `dist/abaco.cyr`; audit consumers for duplicated math that should use
  `abaco::dsp`; `lib/tls.cyr` for the currency cache once the stdlib TLS API
  stabilizes. See [`roadmap.md`](roadmap.md).
