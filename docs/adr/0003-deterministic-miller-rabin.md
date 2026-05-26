# 0003 — Deterministic Miller–Rabin over a fixed witness set

- **Status**: accepted
- **Date**: 2026-05-26 (recorded; decision predates the 2.2.x arc)
- **Version**: 2.2.1 (ADR written; `is_prime` shipped earlier)

## Context

`ntheory::is_prime(n)` must classify any `i64`. The textbook Miller–Rabin test
is probabilistic with randomly chosen witnesses, which has two problems for a
library:

1. **Non-determinism** — the same input can take different code paths; results
   aren't reproducible, and a flaky witness draw can misclassify.
2. **Adversarial inputs** — random-witness MR is defeatable. Albrecht et al.,
   "Prime and Prejudice: Primality Testing Under Adversarial Conditions"
   (ACM CCS 2018), constructs composites that pass with high probability.

## Decision

Use a **fixed** witness set {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37}. This
set is proven to give a deterministic, exact answer for all
*n* < 3.317 × 10²⁴ (Jaeschke 1993; extended by Sorenson & Webster 2015). That
bound covers the entire `i64` range (*n* < 9.22 × 10¹⁸), so abaco's `is_prime`
is **exact, not probabilistic**, and immune to the adversarial class above.

## Consequences

- Reproducible and attack-resistant; no RNG dependency.
- Worst case is 12 modular-exponentiation rounds, each fast via stdlib
  `u64_powmod` (hardware `mul`/`div`).
- **Hard constraint**: the witness set is valid only up to the proven bound. Do
  not widen `is_prime` past `i64` without re-selecting witnesses for the larger
  range. See [`../architecture/004-miller-rabin-witness-bound.md`](../architecture/004-miller-rabin-witness-bound.md).
- Citations live in [`../sources.md`](../sources.md).
