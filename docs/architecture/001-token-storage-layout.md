# 001 — Token storage layout

`src/eval.cyr` stores the tokenized expression as a flat array, not a struct
vector.

- `MAX_TOKENS = 512` — the per-expression token cap. `tok_alloc()` allocates
  `MAX_TOKENS * 16` bytes.
- **Each token is exactly 16 bytes**: the type tag at offset `+0`, the payload
  at offset `+8`. Accessors `tok_type(toks, i)` / `tok_val(toks, i)` /
  `tok_set(toks, i, typ, val)` compute `toks + i*16`.
- The payload is type-dependent:
  - `TOK_NUMBER` → the **f64 bit pattern** of the literal.
  - `TOK_IDENT` → a **pointer** to a NUL-terminated identifier string.
  - operators/parens → `0`.

Consequences if you touch this:

- The 16-byte stride is load-bearing. Adding a field to a token means changing
  the stride in *all four* accessors and the `MAX_TOKENS * 16` allocation — they
  are not derived from a shared constant.
- A NUMBER payload is raw f64 bits, never an integer — read it with the f64
  accessors, not as an `i64` magnitude.
- 512 tokens is a hard cap per expression; the tokenizer does not grow past it.
