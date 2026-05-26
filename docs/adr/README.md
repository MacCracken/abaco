# Architecture Decision Records

Each ADR captures *why* a decision was made — the context, the choice, and the
trade-offs that follow. Code shows *what*; ADRs show *why not the other thing*.

Create an ADR when choosing between competing approaches, adopting or rejecting
a dependency, changing the public API, or accepting a performance/correctness
trade-off. Use [`template.md`](template.md). **Never renumber** — supersede
instead.

| # | Title | Status |
|---|-------|--------|
| [0001](0001-dist-bundle-distribution.md) | Distribute as a single `dist/abaco.cyr` bundle | Accepted |
| [0002](0002-eval-pow-precision-fast-path.md) | Keep `eval_pow` separate from stdlib `f64_pow` | Accepted |
| [0003](0003-deterministic-miller-rabin.md) | Deterministic Miller–Rabin over a fixed witness set | Accepted |
| [0004](0004-error-handling-defer-sakshi.md) | Error handling: enums-by-value; defer sakshi | Accepted |
