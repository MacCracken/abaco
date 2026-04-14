# Development

## Prerequisites

- Cyrius `4.8.x` installed at `$CYRIUS_HOME` (default `~/.cyrius`)
- `~/.cyrius/bin/cyrius` on `PATH`
- The vendored stdlib in `lib/` is refreshed via `cyrius update`

Abaco's `cyrius.toml` pins the minimum Cyrius version. Upgrading
Cyrius? `cyrius update` pulls the latest vendored stdlib modules into
`lib/` and `cyrius build` picks them up automatically.

## Build

```bash
# Build the library entry point.
cyrius build src/main.cyr build/abaco

# Runnable demo.
cyrius run programs/basic.cyr

# Syntax check a single file.
cyrius check src/eval.cyr
```

## Test

```bash
# Auto-discover and run all tests/*.tcyr
cyrius test

# Run a single test file.
cyrius test tests/test_ai.tcyr
```

Expected output: `6 files passed, 0 failed (381 assertions)` (or
higher).

## Bench

```bash
# Run a single bench file.
cyrius bench benches/bench.bcyr

# Run all benches + update CSV history + refresh bench-latest.md.
./scripts/bench-history.sh
```

The CSV history (`bench-history.csv`) is committed — it's the proof
that performance didn't regress. Before claiming a perf win, run the
history script and commit the delta.

## Fuzz

```bash
# All 3 harnesses, 10k iters each.
./fuzz/run.sh

# Higher iter count.
./fuzz/run.sh 100000

# Single harness.
./build/fuzz_eval 50000
./build/fuzz_ntheory 50000
./build/fuzz_units 50000
```

## Lint

```bash
# Style warnings (snake_case, line length, etc.)
cyrius lint src/main.cyr

# Per-file.
cyrius lint src/eval.cyr
```

Abaco uses `Type_method` PascalCase naming for struct-like types
(e.g. `Evaluator_eval`). `cyrius lint` flags these as snake_case
violations — ignore those specific warnings; the convention is
project-wide and intentional.

## Docs

```bash
# Check doc coverage (per file).
cyrius doc --check src/core.cyr

# Generate + serve HTML docs.
cyrius docs --port 8080

# Agent-oriented markdown.
cyrius docs --agent --port 8080

# Run doctest examples (# >>> / # === comments).
cyrius doctest src/ntheory.cyr
```

Every publicly-intended fn has a one-line doc comment. Trivial
accessors and `_`-prefixed private helpers are intentionally left
undocumented as a signal that they're internal.

## Capacity

```bash
# Compile-time resource usage (fn table, identifiers, code size).
cyrius capacity src/main.cyr

# CI gate — exits 1 if any table > 85% full.
cyrius capacity --check src/main.cyr
```

Current headroom: ~3–6% across all tables. We don't need the gate
yet but the data is useful to watch.

## Release

```bash
./scripts/version-bump.sh 1.2.0
git add -A
git commit -m "release 1.2.0"
git tag 1.2.0
git push origin main --tags
```

CI builds `build/abaco` for x86_64 and aarch64 Linux and attaches
them to the GitHub release.

## Project structure

```
abaco/
├── cyrius.toml           # package manifest + stdlib deps
├── VERSION               # single source of truth
├── src/                  # library modules (core, ntheory, dsp,
│                         # eval, units, ai, main)
├── tests/                # *.tcyr — auto-discovered by `cyrius test`
├── benches/              # *.bcyr — run by `cyrius bench`
├── fuzz/                 # fuzz_eval / fuzz_ntheory / fuzz_units
├── programs/             # runnable demos (basic.cyr)
├── lib/                  # vendored Cyrius stdlib (managed by
│                         # `cyrius update`)
├── scripts/              # bench-history.sh, version-bump.sh
├── docs/                 # this directory
└── build/                # gitignored — compiled artifacts
```

## Development loop (per CLAUDE.md)

1. Write feature / fix.
2. **Cleanliness check** — `cyrius lint`, `cyrius test`.
3. Add tests + benchmarks for new code.
4. Run benchmarks (`./scripts/bench-history.sh`) — numbers in CSV.
5. Audit (performance, memory, security, edge cases).
6. Cleanliness check again — must be clean after audit.
7. Deeper tests from audit observations.
8. Re-run benchmarks — prove the wins.
9. Update CHANGELOG + ROADMAP.

Never skip benchmarks. The CSV history is the proof that
performance didn't silently regress.
