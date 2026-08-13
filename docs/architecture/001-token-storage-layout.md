# 001 — Token storage layout

`src/eval.cyr` stores the tokenized expression as a flat array, not a struct
vector.

- `ABACO_MAX_TOKENS = 512` — the per-expression token cap. `tok_alloc()` allocates
  `ABACO_MAX_TOKENS * 16` bytes.
- `tokenize()` and `implicit_mul()` **enforce** the cap, returning
  `ABACO_TOK_OVERFLOW` (which both entry points map to `ABACO_ERR_PARSE`).
  `implicit_mul` needs its own check because it can insert one `*` per token,
  so its output can exceed the cap even when its input did not.

> **Corrected 2026-08-13 (2.3.4).** The cap was declared and allocated against
> but never checked: `tokenize` incremented its counter with no bound, so input
> needing more than 512 tokens wrote past the allocation. Because the bump
> allocator hands out the per-number `nend` scratch cells from the region
> immediately after the token array, the overflow interleaved with them and
> results went **silently wrong** (a 343-term sum of ones returned 342) well
> before it reached a SIGSEGV. This document previously asserted "512 tokens is
> a hard cap per expression" — true of the allocation, not of the code.
- **Each token is exactly 16 bytes**: the type tag at offset `+0`, the payload
  at offset `+8`. Accessors `tok_type(toks, i)` / `tok_val(toks, i)` /
  `tok_set(toks, i, typ, val)` compute `toks + i*16`.
- The payload is type-dependent:
  - `TOK_NUMBER` → the **f64 bit pattern** of the literal.
  - `TOK_IDENT` → a **pointer** to a NUL-terminated identifier string.
  - operators/parens → `0`.

Consequences if you touch this:

- The 16-byte stride is load-bearing. Adding a field to a token means changing
  the stride in *all four* accessors and the `ABACO_MAX_TOKENS * 16` allocation — they
  are not derived from a shared constant.
- A NUMBER payload is raw f64 bits, never an integer — read it with the f64
  accessors, not as an `i64` magnitude.
- 512 tokens is a hard cap per expression; the tokenizer does not grow past it.
