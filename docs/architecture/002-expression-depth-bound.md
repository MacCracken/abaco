# 002 — Expression recursion depth bound (`ABACO_MAX_DEPTH`)

The recursive-descent parser in `src/eval.cyr` tracks nesting depth in the
`Evaluator` struct (`depth` field, offset `+40`) and caps it.

- `ABACO_MAX_DEPTH = 256`. **Every** recursive descent charges depth: a
  parenthesized group, a function-call argument list, a unary `+`/`-` sign, and
  the right-associative `^` exponent. Leaving decrements it.
- When `eval_depth(e) >= ABACO_MAX_DEPTH`, the parser sets `ABACO_ERR_PARSE` and
  returns `f64_from(0)` instead of recursing further.

> **Corrected 2026-08-13 (2.3.4).** Through 2.3.3 only the paren and argument
> sites charged depth. `parse_unary` and `parse_power` recursed into themselves
> without touching the counter, so `----…--1` and `2^2^2^…^1` were bounded by
> nothing but the native stack and overflowed it. The 2026-04-14 audit recorded
> this bound as covering all recursion, which was not true of those two paths.
> Both now charge depth, with regression tests either side of the cap.

**Why it exists** — this is a denial-of-service guard, not an ergonomic limit.
Deeply nested input like `(((((…)))))` or `f(f(f(…)))` would otherwise recurse
the native call stack until it exhausts and crashes. Bounding depth turns an
adversarial input into a clean `ERR_PARSE`. Same class as the SandboxJS-style
recursion-limit fixes.

Consequences:

- 256 is well above any human-written expression but low enough to stay within
  the stack. Don't raise it without re-checking stack headroom.
- The bound is checked at recursion entry, so the rejection is deterministic.
  `test_unary_and_power_depth` pins 200 and 256 signs as accepted, 257 and 300
  as rejected — and asserts `eval_ntoks != 0` on each rejection, because the
  2.3.4 version of that test used inputs so long that `ABACO_MAX_TOKENS` (a
  different guard) rejected them before the parser was entered, leaving this
  bound with no coverage at all. See docs/audit/2026-08-13-fix-audit.md T-1.
- **`parse_power`'s guard is unreachable** at `ABACO_MAX_TOKENS = 512`: reaching
  depth 256 through `^` needs at least 256 operators at 2 tokens each (operator
  plus operand) = 513 tokens, and parenthesising costs 2 tokens per level too.
  A 255-operator chain is the longest that fits and parses fine at depth 255.
  The guard is retained as defence-in-depth against a future cap change.
