# 004 — Miller–Rabin witness set validity bound

`ntheory::is_prime` (`src/ntheory.cyr`) is **deterministic and exact**, not
probabilistic — but only within a proven range.

- Witness set: {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37} (12 witnesses).
- Proven correct for all *n* < **ψ₁₂ = 318665857834031151167461 ≈ 3.187 × 10²³**
  (Sorenson & Webster 2017; OEIS A014233), the smallest strong pseudoprime to
  all twelve bases. (Through 2.4.8 this doc said 3.317 × 10²⁴, which is ψ₁₃ and
  needs witness 41 as well.)
- The entire `i64` range is *n* < 9.22 × 10¹⁸ — comfortably inside that bound.
  So for any `i64`, the test gives the exact answer with zero error probability.

**The hard constraint**: the witness set's correctness is a *range* guarantee.
The set covers all of `u64` too (< 1.85 × 10¹⁹ ≪ ψ₁₂), but if `is_prime` is
ever widened past ψ₁₂ — to `u128` or bignum — the current witnesses are **no
longer sufficient** and the test silently becomes probabilistic. Widening the input
type requires re-selecting (and re-citing) a witness set valid for the new
range. The rationale is recorded in
[`../adr/0003-deterministic-miller-rabin.md`](../adr/0003-deterministic-miller-rabin.md);
citations in [`../sources.md`](../sources.md).

Consequences:

- Safe to call on any `i64` today.
- Strong-pseudoprime regression tests (`test_ntheory.tcyr`) pin known composites
  that fool *subsets* of the witnesses — 2047, 1373653, 25326001, 3215031751 and,
  since 2.4.9, the A014233 values up to 3825123056546413051 (a strong
  pseudoprime to every base through 31, so only witness 37 catches it) — plus
  primes above 2³², which exercise the 128-bit mulmod.
