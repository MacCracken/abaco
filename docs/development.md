# Development

## Prerequisites

- The Cyrius toolchain installed at `$CYRIUS_HOME` (default `~/.cyrius`),
  at the version pinned in `cyrius.cyml` (`[package].cyrius` — the only place
  the pin is written down)
- `~/.cyrius/bin/cyrius` on `PATH`
- The vendored stdlib in `lib/` (gitignored) is populated via `cyrius deps`

Abaco's `cyrius.cyml` pins the toolchain version. Upgrading Cyrius? Bump the
`cyrius = "X.Y.Z"` pin, then `cyrius deps` re-vendors the version-matched
stdlib snapshot into `lib/` and `cyrius build` picks it up automatically.

## Build

```bash
# Build the library entry point.
cyrius build src/main.cyr build/abaco

# Runnable demo.
cyrius run programs/basic.cyr

# Check a single file (with the stdlib it resolves through cyrius.cyml).
cyrius check --with-deps src/eval.cyr
```

## Test

```bash
# Auto-discover and run all tests/*.tcyr
cyrius test

# Run a single test file.
cyrius test tests/test_ai.tcyr
```

Expected output: `9 passed, 0 failed` (one per suite). Each suite prints its
own `N passed, 0 failed (N total)` line; the current assertion count is in
[`development/state.md`](development/state.md).

`tests/test_ccy_tls.tcyr` (HTTPS currency fetch, `#define ABACO_TLS` at its
top) forks its own TLS servers, and raw TCP servers that stall or misbehave
mid-handshake, on ephemeral 127.0.0.1 ports — nothing leaves loopback, and
each server exits on its own after 240 s idle (it is killed at the end of a
normal run). The stall cases run in forked children under a 1.5–4 s deadline
and the silent-server case waits out the 10 s read timeout, all alongside the
rest, so the suite takes ~12 s (~3 min under qemu). Its certificates are
committed under `tests/fixtures/tls/`; `scripts/gen-tls-fixtures.sh`
regenerates them (OpenSSL ≥ 3.4). `tests/test_ccy_tls_libssl.tcyr` builds the
libssl-only configuration and checks that HTTPS is refused there.

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
# All 4 harnesses, 10k iters each (FUZZ_TIMEOUT seconds per harness, default 600).
./fuzz/run.sh

# Higher iter count (1-999999999; anything else exits 2).
./fuzz/run.sh 100000

# Single harness.
./build/fuzz_eval 50000
./build/fuzz_ntheory 50000
./build/fuzz_units 50000
./build/fuzz_ai 50000
```

## Lint

```bash
# Style warnings (line length, whitespace, untracked deferrals). Exit status
# is the warning count with --exit-with-count, which is what CI gates on.
for f in src/*.cyr; do cyrius lint --exit-with-count "$f" || echo "LINT $f"; done
```

Abaco uses `Type_method` PascalCase naming for struct-like types
(e.g. `Evaluator_eval`); the convention is project-wide and intentional.
`src/` is lint-clean, and CI fails on any warning.

## Docs

```bash
# Check doc coverage (per file).
cyrius doc --check src/core.cyr

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

Most tables are under 2% full; the function-name hash is the busiest at
roughly half its slots. We don't need the gate yet but the data is useful to
watch.

## Release

```bash
./scripts/version-bump.sh X.Y.Z     # VERSION, CHANGELOG stub, consumer tags
cyrius distlib                      # regenerate dist/abaco.cyr + dist/abaco.deps
git add -A
git commit -m "release X.Y.Z"
git tag X.Y.Z                       # plain semver, no "v", no suffix
git push origin main --tags
```

The release workflow checks VERSION == cyrius.cyml == tag, runs the CI gate,
and attaches the source tarball, `dist/abaco.cyr`, the x86_64 Linux smoke
binary and `SHA256SUMS`. There is no aarch64 artifact (no aarch64 CI lane
either; see the roadmap).

## Project structure

```
abaco/
├── cyrius.cyml           # package manifest + stdlib deps + [lib] bundle
├── VERSION               # single source of truth
├── src/                  # library modules (core, ntheory, dsp,
│                         # eval, units, ai, main)
├── tests/                # *.tcyr — auto-discovered by `cyrius test`;
│                         # fixtures/tls: HTTPS test certificates
├── benches/              # *.bcyr — run by `cyrius bench`
├── fuzz/                 # fuzz_eval / fuzz_ntheory / fuzz_units / fuzz_ai
├── programs/             # runnable demos (basic.cyr)
├── lib/                  # vendored Cyrius stdlib (gitignored;
│                         # `cyrius deps`, checked by cyrius.lock)
├── scripts/              # bench-history.sh, version-bump.sh,
│                         # gen-tls-fixtures.sh
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
