# Security Policy

## Scope

Abaco is the AGNOS math engine (Cyrius). It has no network I/O in
the core compute modules. The `ai` module's `CurrencyCache_fetch`
uses `lib/http.cyr::http_get` to talk to a configurable hoosh
endpoint; consumers that don't call `fetch` never open a socket.

## Attack surface

| Area | Risk | Mitigation |
|------|------|------------|
| Expression parsing | Stack overflow via deep nesting | `eval_depth` bounded at `ABACO_MAX_DEPTH`; **every** recursive path charges depth — parens, call arguments, unary signs and `^` chains (the last two were unbounded before 2.3.4) |
| Expression parsing | Token-array overrun | `tokenize` / `implicit_mul` bound every write against `ABACO_MAX_TOKENS`, returning `ABACO_ERR_PARSE` (unchecked before 2.3.4 — see the 2026-08-13 audit) |
| Expression parsing | Algorithmic-complexity DoS | `eval_pow` capped at `ABACO_POW_EXACT_MAX`, `totient` at `ABACO_TOTIENT_MAX`, scientific exponents at 308/400, `factorial` at 170, `fibonacci` at 92 |
| Expression parsing | Integer overflow in numeric literals | Scientific exponents are clamped; **`parse_number`'s i64 digit accumulators are not yet guarded** — ~19+ digit literals wrap (audit M-3, open) |
| Division by zero | Undefined / inf propagation | Explicit zero checks in eval + units, returns `ABACO_ERR_MATH` / `UERR_CONVERT` |
| NaN / Infinity | Silent propagation | `sanitize_sample` scrubs inputs in DSP; eval detects and returns error |
| Unit lookup | Malformed query | Hashmap-based, constant work per lookup; unknown → `UERR_UNKNOWN`, never panics |
| AI currency fetch | Malicious response body | Nested JSON extractor bounds-checks every offset; malformed response → `AI_ERR_CURRENCY`, no crash (covered by `fuzz_eval` / explicit tests) |
| Natural-language parse | Adversarial input | `fuzz_eval.fcyr` runs 20k+ inputs through `Evaluator_eval`; **`src/ai.cyr` has no harness of its own** (audit P-1, open) |
| ntheory primality | Timing side-channel | `mod_mul` / `mod_pow` are data-independent in control flow; Miller–Rabin loop iterates a fixed witness set |

## Fuzz coverage

- `fuzz/fuzz_eval.fcyr`    — random expression strings (up to 1200 bytes) → `Evaluator_eval` + `Evaluator_eval_partial`, plus targeted adversarial shapes every 4th iteration: long exponent digit-runs, deep unary-sign chains, deep `^` chains, and token-array overruns
- `fuzz/fuzz_ntheory.fcyr` — random i64 → `is_prime`, `factor`, `totient`, `next_prime`; cross-checks `is_prime` against trial division for n < 10⁶
- `fuzz/fuzz_units.fcyr`   — random cstrings → `UnitRegistry_find`, `UnitRegistry_convert`

Run with `./fuzz/run.sh [iters]`. Each harness has passed 20k+ iterations with
no crashes or invariant violations.

> The 2026-08-13 audit found the pre-2.3.4 `fuzz_eval` could not reach any of
> the four defects it reported: a 48-byte input cap left a 10× margin against
> the 512-token limit that the generator could never close, and `^` appeared
> only via a ~0.04%/byte wild-byte path. Passing iteration counts are not
> coverage. The extended harness hangs against the 2.3.3 evaluator.

## Supported versions

| Version | Supported |
|---------|-----------|
| 2.3.x   | Yes (current — Cyrius 6.5.x) |
| 2.2.x   | Security fixes only |
| 2.0.x – 2.1.x | Security fixes only |
| 1.x     | No (Rust crate, unmaintained) |

## Reporting vulnerabilities

- Email: security@agnos.dev
- Please do not open public issues for security bugs.
- 48-hour acknowledgment SLA.
- 90-day coordinated disclosure timeline.

## Design principles

- **Pure compute, headless.** No filesystem / network / process side
  effects in core compute modules. ai's HTTP fetch is opt-in.
- **Cyrius-level safety.** No raw C, no `unsafe` escape hatches — the
  language itself forbids pointer arithmetic outside what stdlib
  helpers expose.
- **Fuzz-tested.** Three harnesses guard invariants, not just
  happy-path behaviour. New parser / lookup code extends the
  harnesses as part of the PR.
- **Deterministic.** No hidden RNG, no clocks in the compute path.
  `CalcHistory` takes timestamps as caller-supplied strings.
