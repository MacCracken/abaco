# Abaco — State Snapshot

> Volatile state, refreshed every release. Durable rules live in
> [`../../CLAUDE.md`](../../CLAUDE.md); forward plan in [`roadmap.md`](roadmap.md);
> per-tag history in [`../../CHANGELOG.md`](../../CHANGELOG.md).

**Last updated:** 2026-06-15 (2.2.5 — Cyrius 6.2.11 toolchain bump + stdlib re-batch)

## Versions

| What | Value |
|------|-------|
| abaco | **2.2.5** |
| Cyrius toolchain pin | **6.2.11** |
| License | GPL-3.0-only |

## Stdlib (6.2.x batching)

The 6.2.x stdlib re-batched several modules; abaco's `[deps].stdlib` list
adjusted accordingly (symbols unchanged — resolved via the bundles):

| Was (6.0.x) | Now (6.2.x) | Symbols abaco uses |
|-------------|-------------|--------------------|
| `json`, `u128` | `bayan` (batched bundle) | `json_{parse,key,value}`, `u64_mulmod/powmod` (via bayan back-compat aliases) |
| extended `math` | `ganita` (math bundle) | `f64_{a,}sinh/cosh/tanh`, `f64_asin/acos`, `f64_atan2`, `fibonacci`, `binomial` |
| `math` (slim) | `math` | basics: `f64_sin/cos/sqrt/log`, `f64_pow`, constants |

## Artifacts

| Artifact | Size | Notes |
|----------|------|-------|
| `build/abaco` | ~356 KB | DCE smoke binary (`src/main.cyr`) — x86_64 ELF. Grew from ~247 KB: the batched `bayan`/`ganita` bundles carry more code that DCE NOPs but leaves resident |
| `dist/abaco.cyr` | ~112 KB (~3.2k lines) | Committed consumer bundle. 6.2.x distlib is profile-based: `cyrius distlib abaco` → `dist/abaco-abaco.cyr`, renamed to `dist/abaco.cyr` |

## Tests

**472 asserts, 0 failures** across 7 `.tcyr` files:

| Suite | Asserts |
|-------|---------|
| `test_ai` | 96 |
| `test_dsp` | 95 |
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

- **2.2.x modernization arc CLOSED** (2.2.1–2.2.4 released) — see
  [`roadmap.md`](roadmap.md). **2.2.5** is a post-closeout maintenance patch:
  Cyrius 6.2.11 toolchain bump + stdlib re-batch (`json`/`u128` → `bayan`,
  extended math → `ganita`), no functional change. Next: **2.3.0** opens the
  ecosystem-rollout minor (wire dhvani/Abacus to the bundle; audit consumers for
  duplicated math).

- **Deferred to 2.3.0:** migrate off the deprecated `bayan` back-compat aliases
  (`json_*`, `u64_mulmod/powmod`) to the canonical `bayan_*` names. The shims
  exist only for the downstream migration window and will be removed once
  consumers re-pin — appropriate at the 2.3.0 boundary, not a patch.
