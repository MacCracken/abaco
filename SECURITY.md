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
| Expression parsing | Token-array overrun | `tokenize` / `implicit_mul` bound every write against `ABACO_MAX_TOKENS` (1024 as of 2.4.0), returning `ABACO_ERR_PARSE` (unchecked before 2.3.4 — see the 2026-08-13 audit) |
| Expression parsing | Algorithmic-complexity DoS | `eval_pow` uses binary exponentiation (O(log n), no cap needed), `totient` at `ABACO_TOTIENT_MAX`, scientific exponents at 308/400, `factorial` at 170, `fibonacci` at 92 |
| Expression parsing | Integer overflow in numeric literals | `parse_number` uses an exact 18-digit mantissa plus a decimal exponent — no accumulator can wrap. Both exponents are genuinely combined before clamping as of 2.3.5 (2.3.4 still saturated the written exponent separately, preserving the bug its own comment claimed to have removed) |
| History JSON | Structure smuggled through string values | All structural scanning skips string contents via `_jf_skip_string`; keys match only in key position. Before 2.3.4 a `{`, `}` or `]` in any field silently destroyed the whole history on reload |
| Division by zero | Undefined / inf propagation | Explicit zero checks in eval + units, returns `ABACO_ERR_MATH` / `UERR_CONVERT` |
| NaN / Infinity | Silent propagation | `sanitize_sample` scrubs inputs in DSP; eval detects and returns error |
| Unit lookup | Malformed query | Hashmap-based, constant work per lookup; unknown → `UERR_UNKNOWN`, never panics |
| AI currency fetch | Malicious response body | Nested JSON extractor bounds-checks every offset; malformed response → `AI_ERR_CURRENCY`, no crash (covered by `fuzz_eval` / explicit tests) |
| Natural-language parse | Adversarial input | `fuzz_ai.fcyr` runs 20k+ inputs through `nl_parse`, `CalcHistory_*`, `_ccy_load_body` and `_ccy_validate_url`, asserting the MED-7 rate and §4.1 URL invariants directly |
| ntheory primality | Timing side-channel | `mod_mul` / `mod_pow` are data-independent in control flow; Miller–Rabin loop iterates a fixed witness set |

## Fuzz coverage

- `fuzz/fuzz_eval.fcyr`    — random expression strings (up to 1200 bytes) → `Evaluator_eval` + `Evaluator_eval_partial`, plus targeted adversarial shapes every 4th iteration: long exponent digit-runs, deep unary-sign chains, deep `^` chains, and token-array overruns
- `fuzz/fuzz_ntheory.fcyr` — random i64 → `is_prime`, `factor`, `totient`, `next_prime`; cross-checks `is_prime` against trial division for n < 10⁶
- `fuzz/fuzz_units.fcyr`   — random cstrings → `UnitRegistry_find`, `UnitRegistry_convert`
- `fuzz/fuzz_ai.fcyr`      — `nl_parse` on random bytes and grammar-shaped phrases; `CalcHistory` ring-buffer bounds and JSON round-trip; `_ccy_load_body` on adversarial rate payloads and deep nesting; `_ccy_validate_url`. Asserts that every cached rate is finite/positive/< 10⁶ and that no URL with a control byte is ever accepted

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
- **Fuzz-tested.** Four harnesses guard invariants, not just
  happy-path behaviour. New parser / lookup code extends the
  harnesses as part of the PR.
- **Deterministic.** No hidden RNG, no clocks in the compute path.
  `CalcHistory` takes timestamps as caller-supplied strings.
