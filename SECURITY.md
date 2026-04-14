# Security Policy

## Scope

Abaco is the AGNOS math engine (Cyrius). It has no network I/O in
the core compute modules. The `ai` module's `CurrencyCache_fetch`
uses `lib/http.cyr::http_get` to talk to a configurable hoosh
endpoint; consumers that don't call `fetch` never open a socket.

## Attack surface

| Area | Risk | Mitigation |
|------|------|------------|
| Expression parsing | Stack overflow via deep nesting | `eval_depth` bounded in the Evaluator; recursive-descent paths check before recursing |
| Expression parsing | Integer overflow in numeric literals | `parse_number` clamps intermediate accumulation, f64 range used for large values |
| Division by zero | Undefined / inf propagation | Explicit zero checks in eval + units, returns `ERR_MATH` / `UERR_CONVERT` |
| NaN / Infinity | Silent propagation | `sanitize_sample` scrubs inputs in DSP; eval detects and returns error |
| Unit lookup | Malformed query | Hashmap-based, constant work per lookup; unknown → `UERR_UNKNOWN`, never panics |
| AI currency fetch | Malicious response body | Nested JSON extractor bounds-checks every offset; malformed response → `AI_ERR_CURRENCY`, no crash (covered by `fuzz_eval` / explicit tests) |
| Natural-language parse | Adversarial input | `fuzz_eval.cyr` runs 10k+ random-byte inputs through `nl_parse` and `Evaluator_eval`; zero crashes observed |
| ntheory primality | Timing side-channel | `mod_mul` / `mod_pow` are data-independent in control flow; Miller–Rabin loop iterates a fixed witness set |

## Fuzz coverage

- `fuzz/fuzz_eval.cyr`    — random expression strings → `Evaluator_eval` + `Evaluator_eval_partial`
- `fuzz/fuzz_ntheory.cyr` — random i64 → `is_prime`, `factor`, `totient`, `next_prime`; cross-checks `is_prime` against trial division for n < 10⁶
- `fuzz/fuzz_units.cyr`   — random cstrings → `UnitRegistry_find`, `UnitRegistry_convert`

Run with `./fuzz/run.sh [iters]`. Each harness has passed 50k+
iterations with no crashes or invariant violations.

## Supported versions

| Version | Supported |
|---------|-----------|
| 2.0.x   | Yes (Cyrius port) |
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
