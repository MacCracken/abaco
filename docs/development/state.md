# Abaco — State Snapshot

> Volatile state, refreshed every release. Durable rules live in
> [`../../CLAUDE.md`](../../CLAUDE.md); forward plan in [`roadmap.md`](roadmap.md);
> per-tag history in [`../../CHANGELOG.md`](../../CHANGELOG.md).

**Last updated:** 2026-05-26 (2.2.3)

## Versions

| What | Value |
|------|-------|
| abaco | **2.2.3** |
| Cyrius toolchain pin | **6.0.1** |
| License | GPL-3.0-only |

## Artifacts

| Artifact | Size | Notes |
|----------|------|-------|
| `build/abaco` | ~247 KB | DCE smoke binary (`src/main.cyr`) — x86_64 ELF |
| `dist/abaco.cyr` | ~112 KB (~3.2k lines) | Committed consumer bundle (`cyrius distlib`) |

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

| Consumer | Domain | Status |
|----------|--------|--------|
| hisab | Expressions | uses abaco math |
| dhvani | Audio DSP | source of most ported DSP; rollout audit pending (roadmap 2.3.x) |
| Abacus | Desktop calculator app | will import `dist/abaco.cyr` as a lib |

## In flight

- 2.2.x modernization arc — see [`roadmap.md`](roadmap.md). 2.2.1–2.2.3
  released; next: 2.2.4 (closeout before 2.3.0).
