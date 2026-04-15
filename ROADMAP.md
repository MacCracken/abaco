# Abaco Roadmap

## DSP Module Expansion

- [x] Window functions — Hann, Hamming, Blackman, Kaiser (2026-04-14)
- [x] Interpolation math — linear lerp, cubic, windowed sinc kernel (2026-04-14)
- [x] Chromagram helpers — `freq_to_pitch_class`, `freq_to_octave`, `pitch_class_name`, `DSP_C0_FREQ` (2026-04-14)

## Audio Unit Conversions

- [x] BPM ↔ Hz via UnitRegistry (2026-04-14) — `registry.convert(120, "bpm", "Hz")`
- [x] Semitones / cents / octaves (CAT_PITCH) via UnitRegistry (2026-04-14)
- [x] Samples ↔ milliseconds (sample-rate-aware) — `samples_to_ms`, `ms_to_samples` in dsp (2026-04-14)
- [ ] dBFS — log-scale unit, needs special handling (not linear `to_base` factor). Deferred.

## Cyrius Port — unlocks hisab

- [x] Port core module (Value struct, Unit struct, constants)
- [x] Port ntheory module (pure i64, no deps)
- [x] Port dsp module (all 23 functions, Cyrius 1.7.8 transcendentals)
- [x] Port eval module (tokenizer + recursive descent parser, 43+ functions)
- [x] Port units module (80+ units, 18 categories, hashmap registry)
- [x] SIMD batch DSP — f64v_add/sub/mul, batch_add/sub/mul/scale/mac (1us/4096 elements)
- [x] Parity audit — to_latex, eval_partial, list_units, missing units/aliases, hyperbolic trig
- [x] ai module (2026-04-14) — NL parsing, calculation history, currency cache + convert + `CurrencyCache_fetch` via `lib/http.cyr` + nested JSON extractor.

## Cyrius Port — Known Gaps (intentional or blocked)

- **f32 variants** — Cyrius is f64-only, no f32 type. All DSP uses f64. Not a gap.
- **Token enum not public** — Tokenizer is internal to eval. Consumers use Evaluator_eval.
- **Tuple returns** — pan/crossfade use output pointers instead. Cyrius has no multi-return.
- **asin/acos/atan** — Implemented via sin/cos division (stopgap). Cyrius builtins requested.
- **sinh/cosh/tanh** — Now use `lib/math.cyr` (`f64_sinh/cosh/tanh`) as of 2026-04-14.
- **u128 / is_prime perf** — mod_mul still uses the binary double-and-add method. Cyrius 4.8.0 shipped `u128` scalar + `lib/u128.cyr`, but `u128_mod` is a software long-division loop and benched ~40x *slower* than the binary method. Revisit when the backend emits hardware 128-bit div-mod.
- **256 function limit** — Tests must exclude eval to fit units tests. Cyrius raised to 1024 in v1.9+.

## Ecosystem Rollout

- [ ] Audit shruti for duplicated math that should use `abaco::dsp`
- [ ] Audit jalwa for duplicated math
- [ ] Audit tarang for duplicated math
- [ ] Audit aethersafta for duplicated math
- [ ] Audit dhvani for duplicated math (first target — was the source of most ported DSP)
- [ ] Standardize all Agnos projects on abaco for shared math

## Hardening — Completed 2026-04-14

- [x] CalcHistory JSON persistence (`to_json` / `from_json` / `save_to_file` / `load_from_file`)
- [x] Invariant-strengthened fuzzers — ntheory (factor-product + next_prime primality),
      units (round-trip identity on 10 unit pairs + same-unit identity),
      eval (whitespace idempotence + commutativity)
- [x] End-to-end integration tests — `tests/test_integration.tcyr` exercises
      `nl_parse → Evaluator → UnitRegistry → CalcHistory` flow

## Security / CVE research

Full report: [`docs/audit/2026-04-14.md`](docs/audit/2026-04-14.md).

### Applied (2026-04-14)

- [x] **HIGH-1** CRLF injection guard — `_ccy_validate_url` rejects any URL
      containing control bytes (< 0x20, 0x7F). Closes CVE-2019-9741 class
      at the abaco boundary before `lib/http.cyr` ever sees the URL.
- [x] **HIGH-2** HTTPS enforcement — `CurrencyCache_fetch` rejects non-
      `https://` URLs. Exception: `http://localhost` / `http://127.0.0.1`
      for local hoosh development.
- [x] **HIGH-3** Percent-phrase injection — `_nl_try_percent_of` now
      validates both the percentage *and* the base as strict numeric
      literals. `"2+3% of 4"` no longer synthesises `"4 * 2+3 / 100"`.
      Closes CWE-917 in the NL parser.
- [x] 5 regression tests in `test_ai.tcyr` (operator / paren injection,
      plaintext rejection, CRLF rejection, localhost acceptance).

### Open items from audit

- [ ] **MEDIUM-4** Depth-cap `json_parse` at ~64 — needs audit of
      `lib/json.cyr` recursion shape first.
- [ ] **MEDIUM-5** Reject absurd scientific-notation exponents
      (`|exp| > 308`) in `parse_number`; make `factorial(x > 170)` return
      an explicit domain error instead of `inf`.
- [ ] **MEDIUM-6** Cap function-call arity at 32 in the argument-list loop
      (defensive — prevents `alloc(huge)`).
- [ ] **MEDIUM-7** Validate rate-server response — reject NaN/inf, rates ≤ 0
      or > 10^6, missing `base`. Secondary poisoning defence.
- [ ] **LOW-8** Audit `lib/hashmap.cyr` for per-process seed / SipHash-class
      hash. File upstream if not already seeded.
- [ ] **LOW-9** Regression tests — `MAX_DEPTH ± 1` in evaluator, known
      Carmichael numbers in ntheory, truncated HTTP response with lying
      `Content-Length`, unit hashmap under synthetic collisions.
- [ ] **LOW-10** Doc-comment safety invariants — MR witness-set bound,
      HTTP 64 KB cap, `base_url` trust assumption.

### Upstream stdlib fixes to recommend

Add to `cyrius/docs/issues/stdlib-math-recommendations-from-abaco.md`:

- [ ] **lib/http.cyr** `_http_parse_url` / `_http_build_request` should
      sanitise host + path bytes (reject `\r\n` / `< 0x20` / `0x7F`).
      We guard at the abaco boundary, but stdlib should not require
      callers to do so.
- [ ] **lib/tls.cyr** integration — abaco can't route through TLS
      without a stable API; once it has one, replace the plaintext
      `http_get` call in `CurrencyCache_fetch`.

## Waiting on Cyrius 4.8.5

Once the P1/P2 stdlib items land
(`cyrius/docs/issues/stdlib-math-recommendations-from-abaco.md`):

- [ ] **P1-2** Replace inverse trig stopgaps in `src/eval.cyr:645–658`
      with `f64_asin` / `acos` / `atan` / `atan2` (fixes atan2 quadrant bug)
- [ ] **P2-1** Replace inverse hyperbolic stopgaps with stdlib `f64_asinh` / `acosh` / `atanh`
- [ ] **P2-2** Delete `_nl_parse_f64`, use stdlib `f64_parse`; unify with `parse_number`
- [ ] **P2-3** Delete local `str_lower` / `str_upper` in `src/core.cyr`, use stdlib
- [ ] **P2-4** Delete `DSP_ONE` / `DSP_HALF` / `DSP_PI` / `DSP_TAU` etc., use stdlib `F64_*` constants
- [ ] **P3-2** `u64_powmod` replaces local `mod_pow`
