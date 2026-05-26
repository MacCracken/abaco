# 002 — Expression recursion depth bound (`MAX_DEPTH`)

The recursive-descent parser in `src/eval.cyr` tracks nesting depth in the
`Evaluator` struct (`depth` field, offset `+40`) and caps it.

- `MAX_DEPTH = 256`. Entering a parenthesized group or a function-call argument
  list increments depth; leaving decrements it.
- When `eval_depth(e) >= MAX_DEPTH`, the parser sets `ERR_PARSE` and returns
  `f64_from(0)` instead of recursing further.

**Why it exists** — this is a denial-of-service guard, not an ergonomic limit.
Deeply nested input like `(((((…)))))` or `f(f(f(…)))` would otherwise recurse
the native call stack until it exhausts and crashes. Bounding depth turns an
adversarial input into a clean `ERR_PARSE`. Same class as the SandboxJS-style
recursion-limit fixes.

Consequences:

- 256 is well above any human-written expression but low enough to stay within
  the stack. Don't raise it without re-checking stack headroom.
- The bound is checked at recursion entry, so the rejection is deterministic:
  the audit regression tests pin depth 200 as accepted and a depth-512 input as
  rejected.
