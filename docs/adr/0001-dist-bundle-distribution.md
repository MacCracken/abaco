# 0001 — Distribute as a single `dist/abaco.cyr` bundle

- **Status**: accepted
- **Date**: 2026-05-26
- **Version**: 2.2.1

## Context

abaco is a library: its consumers are hisab (expressions), dhvani (audio DSP),
and the Abacus desktop app. Cyrius has no central package registry, so a
consumer pulls a dependency by git tag and names the source files to include.
The question was *which* files a consumer names.

Two options:

- **(a) Name the `src/` modules directly** — `modules = ["src/core.cyr",
  "src/ntheory.cyr", …]`. The consumer's build inlines abaco's six source files
  in dependency order. This couples every consumer to abaco's internal file
  layout: rename or split a module and every consumer's manifest breaks.
- **(b) Ship one self-contained bundle** — concatenate the modules into a single
  `dist/abaco.cyr` and have consumers name only that. abaco owns its internal
  layout; the bundle is the stable contract.

## Decision

Option (b). `cyrius.cyml` gains a library-modules list (the six library modules
in `src/main.cyr` include order, `main.cyr` excluded), and `cyrius distlib`
concatenates them into `dist/abaco.cyr`, which **is committed**. Consumers
depend via:

> **Mechanism update (2.2.5, Cyrius 6.2.x):** distlib became profile-based — the
> flat `[lib]` form was dropped for `[lib.abaco]`, and `cyrius distlib abaco`
> writes `dist/abaco-abaco.cyr` (`<pkg>-<profile>.cyr`, no override), renamed to
> the committed consumer path `dist/abaco.cyr`. The decision (one committed
> bundle as the stable contract) is unchanged.

```toml
[deps.abaco]
git = "https://github.com/MacCracken/abaco.git"
tag = "2.2.1"
modules = ["dist/abaco.cyr"]
```

The bundle is **not** stdlib-self-contained: it carries no `include "lib/…"`
lines, so the consumer supplies the stdlib surface via its own `[deps].stdlib`.
This matches sigil's bundle model and keeps abaco from dictating a consumer's
stdlib set.

## Consequences

- One stable import surface; abaco can refactor `src/` freely.
- The committed `dist/abaco.cyr` must track `src/` — CI regenerates it and fails
  on drift (`git diff --exit-code dist/abaco.cyr`).
- Consumers must list abaco's stdlib deps themselves (documented in
  [`../guides/consuming-abaco.md`](../guides/consuming-abaco.md)).
- `src/main.cyr` (the smoke `main()` + `sys_exit`) is deliberately excluded — it
  is not library surface.
