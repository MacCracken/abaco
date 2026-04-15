# Abaco Roadmap

## DSP Module Expansion

- [x] Window functions — Hann, Hamming, Blackman, Kaiser (2026-04-14)
- [x] Interpolation math — linear lerp, cubic, windowed sinc kernel (2026-04-14)
- [x] Chromagram helpers — `freq_to_pitch_class`, `freq_to_octave`, `pitch_class_name`, `DSP_C0_FREQ` (2026-04-14)

## Audio Unit Conversions

- [x] BPM ↔ Hz via UnitRegistry (2026-04-14) — `registry.convert(120, "bpm", "Hz")`
- [x] Semitones / cents / octaves (CAT_PITCH) via UnitRegistry (2026-04-14)
- [x] Samples ↔ milliseconds (sample-rate-aware) — `samples_to_ms`, `ms_to_samples` in dsp (2026-04-14)
- [x] dBFS (2026-04-14) — `amplitude_to_dbfs` / `dbfs_to_amplitude` in dsp.
      With a 1.0 reference amplitude (the float-audio norm) these are
      identical to `amplitude_to_db` / `db_to_amplitude`; explicit aliases
      make call sites self-documenting.

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
- [x] **MED-5** Scientific-exponent clamp at 308 in `parse_number`;
      factorial(x > 170) already returns ERR_MATH (verified).
- [x] **MED-6** Function-call arity cap at 32. Also fixed latent
      **buffer-overflow** bug: the arg buffer was `alloc(64)` (only 8 slots)
      with no bounds check — calls with > 8 args would have written past the
      buffer. Bumped to 32 slots and bounded.
- [x] **MED-7** Rate sanity check — `_ccy_rate_plausible` rejects NaN/inf,
      zero/negative, or > 10^6 rates. Empty-after-filter responses rejected
      outright as AI_ERR_CURRENCY.
- [x] **LOW-9a** Boundary regression tests — parser depth at 200 (accepted)
      and 512 (rejected via MAX_DEPTH); 33-arg call rejected; pathological
      `1e9999999999` parses finitely; strong pseudoprimes 2047, 1373653,
      25326001, 3215031751 + large primes 104729, 1000000007, 2147483647.
- [x] **LOW-10** Doc-comment safety invariants — MAX_DEPTH rationale +
      SandboxJS CVE ref; Miller–Rabin witness-set bound (3.317 × 10^24)
      with Jaeschke/Sorenson–Webster citation; `CurrencyCache_fetch`
      carries a full invariant block (HTTPS, CRLF guard, 64 KB cap,
      `base_url` trust, rate sanity).

### Still open from audit

- [ ] **MED-4** Depth-cap `json_parse` at ~64 — needs audit of
      `lib/json.cyr` recursion shape first (it's flat today, but verify).
- [ ] **LOW-8** Audit `lib/hashmap.cyr` for per-process seed / SipHash-class
      hash. Upstream concern — file against cyrius stdlib when picked up.
- [ ] **LOW-9b** Additional regression tests — truncated HTTP response with
      lying `Content-Length`, unit hashmap under synthetic collisions.

### Upstream stdlib fixes to recommend

Add to `cyrius/docs/issues/stdlib-math-recommendations-from-abaco.md`:

- [ ] **lib/http.cyr** `_http_parse_url` / `_http_build_request` should
      sanitise host + path bytes (reject `\r\n` / `< 0x20` / `0x7F`).
      We guard at the abaco boundary, but stdlib should not require
      callers to do so.
- [ ] **lib/tls.cyr** integration — abaco can't route through TLS
      without a stable API; once it has one, replace the plaintext
      `http_get` call in `CurrencyCache_fetch`.

## Cyrius 4.8.5 collapse — Completed 2026-04-14

Cyrius 4.8.5 shipped the stdlib math pack
(`cyrius/docs/issues/stdlib-math-recommendations-from-abaco.md`).
abaco collapses and measured wins:

- [x] **P1-1** `mod_mul` / `mod_pow` now thin wrappers onto `u64_mulmod` /
      `u64_powmod` (hardware-fast `mul; div` pair). **Measured: is_prime_small
      17µs → 2µs (~8.5×), is_prime_large 102µs → 4µs (~25×), next_prime
      25µs → 2µs (~12×).** Matches the 4.8.5 changelog "~12× on a full MR
      round" claim.
- [x] **P1-2** Inverse trig in `src/eval.cyr` → `f64_asin` / `f64_acos` /
      `f64_atan` / `f64_atan2` from stdlib `lib/math.cyr`. **atan2 is now
      quadrant-correct** (Q2/Q3 bug closed).
- [x] **P2-1** Inverse hyperbolic → `f64_asinh` / `f64_acosh` / `f64_atanh`.
- [x] **P2-3** Local `str_lower` / `str_upper` in `src/core.cyr` now thin
      aliases onto stdlib `str_lower_cstr` / `str_upper_cstr`.
- [x] **P2-4** DSP math constants (`DSP_TAU`, `DSP_PI`, `DSP_HALF`,
      `DSP_ONE`, etc.) now thin aliases onto stdlib `F64_TAU` / `F64_PI` /
      `F64_HALF` / `F64_ONE`. Domain-specific constants (A4, C0, semitone)
      kept locally.
- [x] **P3-2** `u64_powmod` replaces local `mod_pow` (same as P1-1).
- [x] Cyrius version pin: `4.8.3` → `4.8.5`.

### Deferred to 4.8.6
- [ ] **P2-2** `f64_parse` — deferred by cyrius per its 4.8.5 release notes.
      Our `_nl_parse_f64` stays until the stdlib variant lands with the
      full IEEE 754 grammar (scientific notation, NaN/Inf tokens).

### Bench script fix (2026-04-14)
- [x] `scripts/bench-history.sh` no longer uses `set -o pipefail` — SIGPIPE
      from `head -1` inside the CSV-parsing loop was silently killing the
      markdown-regeneration step. Switched to `set -eu`; CSV-append is
      protected independently.
