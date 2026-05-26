# 004 — Miller–Rabin witness set validity bound

`ntheory::is_prime` (`src/ntheory.cyr`) is **deterministic and exact**, not
probabilistic — but only within a proven range.

- Witness set: {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37} (12 witnesses).
- Proven correct for all *n* < **3.317 × 10²⁴** (Jaeschke 1993; Sorenson &
  Webster 2015).
- The entire `i64` range is *n* < 9.22 × 10¹⁸ — comfortably inside that bound.
  So for any `i64`, the test gives the exact answer with zero error probability.

**The hard constraint**: the witness set's correctness is a *range* guarantee.
If `is_prime` (or `mod_pow` / `mod_mul`) is ever widened past `i64` — to `u64`
beyond 9.22 × 10¹⁸, to `u128`, to bignum — the current witnesses are **no longer
sufficient** and the test silently becomes probabilistic. Widening the input
type requires re-selecting (and re-citing) a witness set valid for the new
range. The rationale is recorded in
[`../adr/0003-deterministic-miller-rabin.md`](../adr/0003-deterministic-miller-rabin.md);
citations in [`../sources.md`](../sources.md).

Consequences:

- Safe to call on any `i64` today.
- Strong-pseudoprime regression tests (`test_ntheory.tcyr`) pin known composites
  (2047, 1373653, 25326001, 3215031751) that fool *subsets* of the witnesses —
  they must classify as composite.
