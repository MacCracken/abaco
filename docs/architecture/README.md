# Architecture — non-obvious invariants

Numbered notes on constraints and quirks you **cannot** derive from reading a
single function — the things that bite if you change code without knowing them.
For module layout and data flow, see [`../architecture.md`](../architecture.md).
For the *why* behind design choices, see [`../adr/`](../adr/).

**Never renumber** — append new items.

| # | Invariant |
|---|-----------|
| [001](001-token-storage-layout.md) | Token storage layout (`src/eval.cyr`) |
| [002](002-expression-depth-bound.md) | Expression recursion depth bound (`MAX_DEPTH`) |
| [003](003-unit-registry-hashing.md) | Unit registry: two maps + lookup order |
| [004](004-miller-rabin-witness-bound.md) | Miller–Rabin witness set validity bound |
