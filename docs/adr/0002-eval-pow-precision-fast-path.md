# 0002 — Keep `eval_pow` separate from stdlib `f64_pow`

- **Status**: accepted
- **Date**: 2026-05-26
- **Version**: 2.2.1

## Context

The stdlib `lib/math.cyr` provides `f64_pow(base, exp)` computed as
`exp2(exp · log2(base))`. The expression evaluator (`src/eval.cyr`) also defined
a function named `f64_pow` with an extra **integer-exponent fast path**: for
whole exponents it uses repeated multiplication, so `2^10` returns exactly
`1024`, not the `1023.9999…` a `log2`/`exp2` round-trip can produce.

Under Cyrius 6.0.1 the two same-named functions collide — `duplicate fn
'f64_pow' (last definition wins)`. Three options: drop the local one and accept
the stdlib's rounding; keep both (warning, and ambiguous which wins); or rename.

## Decision

Rename the evaluator's function to **`eval_pow`** and keep its integer-exponent
fast path. A calculator must return `2^10 = 1024` exactly; the log/exp form is
retained only for non-integer exponents. The stdlib `f64_pow` stays available
for callers who want the transcendental form.

## Consequences

- No duplicate-fn warning; the lint gate can be blocking.
- The evaluator's `^` / `pow(...)` are exact on whole powers — the behavior a
  calculator user expects — and diverge intentionally from stdlib `f64_pow`
  there.
- Two power functions now coexist; the comment on `eval_pow` documents why, so a
  future reader doesn't "deduplicate" them back into a regression.

## Update — 2.4.9: the integer path is double-double

"Exact on whole powers" held only while every intermediate stayed exact
(`2^10`, `3^20`). Past that, plain f64 square-and-multiply doubles its relative
error at each squaring: the 2.4.9 audit measured up to 2.7 × 10⁹ ulp
(`1.0000000001^(2^40)`), `10^k` disagreeing with the literal `1ek`, and negative
powers computed as `1/x^|n|` flushing representable subnormals to 0.

`_pow_int_dd` keeps the decision — integer exponents stay on their own path,
never `exp2(y · log2 x)` — and makes the path carry a double-double product
(Dekker's two-product) with a separate binary exponent, renormalised every
step, a Newton-step reciprocal for negative exponents, and one scaling at the
end. Over 2990 cases checked against a 50-digit reference every normal result
was correctly rounded; one subnormal was 1 ulp off. Exact cases stay exact, and
the cost is a few extra multiplies per step on a path already bounded at 63
steps. See `docs/sources.md` (Integer exponents).

