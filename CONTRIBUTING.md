# Contributing to Abaco

## Development workflow

1. Fork the repository.
2. Create a branch from `main`.
3. Make your changes. Ensure:
   - `cyrius test` — all 381+ assertions pass
   - `cyrius lint src/*.cyr` — clean (ignore `Type_method` PascalCase
     warnings; convention is intentional)
   - `./scripts/bench-history.sh` — no unexplained regressions
   - `./fuzz/run.sh` — all fuzz harnesses still pass
4. Open a pull request to `main`.

## Prerequisites

- Cyrius `4.8.x` on `PATH` (`which cyrius` should resolve)
- `~/.cyrius/lib/` populated (install via `cyrius pulsar`)

No Cargo, no Rust, no Python — the entire toolchain is Cyrius.

## Adding a new module

1. Create `src/your_module.cyr` with a header comment describing the
   module's role in one line.
2. Add `include "src/your_module.cyr"` to `src/main.cyr` (respect the
   dependency order: depend on modules already included above you).
3. Add `tests/test_your_module.tcyr` with at least one assertion per
   public fn.
4. Add benchmarks in `benches/bench.bcyr` if the module has hot paths.
5. If new stdlib deps are needed, add them to `[deps] stdlib = [...]`
   in `cyrius.toml`.
6. Update the `Modules` table in `README.md` and the module list in
   `docs/architecture.md`.

## Code style

- **Prefix naming for struct-like types** — `Evaluator_eval`,
  `UnitRegistry_find`. Match the existing conventions.
- **One-line doc comment on every publicly-intended fn** — verified
  by `cyrius doc --check`. Skip trivial accessors and `_`-prefixed
  private helpers.
- **Hex bit-patterns for f64 constants** — use `_` separators for
  readability: `0x4009_21FB_5444_2D18` (π). See `src/dsp.cyr`.
- **No task-narration comments** — `# Ported from rust-old` etc.
  don't help a future reader; drop them.
- **WHAT-comments are dead weight** — only write comments for
  non-obvious WHY (hidden constraint, workaround, subtle invariant).

## Testing

- Tests live in `tests/test_<module>.tcyr` and are discovered
  automatically by `cyrius test`.
- Every new public fn gets at least one assertion.
- Error paths should be exercised (unknown unit, parse failure,
  bounds violation).
- Fuzz harnesses in `fuzz/` are invariant tests — they guard
  "never crashes on any input" and similar properties. Extend them
  when you touch parser / lookup / conversion code.

## Benchmarking

- Add benches in `benches/bench.bcyr` (or a new `bench_<module>.bcyr`)
  for any function you expect to be on a hot path.
- Before claiming a perf win:
  1. Run `./scripts/bench-history.sh` with the old code; commit the CSV
     entry.
  2. Make your change.
  3. Run the script again; the new CSV entry is the proof.
- Don't merge a perf refactor without a measured delta.

## Commit messages

Short, imperative, lowercase is fine. Examples from the log:
- `fixing for higher cyrius`
- `all done only ai missing`
- `fixing erroneous`
- `hardening for conversion later`

Longer messages (multi-paragraph) are welcome for non-trivial changes
— prefer them over a cryptic one-liner + PR description-only detail.

## License

By contributing you agree your contributions are licensed under
GPL-3.0-only.
