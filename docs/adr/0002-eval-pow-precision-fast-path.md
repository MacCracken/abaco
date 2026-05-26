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
