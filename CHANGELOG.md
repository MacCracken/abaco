# Changelog

All notable changes to Abaco will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.4.1] — 2026-08-13

**Cyrius 6.5.21 — and abaco adopts the tuples it asked for.** The proposal abaco
filed at 2.4.0 (`2026-08-13-tuple-multi-value-returns`) shipped in 6.5.21 as
declared multi-value returns with arity 2 and 3, typed element signatures, and a
destructure contract. Every error-compensated primitive here is now written as
the two- or three-valued function it actually is, rather than an out-parameter.

⛔ **Correction: that proposal's premise was partly FALSE, and the error was
abaco's.** Two-value multi-value return with destructuring had shipped at
Cyrius **v3.7.2** — four majors before the filing. The proposal's capability
table tested `return a, b;` and `var (x, y) = f();` and recorded both as parse
errors; the working forms are `return (a, b);` and `var q, r = f();`, differing
only in where the parens go. So the 2.4.0 out-parameter workarounds in this
crate were never necessary — `_two_product` could have been a real two-valued
function all along. (Upstream notes their own vidya entry documented the
paren-less form under a heading reading "NATIVE MULTI-RETURN", so the
authoritative source pointed the wrong way; that mitigates the miss but does not
undo it.) What was genuinely missing and did ship at 6.5.21: **arity 3** — the
one real capability gap, and `_dd_pow10` is the motivating case — a declared
return type making arity checkable at a forward call, and a destructure contract
(there was none: `var q, r = 42;` compiled).

⭐ **Implementing it surfaced three silent miscompiles in Cyrius's own
multi-value return** — shipped since v3.7.2 with three lines of integer-only
coverage and nothing cross-platform. One of them corrupted **precisely this
crate's shape**: a `: f64` fn returning a tuple lost its FIRST value on x86/PE,
so `two_product(3.0, 4.0)` returned 7 for both slots instead of 12 and 7. Their
`crossos/multi_return.tcyr` now covers it and names `_dd_pow10` as the
motivating arity-3 primitive.

Suite **657 asserts** (was 643); fuzz 4/4 at 20,000 iterations; fmt/lint/vet
clean; parse accuracy **unchanged** (13/5000 corpus literals at 1 ulp, identical
before and after the refactor).

### Changed

- **Toolchain pin 6.5.20 → 6.5.21.** Stdlib re-vendored; only `sandhi` changed
  upstream and abaco does not declare it, so no source impact.
- **`_two_product(a, b): (f64, f64)`** replaces `_dd_prod_err(a, b, p)` — Dekker's
  error-compensated product returns the product *and* the exact residual, which
  is what it always was.
- **`_dd_pow10(k): (f64, f64, i64)`** replaces the 3-word out-buffer. Mixed
  element types, arity 3 — the case 6.5.21 added.
- **`_dd_mul_d`, `_dd_renorm`, `_dd_split_mant`** likewise return their pairs
  directly; `_dd_renorm` in particular is now a pure function instead of a
  buffer mutator.
- **`parse_number(input, start, len): (f64, i64)`** returns the value and the
  index after it. This **removes an `alloc(8)` per numeric literal** — the
  tokenizer was allocating a scratch cell for every number, from a bump
  allocator with no free, so a 500-literal expression leaked a few hundred bytes
  of scratch per parse.
- **`tokenize` / `implicit_mul` return `(count, ok)`** instead of smuggling
  overflow back as a **negative count**. `ABACO_TOK_OVERFLOW = -1` is deleted.
  That sentinel existed only because there was no way to return "count **and**
  did-it-overflow", and a caller that skipped the check would have used −1 as a
  length — the specific hazard the proposal cited.

### Added

- **`test_two_product_exact`** — covers the shape 6.5.21 fixed, deliberately
  order-sensitive: the two slots must be distinguishable or first-value-twice
  passes unnoticed. Asserts the exact product/residual pair, a genuinely inexact
  product whose residual is sub-ulp, the arity-3 `_dd_pow10` including a case
  where the binary-exponent slot is nonzero (the slot an arity-2 return could
  not have carried), and the mantissa split past 2⁵³.

### Notes

- **Ergonomic wrinkle worth recording:** a destructured binding carries its
  declared element type, and re-assigning it from an untyped expression warns
  (`assigning non-pointer to typed pointer`). Accumulating into a fresh local
  is the workaround used throughout. Destructuring also cannot re-bind — only
  `var a, b = f();` is accepted, not `a, b = f();` — so loops that carry a pair
  across iterations destructure into fresh names and assign across.
- Parse accuracy is byte-identical to 2.4.0: the same 13 of 5000 corpus literals
  land 1 ulp low. The refactor is a shape change, not a numeric one, and was
  verified as such rather than assumed.

## [2.4.0] — 2026-08-13

**Closes both residuals the 2.3.5 fix audit left open.** Minor rather than patch
because one of them is closed by raising the expression token cap, which is
user-visible: expressions up to **1024 tokens** now parse where the limit was 512.

Suite **643 asserts** (was 631); fuzz 4/4 at 20,000 iterations; fmt/lint/vet
clean; DCE build passes; parsing benchmarks **faster** than 2.3.5 despite the
added precision.

### Changed

- **Decimal literals are now correctly rounded.** `parse_number` scales through
  an **error-compensated (double-double) factor** — Dekker's two-product carries
  the ~53 bits a single f64 drops, so the final rounding sees them. The mantissa
  is split the same way: it holds up to 18 digits, past f64's exact-integer
  limit of 2⁵³, so `f64_from(mant)` alone was already spending the last ulp.

  Measured over a 5000-literal random corpus spanning the whole exponent range:

  | | mean abs. error | top of range |
  |---|---|---|
  | 2.3.3 | ~72932 ulp | finite, 287 ulp low |
  | 2.3.5 | 1–3 ulp | **+Inf** (saturated) |
  | 2.4.0 | **99.74% bit-exact, worst 1 ulp** | bit-exact |

  `1.7976931348623157e308` — the canonical shortest round-trip decimal for
  DBL_MAX — now parses to DBL_MAX exactly. Subnormals (`1e-320`, `1e-323`,
  `5e-324`) and the exact-power boundary (`1e22`/`1e23`) are bit-exact too.

  A **Clinger fast path** keeps this free: when the mantissa is below 2⁵³ and
  |exponent| ≤ 22 both operands are exact, so one multiplication is provably
  correctly rounded and the compensated path is skipped entirely. That covers
  essentially every literal a human writes, and because the exact powers of ten
  are now built **once** into a table rather than rebuilt per literal, parsing is
  faster than 2.3.5: `sci_add` 2.31 → 2.20 µs, `sci_mul` 2.21 → 2.11 µs.

- **`ABACO_MAX_TOKENS` 512 → 1024.** Expressions up to 1024 tokens now parse.
  This also closes the second residual: at 512, reaching `ABACO_MAX_DEPTH`
  through a right-associative `^` chain needed ≥256 operators at 2 tokens each =
  ≥513 tokens, so the token cap always fired first and **`parse_power`'s depth
  guard was structurally unreachable** — retained but untestable. At 1024 the
  window exists (257 operators = 515 tokens), and
  `test_unary_and_power_depth` now pins it: 256 operators evaluate, 257 are
  rejected on depth with `eval_ntoks != 0` asserted so a token-cap rejection
  cannot masquerade as a depth one. **Verified discriminating** — reverting the
  `parse_power` guard fails 2 assertions where before it failed none.

  Costs 16 KB per `tok_alloc` (was 8 KB). Test boundaries move with it:
  512-term sums accept / 513 reject, 256 `2pi` terms accept / 257 reject.

### Fixed

- The `_pow10` accuracy residual (fix-audit P-4) — see above.
- `parse_power`'s unreachable depth guard (fix-audit T-1 structural note) — see
  above.

### Notes

- The double-double pair travels in a 3-word buffer and every error-compensated
  primitive returns through an out-parameter, because this crate believed Cyrius
  had no multi-value return. **That was wrong** — see the 2.4.1 correction: the
  2-value form had shipped at v3.7.2 and abaco tested the wrong syntax. Filed
  upstream as
  `cyrius/docs/development/proposals/2026-08-13-tuple-multi-value-returns.md`;
  arity 3 (which `_dd_pow10` needs) was the one genuine gap.
- 13 of 5000 corpus literals remain 1 ulp low. Reaching 100% correctly-rounded
  needs a bignum fallback (Clinger/Eisel-Lemire); not pursued — the remaining
  error is at the rounding boundary and bounded at 1 ulp.

## [2.3.5] — 2026-08-13

**A second audit, run against 2.3.4's fixes rather than the bugs they replaced.**
It found that two of those fixes introduced **new silent wrong answers**, that
the H-4 JSON fix was incomplete, and — the part worth dwelling on — that six of
2.3.4's regression assertions were **decorative**: green whether or not the code
they guarded was present. Full report:
[`docs/audit/2026-08-13-fix-audit.md`](docs/audit/2026-08-13-fix-audit.md)
(74 findings raised, 39 confirmed).

Suite **631 asserts** (was 547); fuzz 4/4 at 20,000 iterations; fmt/lint/vet
clean; DCE build passes; benchmarks flat.

### Fixed — regressions introduced by 2.3.4

- **`(±Inf)^n` returned NaN for integer n > 1024.** The `ABACO_POW_EXACT_MAX`
  cap sent every exponent above it to `exp2(exp · log2|base|)`, and
  `exp2(+Inf)` is NaN. 2.3.3 returned ±Inf, the IEEE-754 answer. The boundary
  sat exactly at the cap; `eval_err` stayed `ABACO_ERR_NONE`, so it was silent.
  `test_pow_exponent_cap` used only finite bases, which is why it passed.
- **`0e400` parsed as NaN.** An all-zero significand made the scale multiply
  `0.0 * +Inf`. Affected `0e309`..`0e400`, `0.0e400`, `.e400`,
  `0e9999999999`; a differential fuzz found 70 NaNs under 2.3.4 and none under
  2.3.3. `parse_number` now short-circuits a zero mantissa.

- **`eval_pow` rewritten to binary exponentiation**, replacing the cap
  entirely. Square-and-multiply is O(log|exp|) — at most 63 multiplications for
  any i64 exponent — so it removes the DoS *without* a cap, propagates Inf and 0
  as IEEE requires, and keeps the exact path across the whole range. The cap's
  "behaviour-preserving" claim was measurably false: `0.5^1024` is 2⁻¹⁰²⁴ (a
  normal value, zero only at 1075), the band `0.5004 < |base| < 1.998` is still
  changing past 1024, and handing that tail to exp2/log2 cost up to ~493 ulps in
  a function whose stated purpose is exactness.

### Fixed — parse_number

- **The two exponents were still clamped separately.** The 2.3.4 comment
  promised "combine, THEN clamp", but the written exponent was saturated at 400
  per digit *before* the combination, so the cancellation bug it described
  survived verbatim. Per-digit saturation is now `ABACO_DEC_EXP_MAX` (10⁹), far
  above the f64 range, leaving `total = dec_exp + sci` as the only clamp.
- **The lower clamp broke genuine subnormals.** `ABACO_POW10_MAX = 340` was
  justified as "nothing representable survives past this" — true only for
  `mant == 1`. With a mantissa up to ~10¹⁸ the product lands near 10⁻³²², inside
  the subnormal range; 296 of 710 probed literals were wrong, 293 of them
  returning nonzero where the answer is exactly 0.0. The bound now carries
  `ABACO_MANT_DIGITS` of headroom.
- **`_pow10` accuracy** — the scale factor is now built through the exact 10²²
  block, cutting roundings from k to ceil(k/22) and holding the parse to 1–3 ulp
  across the range. **Known residual:** literals within ~2 ulp of DBL_MAX still
  saturate to +Inf, because 1 ulp of upward error at the top of the range *is*
  +Inf. Closing it needs a double-double scale; pinned by
  `test_top_of_range_literals` and tracked in the roadmap. For proportion: mean
  absolute error over a 1510-literal corpus is 1.6 ulp here versus 72932 at 2.3.3.

### Fixed — src/ai.cyr (the H-4 fix was incomplete)

- **`_jf_get_object` was never made string-aware.** The 2.3.4 fix rewrote the
  other scans and its comment claimed "every structural scan in this module goes
  through this" — but this one still counted raw braces. A `}` inside a rates
  **key** truncated the object, and since `_ccy_load_body`'s only completeness
  check is `kept == 0`, a truncated object that still yielded one valid pair was
  reported as **success** with a silently incomplete rate table.
- **`_ccy_json_depth_ok` could FALSE ACCEPT.** It counted braces and brackets
  with no floor, so unbalanced closers drove the counter negative and masked
  genuine nesting: N `]` bytes bought N free levels. That is the one direction a
  DoS guard must never fail in, and it contradicted the function's own "only
  ever rejects more aggressively" comment. Fixed with a clamp at zero.
- **1-byte out-of-bounds write in `CalcHistory_load_from_file`.** It allocated
  65536, read up to 65536, then wrote the NUL at `buf + n` — one past the end for
  any file at or above the cap, landing exactly where the header-less bump
  allocator would hand out next, and on unmapped memory when the block ended on a
  chunk boundary. Now allocates `cap + 1` and rejects an over-cap file rather
  than silently truncating it.

### Fixed — tests and fuzz harnesses that did not test

- **`test_unary_and_power_depth` was decorative.** Its inputs (5000 minuses,
  a 300-operator `^` chain) both exceed `ABACO_MAX_TOKENS`, so the tokenizer
  rejected them before the parser ran — the assertions passed on H-1's token cap,
  not H-3's depth accounting, and stayed green with the depth fix reverted.
  Rewritten to sit inside the token budget (256 accepted, 257 and 300 rejected),
  with `eval_ntoks != 0` asserted on every rejection so a token-cap rejection
  cannot masquerade as a depth-cap one. Also records that `parse_power`'s guard
  is **unreachable** through `^` chains at the current cap (256 operators need
  513 tokens) and is retained as defence-in-depth.
- **`test_history_json_key_confusion` could not fail** — its payload escaped the
  lookalike, so the pre-fix matcher never matched it either. Replaced with an
  unescaped lookalike; the 2.3.3 parser returns `HIJACK`, this one returns `REAL`.
- **`fuzz_eval` did not detect H-1**: it crossed the token boundary but discarded
  every result, so with all bounds removed it passed 50,000 iterations. Each
  pathological shape now asserts its contract — the sum shape checks the value is
  exactly n, which is what the out-of-bounds write corrupted.
- **`fuzz_ai`'s CRLF invariant was vacuous.** The assertion required a URL to be
  both control-byte-bearing and *accepted*, but the generated URLs were random
  bytes that the scheme check always rejects, so it could never fire. URLs are
  now built from a real accepted prefix plus fuzz bytes, with a mirrored
  assertion so the guard cannot be satisfied by rejecting everything.
- **The MED-7 rate probe inspected 1 key in 26** — it hard-coded the third letter
  of its probe keys while the generator randomised all three (194 of 4028
  accepted rates actually checked). It now walks exactly the keys planted.
- **The deep-nesting block asserted nothing.** It cannot be made discriminating
  end-to-end (a nested payload is rejected by `kept == 0` regardless), so the
  guard's contract is asserted directly, alongside a probe for the `]`-padding
  false accept.
- **No harness could reach the `totient` domain cap** — the generators emit no
  function-call syntax. A fifth pathological shape composes calls from a fixed
  vocabulary of function names.
- **`implicit_mul`'s own overflow bound had no test** (every `test_token_cap`
  input is a plain `1+1+…`); `2pi` terms are the only shape that reaches it —
  128 fit, 129 do not.
- **No test evaluated the expression `round(…)`**, so the evaluator binding the
  2.3.4 rename exists to protect could have been switched back to the
  ties-to-even builtin with the suite green.

### Fixed — other

- **Token cap rejected a valid expression followed by whitespace.** The check ran
  before the byte was classified, so a pass that emitted no token still counted.
- **Docs**: the 2.3.4 CHANGELOG claimed the README fix wrote "486 asserts" when
  it wrote 547; `SECURITY.md` said "Three harnesses" two sections after listing
  four; `docs/development.md` carried stale expected output; the quoted DCE
  statistics did not reproduce; `docs/architecture/002` repeated the mistaken
  claim about what the depth tests pin.

## [2.3.4] — 2026-08-13

**Cyrius 6.5.20 toolchain bump — first bump that is _not_ source-neutral — plus
six pre-existing defects fixed.** The 6.5.x toolchain broke the build in two ways:

1. **`f64_round` became a reserved builtin.** abaco had hand-rolled
   `fn f64_round` in `src/dsp.cyr` since the port, and 6.5.x rejects that
   outright. Investigating it turned up something worse than a rename: the
   compiler had been *silently shadowing* that function since at least 6.2.11,
   so the shipped `round()` was ties-to-**even** while the source, tests and
   citations all described ties-**away-from-zero**. See below — **`round()`
   changes for exact-`.5` inputs in this release.**
2. **Call arity is now enforced.** A bare `print(cstr)` compiled at 6.4.66 even
   though `lib/io.cyr` declares `print(msg, len)`; 6.5.20 rejects it. All three
   fuzz harnesses used the 1-arg form.

Auditing the bump then surfaced six pre-existing defects, none related to the
toolchain. In `src/eval.cyr`: an **out-of-bounds heap write** in the tokenizer
that produced silently wrong answers, an **unbounded exponent loop** (~81 years
of work from 20 bytes of input), **unbounded parser recursion** past
`ABACO_MAX_DEPTH`, an **uncapped `totient()` domain**, and **i64 accumulators
that wrapped** on long numeric literals. In `src/ai.cyr`, a new `fuzz_ai`
harness immediately found that **`CalcHistory_from_json` silently discarded the
entire history** if any field contained a `{` or `}`. All are fixed here with
regression tests — see *Fixed* and
[`docs/audit/2026-08-13-audit.md`](docs/audit/2026-08-13-audit.md).

**No stdlib re-batch** — the `[deps].stdlib` module list is unchanged and all 18
declared modules still exist at 6.5.20. Suite green at **547 asserts, 0
failures** (was 479); fuzz **4/4** at 20,000 iterations each; fmt/lint/vet clean
(per-file — see below); DCE build passes.

### Changed

- **Toolchain pin 6.4.66 → 6.5.20** (`cyrius.cyml [package].cyrius`).
- **`f64_round` → `f64_round_half_away`** (`src/dsp.cyr`), which **changes
  `round()` results for exact-`.5` inputs.** Call sites updated in
  `src/eval.cyr` (the evaluator's `round`), `src/dsp.cyr`
  (`_freq_semis_from_c0`), `tests/test_dsp.tcyr`, and `benches/bench.bcyr`.

  | input | 2.3.3 shipped | 2.3.4 |
  |-------|---------------|-------|
  | `round(0.5)` | 0 | **1** |
  | `round(2.5)` | 2 | **3** |
  | `round(-2.5)` | −2 | **−3** |
  | `round(3.5)` | 4 | 4 |

  The reason is worth recording, because it is not what it looks like. abaco's
  hand-rolled `f64_round` was **dead code**: `f64_round` has been a compiler
  intrinsic since at least 6.2.11, and the intrinsic shadowed the local
  definition without a diagnostic. Verified by building the 2.3.3 tree against
  the 6.2.11, 6.3.10 and 6.4.66 toolchains — all three answer `round(2.5) = 2`,
  i.e. ties-to-even, while the function 20 lines away in `src/dsp.cyr` said
  ties-away-from-zero and `docs/sources.md` cited it as such. So for four
  releases the documented behaviour and the shipped behaviour disagreed, and
  nothing could have caught it: the existing tests only checked `3.7`, `3.2` and
  `-3.7`, which both rounding modes answer identically.

  6.5.x promoting `f64_round` to a reserved word forced the name apart and made
  the divergence visible. Renaming restores the documented, tested, cited
  behaviour — ties away from zero, as the evaluator's `round()` was always
  specified to do and as a desk calculator is expected to behave.
  `test_round_ties_away` now pins **both** modes so they can never silently
  swap again.

  Two consequences worth noting. First, abaco no longer *supplies* `f64_round`
  to the vendored stdlib — at 6.4.66, `lib/math.cyr` called `f64_round` without
  defining it and resolved to abaco's copy (which the intrinsic had already
  displaced); at 6.5.20 it resolves to the intrinsic explicitly. Second, the
  `round_4096` benchmark went 7.86 µs → 26.7 µs. That is not a regression: the
  old figure timed a single `roundsd` instruction because the intrinsic had
  replaced the call, and the new figure is the first honest measurement of the
  hand-rolled function actually running.
- **Fuzz harnesses fixed for arity enforcement** — added a local `_fz_puts(s)`
  helper (`print(s, strlen(s))`) to `fuzz/fuzz_{eval,ntheory,units}.fcyr` and
  routed the 1-arg `print(...)` diagnostics through it. Diagnostic-only change;
  no invariant altered.
- **Deferral comments cross-referenced** — `cyrlint` has an untracked-deferral
  gate: a comment carrying a deferral marker (`TODO`, `FIXME`, `follow-up`,
  `deferred`, `for now`, `not yet`, `out of scope`, …) must carry a `CHANGELOG` /
  `roadmap` / `docs/` / `issue` / `See ` reference **on the same line**, or
  `#skip-lint`. The two hoosh live-fetch follow-ups in `src/ai.cyr` now point at
  `docs/development/roadmap.md`; the roadmap gained a matching "Still open"
  entry so the reference is real.
  **This gate is not new in 6.5.x** — verified by running the 6.3.10 and 6.4.66
  `cyrlint` binaries against the 2.3.3 tree, where it reports the same two
  deferrals. 2.3.1 and 2.3.3 both claimed "lint clean" because nothing was
  looking: CI matched only `^\s*warn ` lines (fixed below), and the advisory
  prints under a separate `deferral line N:` heading above a `0 warnings`
  summary that reads as success.
- **Stdlib re-vendored at 6.5.20** (`cyrius deps`). Module list unchanged; the
  bundles grew internally (`bayan` +774 lines, `syscalls_x86_64_agnos` +442,
  `alloc` +241, `io` +203, `vec` +187, `bench` +185, `syscalls_aarch64_linux`
  +138, `syscalls_linux_common` +92, `fmt` +61, `syscalls_macos` +65,
  `syscalls` +51, `syscalls_windows` +43, `atomic` +21, `ganita` +20,
  `syscalls_x86_64_linux` +18, `math` +8, `assert` +6, `string` +4;
  `alloc_*`, `args_*`, `hashmap`, `http`, `net`, `str` byte-identical).
- **Smoke binary `build/abaco`** ≈ 395,712 B — up ~42 KB from 353,408 B at
  2.3.3/6.4.66 (1203 unreachable fns, 312,537 B NOPed by DCE).
- **`dist/abaco.cyr` regenerated** — 123,434 B / 3,380 lines (was 112,517 B /
  3,167). Carries the `f64_round_half_away` rename, the evaluator bound checks
  and the JSON string-aware scanning, so it is **not**
  byte-identical to 2.3.3 and consumers must re-vendor. Any consumer that was
  calling the bundle's `f64_round` now silently gets the ties-to-even builtin
  instead — call `f64_round_half_away` to keep the old behaviour.

### Added

- **`test_round_ties_away`** (`tests/test_dsp.tcyr`) — pins the tie behaviour of
  both `f64_round_half_away` (0.5→1, 1.5→2, 2.5→3, −0.5→−1, −2.5→−3) *and* the
  6.5.x builtin `f64_round` (0.5→0, 2.5→2), so a future toolchain bump cannot
  silently swap one rounding mode for the other — the pre-existing tests used
  only `3.7`/`3.2`/`-3.7`, which both modes answer identically, which is exactly
  why the four-release divergence went unnoticed.
- **`test_token_cap`, `test_pow_exponent_cap`, `test_unary_and_power_depth`,
  `test_totient_cap`** (`tests/test_eval.tcyr`) — regression coverage for the
  four defects in *Fixed* below, each pinned either side of its boundary.
- **`test_long_literals`** (`tests/test_eval.tcyr`) — 19/20/21-digit integers
  stay positive and keep their magnitude, 19- and 65-digit fractions stay 1.0,
  leading zeros do not consume mantissa room, `1e-320` stays a nonzero
  subnormal, and whole numbers stay bit-exact.
- **`test_history_json_braces`, `test_history_json_key_confusion`**
  (`tests/test_ai.tcyr`) — brace/bracket payloads round-trip, and a genuine key
  wins over a lookalike inside a value.
- **`fuzz/fuzz_ai.fcyr`** — the crate's fourth fuzz harness, covering the
  previously unfuzzed `src/ai.cyr`: `nl_parse` on both random bytes and
  grammar-shaped phrases, `CalcHistory` ring-buffer bounds and JSON round-trip,
  `_ccy_load_body` on adversarial rate payloads and deep nesting, and
  `_ccy_validate_url`. It asserts the audit's stated safety invariants directly
  — every rate surviving the loader is finite/positive/< 10⁶ (MED-7), and no URL
  containing a control byte is ever accepted (CRLF injection, §4.1). Confirmed
  to discriminate: it fails on the 2.3.3 `src/ai.cyr` and passes 50,000
  iterations against this one.

  Suite total 479 → **547 asserts**.

### Fixed

Six defects found by the 2026-08-13 audit
([`docs/audit/2026-08-13-audit.md`](docs/audit/2026-08-13-audit.md)) — five in
`src/eval.cyr`, one in `src/ai.cyr`. All predate this release and are unrelated
to the toolchain bump; all were reproduced before being fixed and have
regression tests.

- **`tokenize()` wrote past the end of the token array — heap corruption.**
  `ABACO_MAX_TOKENS` (512) was declared and allocated against, but the token
  counter was never checked, so any input needing more than 512 tokens wrote
  outside `tok_alloc()`'s 8192 bytes. The bump allocator hands out the
  per-number `nend` scratch cells from the region immediately after the token
  array, so the overflow interleaved with them and **produced silently wrong
  answers long before it crashed** — a 343-term sum of ones returned 342, and
  every larger input also returned 342, with no error set. `TOK_IDENT` payloads
  are pointers, so a corrupted one was passed to `streq()` as a wild read.
  Around 16M tokens it SIGSEGVs. `tokenize` and `implicit_mul` now both bound
  every write and return `ABACO_TOK_OVERFLOW`, which `Evaluator_eval` and
  `Evaluator_eval_partial` surface as `ABACO_ERR_PARSE`. `implicit_mul` needs
  its own check: it inserts up to one `*` per token, so its output can exceed
  the cap even when its input did not.
- **`eval_pow` had an unbounded O(exponent) loop — DoS.** The integer-exponent
  fast path multiplied `exp` times with the count taken straight from user text
  and no cap, so `2^1000000000000000000` (20 bytes) was ~10¹⁸ multiplications,
  about **81 years** at the measured 2.57 ns/iteration. `2**N` and `pow(2,N)`
  reach the same path. Now capped at `ABACO_POW_EXACT_MAX` (1024), above which
  the existing O(1) `exp2`/`log2` path returns the identical value — f64 has
  already saturated to ±∞ or 0 by 2¹⁰²⁴, so every skipped iteration was
  provably a no-op. Same class of fix, and the same argument, as the Cyrius
  stdlib's own 6.4.69 clamp in `lib/math.cyr`. The three pathological inputs now
  return in **2 ms** total.
- **`ABACO_MAX_DEPTH` did not bound unary or power recursion — stack overflow.**
  Only the paren and function-argument sites charged depth. `parse_unary` and
  `parse_power` recursed into themselves without touching the counter, so
  `----…--1` and `2^2^2^…^1` were limited by nothing but the native stack.
  Both now charge depth and early-out with `ABACO_ERR_PARSE`, matching
  `parse_expr`. `docs/architecture/002-expression-depth-bound.md` claimed the
  bound already covered all recursion; corrected there too.
- **`totient(n)` had no domain cap.** The evaluator exposed it directly over an
  O(√n) trial-division loop while `factorial` (170) and `fibonacci` (92) were
  both bounded. Capped at `ABACO_TOTIENT_MAX` (10¹²), keeping the loop under
  ~10⁶ iterations.
- **`parse_number`'s i64 accumulators wrapped on long literals.** `int_part`,
  `frac_part` and `frac_div` were each multiplied by 10 per digit with no
  guard, so a ~19-digit integer literal wrapped i64 **negative** and a ~19-digit
  fraction wrapped `frac_div` into a nonsense divisor — both returning silent
  garbage. Replaced with an exact 18-digit mantissa plus a decimal exponent
  (`ABACO_MANT_DIGITS`, `ABACO_POW10_MAX`): the mantissa stops at the largest
  count that cannot overflow, and every further digit only moves the exponent,
  so there is no wrap point left. This is also *more* accurate for fractions —
  `3.14` is now one correctly-rounded 314/100 rather than an accumulation of
  inexact tenths — and it keeps whole numbers bit-exact, which `eval_pow`'s
  integer fast path depends on.
  Two follow-on effects: the literal's own decimal exponent and the scientific
  exponent are now combined *before* clamping (clamping them separately let a
  long fraction and a large exponent cancel into the wrong magnitude), and
  negative exponents divide in steps so representable subnormals like `1e-320`
  no longer collapse to 0. `1e400` now yields `+Inf` rather than the old finite
  `1e308` — the correct IEEE-754 overflow result, where the old clamp turned an
  overflow into a plausible-looking finite number.
- **`CalcHistory_from_json` lost the entire history if any field contained a
  brace.** Object boundaries were found by counting raw `{`/`}` without skipping
  string contents. Braces inside a JSON string are legal and need no escaping —
  and `CalcHistory_to_json` emits them verbatim — so one `{` or `}` in any
  input, result or timestamp shifted the boundary and made the rest of the array
  unparseable. A two-entry history containing `a}b` round-tripped to **zero**
  entries, silently, meaning `save_to_file` → `load_from_file` discarded
  everything. A `]` inside a field ended the array early by the same mechanism.
  All structural scanning now goes through a `_jf_skip_string` helper that
  honours backslash escapes. **Found by the new `fuzz_ai` harness on its first
  run.**
- **`_jf_find_value` matched key names inside values.** It searched for
  `"key"` anywhere in the object slice, so a value whose text looked like a key
  could be picked up as that field. It stayed benign only because
  `CalcHistory_to_json` always writes the real key first; a hand-written or
  attacker-supplied file carries no such ordering guarantee, and
  `load_from_file` reads untrusted data. The scan is now string-aware and only
  matches a string that is actually in key position (followed by `:`).

- **Parser limit globals namespaced** — `MAX_TOKENS` / `MAX_DEPTH` →
  `ABACO_MAX_TOKENS` / `ABACO_MAX_DEPTH`, joining `ABACO_ERR_*`. `MAX_TOKENS`
  collided with `enum TokLim { MAX_TOKENS = 128; }` in the stdlib's
  `lib/patra.cyr`, and duplicate globals are a **warning, not an error**, so a
  consumer bundling `dist/abaco.cyr` alongside `patra` could silently get 128 —
  a 4× undersized token array — with only a build warning to show for it.
- **Fuzz harness could not reach any of the above.** `fuzz_eval` capped inputs
  at 48 bytes, so against a 512-token limit it had a 10× margin it could never
  close, and it emitted `^` only through the ~0.04%/byte wild-byte path. The
  cap is now 1200 bytes, `^` and `!` are first-class biased bytes, and every
  4th iteration runs a targeted adversarial shape (long exponent runs, deep
  sign chains, deep `^` chains, token-array overruns). Confirmed to
  discriminate: the extended harness **hangs** against the 2.3.3 evaluator and
  passes 20,000 iterations against this one.
- **CI lint gate was blind to a whole advisory class.** `.github/workflows/ci.yml`
  matched only `^\s*warn ` lines, but `cyrlint` prints untracked deferrals as
  `deferral line N: …` followed by a `0 warnings` summary — so the gate went
  green while `src/ai.cyr` carried two untracked deferrals through **2.3.1,
  2.3.2 and 2.3.3**, each of which recorded "lint clean". The gate now matches
  both forms.
- **`cyrius lint`/`fmt` glob invocations silently checked one file.** `cyrlint`
  and `cyrfmt` accept a *single* path; the extra arguments from
  `cyrius lint src/*.cyr` are ignored without a diagnostic, so that command —
  as documented in `CLAUDE.md`'s Quick Start and used for prior releases —
  checked only `src/ai.cyr` and reported clean for the whole crate. `CLAUDE.md`
  now documents the per-file loop that CI already used. (Re-checked per file at
  2.3.4: all 7 modules are genuinely clean.)
- **Consumer stdlib list in `README.md` and `docs/guides/consuming-abaco.md` was
  broken.** Both still told consumers to declare `"json"` and `"u128"` — modules
  that stopped existing when 6.2.x folded them into `bayan` (abaco's own
  `[deps].stdlib` was updated at 2.2.5; the consumer-facing copies were not).
  `cyrius deps` cannot resolve those names, so anyone following the README would
  have failed at step one. Both lists now read `…, "math", "ganita", "io",
  "net", "http", "bayan"` — `ganita` was also missing, which would have left
  abaco's extended transcendentals undefined at link time. Pre-existing since
  2.2.5, surfaced while re-checking the dependency story for this bump.
- **`README.md` distlib command** was the pre-6.2.x bare `cyrius distlib`;
  distlib is profile-based now and writes `dist/abaco-abaco.cyr`, so the
  documented step needs the profile argument and the rename.
- **`README.md` status counts** — "381 tests" / "56 benchmarks" were several
  releases stale; now 547 asserts / 77 benchmarks. Consumer-snippet `tag` in
  `README.md` and the guide bumped 2.2.1 → 2.3.4.
- **`README.md` called hisab a consumer.** It is a *sibling* higher-math library
  with a distinct domain, as `CLAUDE.md` and `docs/development/state.md` both
  state; the ecosystem list now agrees with them.

## [2.3.3] — 2026-07-17

**Cyrius 6.4.66 toolchain bump + error-enum namespacing.** A maintenance patch
tracking the toolchain update, plus one hygiene fix surfaced by the 6.4.x lint.
**No stdlib re-batch** — unlike 2.2.5, the 6.4.x stdlib keeps the same module
layout, so abaco's `[deps].stdlib` list is unchanged and no source symbols
moved; the 6.4.x bundles only grew internally. The 6.4.x `cyrlint` adds an
error-enum-namespace advisory (proposal `2026-07-11-error-enum-namespace-lint-gate`):
a leaf library must prefix its error enum (`<LIB>_ERR_*`) so flat enum constants
don't collide across libraries. abaco's `EvalErr` members were bare `ERR_*`; they
are now `ABACO_ERR_*`. Suite green at **479 asserts, 0 failures**; fuzz 3/3 (5000
iterations each); fmt/lint/vet clean; clean-from-scratch DCE build passes;
benchmarks flat-to-faster (no regression).

### Changed

- **Toolchain pin 6.3.10 → 6.4.66** (`cyrius.cyml [package].cyrius`).
- **Stdlib re-vendored at 6.4.66** (`cyrius deps`). The module list in
  `[deps].stdlib` is unchanged — all 18 modules abaco depends on still exist and
  expose the same API. The 6.4.x bundles grew internally (e.g. `bayan`
  +1106 lines, `math` +246, `io` +157, `syscalls` +53, `alloc` +39, `net` +22,
  `fnptr` +17, `ganita` +7); the 10 unchanged declared deps (`string`, `fmt`,
  `vec`, `str`, `tagged`, `hashmap`, `assert`, `bench`, `args`, `http`) are
  byte-identical. The symbols abaco calls (`bayan_u64_powmod`,
  `bayan_json_{parse,key,value}`, the `ganita` extended transcendentals /
  combinatorics) resolve unchanged.
- **`EvalErr` error enum namespaced `ERR_*` → `ABACO_ERR_*`** — `ERR_NONE`,
  `ERR_DIV_ZERO`, `ERR_UNKNOWN_FN`, `ERR_UNKNOWN_VAR`, `ERR_PARSE`, `ERR_MATH`,
  `ERR_INVALID` are now `ABACO_ERR_*`. Clears the new 6.4.x `cyrlint`
  error-enum-namespace advisory (a leaf lib must prefix its error enum so the
  flat enum constant doesn't collide with the sakshi base logger's reserved
  `ERR_*` or another library's). Behaviour-neutral: the underlying integer
  values (0–6) and all control flow are identical; only the identifiers change.
  Callers comparing `eval_err(e)` against these constants must use the new
  names (safe: no live bundle consumer is wired yet).
- **Smoke binary `build/abaco`** ≈ 353,408 B — down ~4.3 KB from 357,736 B at
  2.3.1/6.3.10. The 6.4.x stdlib carries more code that DCE NOPs (1134
  unreachable fns, 286,827 B NOPed vs 1072/271,671 B at 6.3.10) but the resident
  binary is marginally smaller.
- **`dist/abaco.cyr` regenerated** — **no longer byte-identical to 2.3.2**; the
  bundle now carries the `ABACO_ERR_*` names (and the 2.3.3 version header).
  Consumers must re-vendor the bundle.

## [2.3.2] — 2026-07-05

**dB conversion constant fix.** The DSP decibel constants `DB_SCALE` (20/ln10),
`DB_EXP` (ln10/20), and `DB_GAIN_EXP` (ln10/40) were encoded from slightly-wrong
f64 bit patterns — all three shared a corrupted mantissa (`…764D5B4BCDB5`).
`DB_SCALE * DB_EXP` came to **0.99919** instead of ~1.0, so
`amplitude_to_db(db_to_amplitude(db))` was no longer an identity: the round trip
drifted **~0.081% of |db|**, exceeding a 0.01 dB tolerance for |db| ≥ 20. Small
levels looked fine (which is why the pre-existing tests missed it), but at
mixing/mastering levels the error was real — −0.049 dB at |db|=60. This is a
Cyrius-port encoding regression, **not** an f32→f64 precision issue: the Rust
`abaco-1.1.0` f32 originals round-trip tightly. Suite green at **479 asserts, 0
failures** (+7 from the new round-trip guard); fmt/lint/vet clean.

### Fixed

- **`DB_SCALE` `0x40215D38B4E225BA` → `0x40215F2CED384F28`** (8.682073… →
  8.685889638065035 — the correctly-rounded 20/ln(10)).
- **`DB_EXP` `0x3FBD764D5B4BCDB5` → `0x3FBD791C5F888823`** (0.115086… →
  0.11512925464970229 — ln(10)/20).
- **`DB_GAIN_EXP` `0x3FAD764D5B4BCDB5` → `0x3FAD791C5F888823`** (0.057543… →
  0.057564627324851146 — ln(10)/40, used in filter gain/shelf coefficient design).

### Added

- **`test_db_roundtrip`** in `tests/test_dsp.tcyr` — asserts
  `amplitude_to_db(db_to_amplitude(db)) == db` within 0.01 dB across
  ±{0, 6, 20, 60}. Anchored at |db|=60 it fails on the old constants, guarding
  the encoding going forward.

### Changed

- **`dist/abaco.cyr` regenerated** — **no longer byte-identical to 2.3.0/2.3.1**;
  it now carries the corrected constants (and an expanded rationale comment).
  Consumers must re-vendor the bundle.

## [2.3.1] — 2026-06-30

**Cyrius 6.3.10 toolchain bump.** A maintenance patch tracking the toolchain
update. **No stdlib re-batch this time** — unlike 2.2.5, the 6.3.x stdlib keeps
the same module layout, so abaco's `[deps].stdlib` list is unchanged and no
source symbols moved. No functional change to abaco's surface; the `dist/abaco.cyr`
bundle is byte-identical to 2.3.0 except for its version header. Suite green at
**472 asserts, 0 failures**; fuzz 3/3 (5000 iterations each); fmt/lint/vet clean;
clean-from-scratch DCE build passes; benchmarks flat (no regression).

### Changed

- **Toolchain pin 6.2.11 → 6.3.10** (`cyrius.cyml [package].cyrius`).
- **Stdlib re-vendored at 6.3.10** (`cyrius deps`). The module list in
  `[deps].stdlib` is unchanged — all 18 modules abaco depends on still exist and
  expose the same API. The 6.3.x bundles grew internally (e.g. `bayan`
  +144 lines, `net` +38, `math` +5) but the symbols abaco calls
  (`bayan_u64_powmod`, `bayan_json_{parse,key,value}`, the `ganita` extended
  transcendentals / combinatorics) resolve unchanged.
- **Smoke binary `build/abaco`** ≈ 357,736 B (~358 KB) — essentially unchanged
  from 2.3.0; the marginal growth tracks the larger 6.3.x `bayan`/`net` bundles
  that DCE NOPs but leaves resident (1072 unreachable fns, 271,671 B NOPed).

## [2.3.0] — 2026-06-15

**Opens the 2.3.x ecosystem-rollout minor with the deferred API cleanups.** The
two source/API changes held back from patch releases (an API change fits a
minor, not a patch) land here. Suite green at **472 asserts, 0 failures**; fuzz
3/3; fmt/lint/vet clean; clean-from-scratch DCE build passes; benchmarks hold
no regression.

### Changed

- **Migrated off the deprecated `bayan` back-compat aliases** to the canonical
  `bayan_*` API (deferred from 2.2.5). `mod_pow` now calls `bayan_u64_powmod`;
  the currency-cache JSON path calls `bayan_json_parse` / `bayan_json_key` /
  `bayan_json_value`. abaco no longer routes through bayan's deprecated shim
  layer, so it is unaffected when those aliases are eventually removed upstream.

### Removed

- **Trimmed three semi-public uncalled helpers** flagged in the 2.2.4 dead-code
  audit (deferred to the 2.3.0 boundary where an API change is appropriate):
  - `mod_mul` (`src/ntheory.cyr`) — superseded; `is_prime` uses `mod_pow` only.
    Removing it drops abaco's last use of `bayan_u64_mulmod`.
  - `reg_alias_exact` (`src/units.cyr`) — the live alias path is `reg_alias`.
  - `eval_has_more` (`src/eval.cyr`) — the parser uses `eval_at` / `eval_not_at`.

  These were never exercised by abaco's tests and were DCE-stripped from
  binaries already; the removal is a source-surface cleanup. Technically a
  public-symbol removal, hence the minor bump.

## [2.2.5] — 2026-06-15

**Cyrius 6.2.11 toolchain bump + stdlib re-batch.** A maintenance patch tracking
the toolchain's stdlib reorganization. No functional change to abaco's surface —
suite green at **472 asserts, 0 failures**; fuzz 3/3; fmt/lint/vet clean.

### Changed

- **Toolchain pin 6.0.1 → 6.2.11** (`cyrius.cyml [package].cyrius`).
- **Stdlib re-batch in `[deps].stdlib`.** The 6.2.x stdlib merged several
  modules; abaco's dependency list follows. Source symbols are unchanged —
  they resolve through the new bundles:
  - `json` + `u128` → **`bayan`** (batched bundle: base64/csv/u128/bigint/toml/
    cyml/json). `json_{parse,key,value}` and `u64_mulmod/powmod` resolve via
    bayan's back-compat aliases.
  - Extended math (`f64_{a,}sinh/cosh/tanh`, `f64_asin/acos`, `f64_atan2`,
    `fibonacci`, `binomial`) moved out of `math` into **`ganita`** — added to
    the deps list. The slim `math` retains the basics (`f64_sin/cos/sqrt/log`,
    `f64_pow`, constants).
- **`distlib` is profile-based.** 6.2.x dropped the flat `[lib]` form; the
  manifest now uses `[lib.abaco]` and `cyrius distlib abaco` writes
  `dist/abaco-abaco.cyr`, renamed to the consumer path `dist/abaco.cyr`. CI and
  release workflows updated to match.
- **Test includes.** `tests/test_{ai,integration}.tcyr` pointed their dead
  `include "lib/json.cyr"` at `lib/bayan.cyr`.
- **Smoke binary `build/abaco`** grew ~247 KB → ~356 KB: the batched
  `bayan`/`ganita` bundles carry more code that DCE NOPs but leaves resident
  (expected for a library smoke entry; consumers DCE per their own surface).

### Deferred (2.3.0)

- Migrate off the deprecated `bayan` back-compat aliases to the canonical
  `bayan_*` API. The shims exist only for the downstream migration window and
  will be removed once consumers re-pin — appropriate at the 2.3.0 boundary,
  not a patch.

## [2.2.4] — 2026-05-26

**Closeout of the 2.2.x modernization arc** (last patch before 2.3.0). A
full closeout pass — clean build, full suite, dead-code audit, downstream
consumability check, security re-scan, doc sync, version verify. Suite green at
**472 asserts, 0 failures**; clean-from-scratch DCE build passes.

### Verified

- **Downstream consumability** — built a synthetic consumer that depends on
  abaco purely via `[deps.abaco] modules = ["dist/abaco.cyr"]` + the stdlib list
  (no manual includes), exercising `Evaluator_eval`, `UnitRegistry_convert`, and
  `is_prime`. Confirms the bundle contract ([ADR 0001](docs/adr/0001-dist-bundle-distribution.md))
  is genuinely consumable end-to-end.
- **Clean build** — `rm -rf build && cyrius deps && CYRIUS_DCE=1 cyrius build`
  passes; smoke binary 246,968 B.
- **Security re-scan** — no `exec`/`fork`/`sys_system`, no hardcoded system
  paths, no large stack buffers (CI parity).

### Changed

- **Dead-code audit.** Removed the unused private helper `_nl_split` in
  `src/ai.cyr` (superseded by `_nl_split_ws`). Floor: the smoke `main()`
  exercises no library surface, so DCE NOPs the whole library (709 fns /
  ~185 KB) — this is expected for a library and not a source-cleanliness
  signal. Of the public surface, ~40 functions (`Value_*`, `ConversionResult_*`,
  `batch_*`, `CalcHistory_save/load_from_file`, …) are exercised by consumers
  rather than abaco's own tests — retained as product surface. A small set of
  semi-public-but-uncalled helpers (`mod_mul`, `reg_alias_exact`,
  `eval_has_more`) is flagged for a possible trim at the **2.3.0** boundary,
  where an API change is appropriate; kept here to avoid a breaking change in a
  patch (DCE strips them from binaries regardless).
- **Consumer docs corrected.** abaco has **no live bundle consumer today**.
  hisab is a *sibling* higher-math library (linear algebra / geometry /
  calculus) with a distinct domain — it does not depend on abaco. The intended
  consumers (the Abacus desktop app; dhvani, from which abaco's DSP was ported)
  are an ecosystem-rollout item tracked for 2.3.x. `state.md` / `CLAUDE.md`
  updated to say *planned* rather than current.

### Notes

- No functional source change beyond the `_nl_split` removal; `dist/abaco.cyr`
  differs from 2.2.3 only by that removal + the version stamp.

## [2.2.3] — 2026-05-26

Third slot of the 2.2.x arc: **P(-1) hardening + convention alignment**. A fresh
security audit under 6.0.1 closes the two deferred items from the 2026-04-14
audit. Suite green at **472 asserts, 0 failures** (was 470).

### Security

- **MED-4 — `json_parse` stack-exhaustion guard (fixed).** `lib/json.cyr`'s
  `json_parse` recurses once per nesting level with no depth cap, and abaco
  feeds it the `rates` object from a currency endpoint. A deeply-nested payload
  from a compromised / MITM'd server could exhaust the native stack before any
  value is read. Added `_ccy_json_depth_ok` — bounds nesting at
  `CCY_MAX_JSON_DEPTH = 8` before `json_parse` runs (legitimate fiat-rate maps
  are depth-1). Regression: `test_ccy_deep_nesting_rejected`. Defense-in-depth
  over the existing HTTPS guard. (+2 asserts.)
- **LOW-8 — hashmap HashDoS (documented; upstream).** `lib/hashmap.cyr` uses an
  unseeded content hash (CWE-407). abaco's residual risk is **LOW** — registry
  keys are fixed/trusted, currency keys arrive only over validated HTTPS, and
  expression variable names are consumer-supplied. No abaco-side change; a
  seeded/SipHash-class hash is recommended for the cyrius stdlib.
- Full report: [`docs/audit/2026-05-26-audit.md`](docs/audit/2026-05-26-audit.md).
  The 2026-04-14 mitigations (HIGH-1/2/3, MED-5/6/7, LOW-9a/10) were re-verified
  under 6.0.1. No new HIGH/CRITICAL findings; `src/` has no fixed-size stack
  buffers and no raw syscalls.

### Changed

- **Fuzz harnesses renamed `fuzz/fuzz_*.cyr` → `fuzz/fuzz_*.fcyr`** (the
  patra/sigil `.fcyr` extension); `fuzz/run.sh` simplified to a `*.fcyr` loop and
  CI updated to glob `fuzz/*.fcyr`.
- `src/ai.cyr` — added the MED-4 depth guard (the bundle grows accordingly).

### Added

- `docs/audit/2026-05-26-audit.md` — the P(-1) hardening audit report.
- `docs/benchmarks.md` — 3-point trend (baseline → 4.8.5-optimized → 6.0.1)
  proving no regression held across the arc; `is_prime_small` 17µs → 2µs → 1µs.
- [ADR 0004](docs/adr/0004-error-handling-defer-sakshi.md) — error handling stays
  enums-by-value; **sakshi adoption deferred on heft-vs-need** (user-ratified): a
  transitive dep every consumer must vendor, with no concrete need in abaco
  today. Recorded as an explicit, non-conforming exception with concrete
  re-examine triggers — a consumer surfacing a specific need, or sakshi being
  folded into the stdlib. Same evaluation: flat `tests/*.tcyr` layout kept.

## [2.2.2] — 2026-05-26

Second slot of the 2.2.x modernization arc: **documentation depth + light
hardening**. No source changes — `src/` is byte-identical to 2.2.1 (the bundle
differs only by version stamp); this slot adds tests, docs, and a fresh
benchmark baseline. Suite green at **470 asserts, 0 failures** (was 452).

### Added

- **`docs/architecture/`** — non-obvious invariants that can't be derived from a
  single function: token storage layout, expression depth bound (`MAX_DEPTH`),
  unit-registry two-map hashing + lookup order, Miller–Rabin witness validity
  bound. (README + 4 numbered notes.)
- **`docs/adr/`** — architecture decision records (+ template): dist-bundle
  distribution (0001), `eval_pow` vs stdlib `f64_pow` precision fast path
  (0002), deterministic Miller–Rabin over a fixed witness set (0003).
- **`docs/guides/consuming-abaco.md`** — how hisab/dhvani/Abacus wire
  `dist/abaco.cyr` as a dependency, with an API surface map.
- **LOW-9b regression tests** (closes the open 2026-04-14 audit item):
  - `test_ai::test_ccy_truncated_body` — a truncated body / lying
    Content-Length is rejected cleanly (`AI_ERR_CURRENCY`), and `_jf_get_string`
    is shown len-bounded (no over-read past the declared length). +5 asserts.
  - `test_units::test_hashmap_collisions` — a 200-entry round-trip exercises the
    cstr-keyed registry map well past its 16-slot cap (guaranteed collisions +
    grows); every key must read back. +13 asserts (with integrity check).
  - `test_units::test_registry_integrity` — per-category lookup + conversion
    integrity across the registry surface.

### Changed

- **`docs/sources.md` completeness audit** — every algorithm, formula, and
  constant cross-checked against a citation; added PolyBLEP (Välimäki &
  Huovilainen 2007), the constant-power pan/crossfade law, and the exponential
  envelope time constant.
- **`docs/README.md`** index refreshed (architecture/, adr/, guides/, sources,
  doc-health, development/); **`docs/mcp-tools.md`** read-through vs `src/ai.cyr`
  — the 5 tools mapped to their backing functions, implementation status (engine
  vs consumer-side server) clarified.
- **Benchmark baseline refreshed under 6.0.1** (`bench-latest.md` +
  `bench-history.csv`) as the early arc baseline; `is_prime_small` 2µs → 1µs.
  The full 3-point trend lands in 2.2.3.
- `docs/doc-health.md` refreshed: 30 docs, 0 stale, 0 read-through outstanding.

## [2.2.1] — 2026-05-26

Cyrius 5.7.23 → 6.0.1 upgrade and first slot of the 2.2.x modernization
arc — aligning abaco with the patra/sigil first-party reference shape.
Full suite green at **452 asserts, 0 failures** (up from 442 with SIMD
quarantined). No public API changes. Forward plan:
[`docs/development/roadmap.md`](docs/development/roadmap.md).

### Changed

- **Cyrius pin 5.7.23 → 6.0.1.** 6.0.x is internal to the toolchain
  (the `cc5→cycc` / `cyrc→cybs` rename ceremony plus two stdlib-path
  fixes) — abaco's `src/` builds clean against it with no language
  changes required.
- **`lib/` is no longer committed.** It's a build artifact regenerated
  by `cyrius deps` from `[deps].stdlib`, gitignored to match patra/sigil.
  Eliminates the "cwd ./lib/ shadows version-pinned snapshot" note and
  keeps the vendored stdlib version-matched to the pin automatically.
- **CI/release modernized to the patra/sigil shape:**
  - Toolchain installs via the upstream `scripts/install.sh`, reading the
    version straight from the `cyrius.cyml` pin (single source of truth).
  - CI now gates on `cyrius fmt --check` and `cyrius lint` (src is
    warning-clean), verifies `dist/abaco.cyr` is current, and runs the
    full `.tcyr` suite (SIMD included).
  - Release regenerates the bundle with `cyrius distlib` and ships
    `dist/abaco.cyr` alongside the source tarball + smoke binary.
  - Release tag filter narrowed to plain semver (`X.Y.Z`); the old
    `v`-prefixed style is dropped to match `git tag $(cat VERSION)`.
- Local `f64_pow` in `src/eval.cyr` renamed to **`eval_pow`** — it shadowed
  stdlib `lib/math.cyr`'s `f64_pow` (duplicate-fn warning). The evaluator
  keeps its integer-exponent fast path (exact whole powers); the rename
  just disambiguates it from the stdlib transcendental form.

### Added

- **`dist/abaco.cyr` — self-contained library bundle** generated by
  `cyrius distlib` from the new `[lib] modules` list in `cyrius.cyml`.
  This is the consumer-facing artifact: hisab, dhvani, and the Abacus
  desktop app import abaco via `[deps.abaco] modules = ["dist/abaco.cyr"]`.
- `docs/development/roadmap.md` (forward 2.2.x arc) and
  `docs/development/state.md` (volatile state snapshot), per first-party
  standards.
- `docs/sources.md` — academic/domain citations for every algorithm,
  formula, and constant (required for a math crate).

### Fixed

- **`tests/test_ntheory.tcyr` compile failure** under 6.0.1 — it compared
  `tag(r) == ERR` / `tag(r) == OK` against bare constants the stdlib
  dropped in the v5.8.x `Result` migration. `prev_prime` returns
  `Ok`/`Err`, so the asserts now use `is_ok(r)` / `is_err_result(r)` from
  `lib/result.cyr`. (107 asserts.)
- `src/core.cyr` consecutive blank lines and `src/dsp.cyr` 120-column
  overflow (cubic-interpolation coefficient split into an intermediate) —
  both cleared so the lint gate can be blocking.

### Removed

- **`tests/test_simd.tcyr` quarantine.** The `f64v_*` SIMD intrinsics
  that SIGSEGV'd under Cyrius 5.7.23 are fixed in 6.0.1 — the test passes
  (10 asserts) and `src/dsp.cyr`'s `batch_*` wrappers are sound again.
  CI no longer skips it.
- 25 committed `lib/*.cyr` stdlib files (now regenerated via `cyrius deps`).

### Migration

- Run `cyrius deps` after pulling to vendor the 6.0.1 stdlib into `lib/`.
- Consumers: `dist/abaco.cyr` is the import surface — no source-API change,
  but the bundle is new this release.

## [2.2.0] — 2026-04-28

Cyrius 4.10.2 → 5.7.23 upgrade and project modernization. No source
changes — abaco's `src/` builds clean against 5.7.23. 442 test asserts
pass; `tests/test_simd.tcyr` is quarantined (see Known issues).

### Changed

- **Cyrius bumped 4.10.2 → 5.7.23.** All `lib/` stdlib files refreshed
  via `cyrius deps`.
- **Manifest renamed `cyrius.toml` → `cyrius.cyml`** and modernized:
  - `version` now interpolates from the `VERSION` file via
    `${file:VERSION}` (single source of truth).
  - Added `repository = "https://github.com/MacCracken/abaco"`.
  - `output` is now `build/abaco` to match the standard layout.
  - `[deps].stdlib` is now multi-line for diff-friendliness.
- **CI/release workflows modernized** to match the yukti/daimon shape:
  - Toolchain version is read directly from `cyrius.cyml` — no more
    parallel `.cyrius-toolchain` pin to keep in sync.
  - New steps: `cyrius deps`, `cyrius vet`, advisory `cyrius lint`,
    DCE-enabled build, ELF verification, best-effort aarch64
    cross-build, security pattern scan, doc-presence check.
  - Release workflow accepts both `1.2.3` and `v1.2.3` tag styles,
    archives source + per-arch binaries with `SHA256SUMS`, and pulls
    the per-version body straight from `CHANGELOG.md`.

### Removed

- `.cyrius-toolchain` — superseded by reading the version from
  `cyrius.cyml` (`grep -oP '(?<=^cyrius = ")[^"]+' cyrius.cyml`).
- 40 stale `lib/*.cyr` files that were never referenced from `src/`
  (leftovers from earlier dep experiments: `audio`, `chrono`,
  `linalg`, `mabda`, `patra`, `regex`, `sakshi`, `sigil`, `thread`,
  `toml`, `ws`, `yukti`, etc.). `cyrius deps` now produces a `lib/`
  that exactly matches `[deps].stdlib`.

### Known issues

- **`tests/test_simd.tcyr` quarantined.** The `f64v_*` SIMD
  intrinsics (`f64v_add` / `f64v_sub` / `f64v_mul` / `f64v_div` /
  `f64v_sqrt` / `f64v_abs` / `f64v_fmadd`) SIGSEGV at runtime under
  Cyrius 5.7.23. This is a pre-existing compiler regression — the
  test last passed on 4.8.5; the prior CI loop only checked for
  the literal string `failed` in test output, so the crash went
  unnoticed across the 4.8.5 → 4.10.2 bump. The new CI's strict
  exit-code check exposes it. `src/dsp.cyr`'s `batch_*` thin
  wrappers (`batch_add` / `batch_sub` / `batch_mul` / `batch_div`
  / `batch_sqrt` / `batch_abs` / `batch_mac` / `batch_fmadd`) are
  affected at runtime too — DCE strips them in release builds
  today, but any caller that lands will hit the same crash. CI
  and release workflows skip `test_simd` until the intrinsics are
  fixed upstream.

## [2.1.0] — 2026-04-15

Cyrius 4.10.2 upgrade — stdlib now provides primitives that were
previously hand-rolled in abaco. Net effect: less code to maintain,
identical behaviour, and abaco gains stdlib improvements for free
(e.g. `f64_parse` handles scientific notation and NaN/Inf).

### Changed

- **Cyrius bumped 4.8.5 → 4.10.2.** All `lib/` stdlib files synced.
- **`src/dsp.cyr`** — removed 8 hand-rolled functions now in stdlib
  `math.cyr`: `f64_clamp`, `f64_max`, `f64_min`, `f64_lerp`,
  `f64_hypot`, `f64_trunc`, `f64_fract`, `f64_sign`. Call sites
  resolve to the stdlib versions transparently.
- **`src/eval.cyr`** — `CONST_PI`/`CONST_E`/`CONST_TAU` now alias
  `F64_PI`/`F64_E`/`F64_TAU` from stdlib. `gcd_int` delegates to
  stdlib `gcd()`. `lcm()` dispatch delegates to stdlib `lcm()`.
- **`src/ntheory.cyr`** — `fibonacci` and `binomial` removed; callers
  resolve to identical stdlib implementations in `math.cyr`.
- **`src/ai.cyr`** — `_nl_parse_f64` now uses stdlib `f64_parse` for
  the heavy lifting while preserving strict "entire string consumed"
  semantics (CWE-917 guard intact).

### Added (stdlib)

- **`lib/math.cyr`** gains: `f64_lerp`, `f64_hypot`, `f64_sign`,
  `f64_trunc`, `f64_fract`, `gcd`, `lcm`, `fibonacci`, `binomial`,
  `f64_parse`, `f64_parse_ok`.
- **`lib/fmt.cyr`** — `fmt_sprintf` now takes a `bufsz` parameter for
  bounds-checked formatting (breaking change in stdlib; no abaco call
  sites affected).
- **`lib/linalg.cyr`** — new stdlib module (LU, Cholesky, QR,
  determinant, inverse, least-squares). Not yet dep'd by abaco.
- **`lib/cyml.cyr`** — new stdlib module (CYML document parser).

## [2.0.0] — 2026-04-14

Major version bump: abaco is no longer a Rust crate. The entire library
has been ported to [Cyrius][cyrius] and the Rust implementation removed.
This is a breaking change for anyone who was depending on `abaco` via
`crates.io` or Cargo.

[cyrius]: https://github.com/MacCracken/cyrius

### Breaking

- **Implementation language changed from Rust to Cyrius.** `Cargo.toml`,
  `crates/*`, and all `.rs` sources are gone.
- **Distribution format changed.** abaco is now a Cyrius module set
  consumed via `[deps.abaco]` in a downstream `cyrius.toml`, not a
  crates.io dependency.
- **API shape changed.** Method-style `Evaluator::new()`/`.eval()` is now
  prefix-style `Evaluator_new()` / `Evaluator_eval(e, ...)`. See
  `docs/architecture.md` for naming conventions.
- **f64 values are bit patterns** through the public API (Cyrius convention).
- **`#[non_exhaustive]`, `Serialize`/`Deserialize`, `Display`, async
  futures** — no Rust-specific annotations apply anymore. Structured
  output goes through explicit `*_to_latex`, `*_to_json`, etc.
- **No Cargo features.** The `ai` feature is now an included module
  (`src/ai.cyr`), not feature-gated at link time. Cyrius's cross-unit
  DCE strips unused modules at build.

### Added — Cyrius port

- **`src/ai.cyr`** (520 lines) — NL parsing, `CalcHistory`,
  `CurrencyCache` with live `http_get` fetch and nested JSON extractor.
- **DSP expansions** — Hann / Hamming / Blackman / Kaiser windows,
  `window_kaiser_fill` (hoists I0(β) denom), `_bessel_i0`, `f64_cubic`
  (Catmull–Rom), `f64_sinc`, `sinc_kernel`, `freq_to_pitch_class`,
  `freq_to_octave`, `pitch_class_name`, `samples_to_ms` /
  `ms_to_samples`, `bpm_to_hz` / `hz_to_bpm`.
- **`CAT_PITCH` category** — semitone / cent / octave unit conversions.
- **BPM in `CAT_FREQUENCY`** — `registry.convert(120, "bpm", "Hz")`
  works naturally.
- **Multi-word aliases** — `"square kilometers"`, `"meters per second"`,
  `"kilometers per hour"`, `"miles per hour"`, `"miles per gallon"`.
- **`programs/basic.cyr`** — runnable end-to-end demo.
- **`fuzz/` harnesses** — `fuzz_eval`, `fuzz_ntheory`, `fuzz_units`
  with a `run.sh` runner. Clean at 50k iters each.
- **`cyrius capacity`** + `cyrius doc --check` wired into the dev loop.

### Changed

- Test count: 283 → **381 assertions** across 6 `.tcyr` files.
- Benchmarks: 56 tracked in `bench-history.csv`, last-3-runs table in
  `bench-latest.md`.
- Hyperbolic trig (`sinh`/`cosh`/`tanh`) now uses stdlib
  `lib/math.cyr::f64_sinh/cosh/tanh` instead of inlined formulas.
- Numeric constants in `src/dsp.cyr` use `_` digit separators
  (Cyrius 4.8.0): `0x4009_21FB_5444_2D18`.
- Docs rewritten for Cyrius — `README.md`, `docs/architecture.md`,
  `docs/development.md`, `CONTRIBUTING.md`, `SECURITY.md`.

### Known gaps

- **u128 is_prime perf** — Cyrius 4.8.0 `u128_mod` software long-division
  is ~40× slower than the current binary double-and-add; reverted.
  Waiting on hardware 128-bit div-mod emission.
- **`asin` / `acos` / `atan` / `atan2`** — still identity-formula stopgaps;
  filed as P1-2 in `cyrius/docs/issues/stdlib-math-recommendations-from-abaco.md`.
- **dBFS** — log-scale unit, requires special handling beyond the
  linear `to_base` factor; deferred.

## [1.1.0] - 2026-03-27

### Added

- **`ntheory` module** — number theory primitives, zero dependencies:
  - `is_prime(n)` — deterministic Miller-Rabin, correct for all u64 (Sorenson & Webster 2015 witnesses)
  - `next_prime(n)`, `prev_prime(n)` — nearest prime search
  - `factor(n)` — prime factorization via trial division, returns sorted `Vec<u64>`
  - `totient(n)` — Euler's totient function
  - `fibonacci(n)` — fast doubling algorithm, exact for n <= 93
  - `binomial(n, k)` — overflow-safe multiplicative formula
- **Evaluator functions** from ntheory: `isprime`, `nextprime`, `prevprime`, `totient`, `fibonacci`/`fib`, `binomial`/`choose`
- 8 ntheory criterion benchmarks (is_prime, factor, totient, fibonacci, binomial)
- 28 new tests (348 total + 10 doctests)
- 8 doc-tested examples in ntheory module

### Changed

- Evaluator now supports 43+ functions (was 35+)
- Roadmap updated with hisab integration plan (solver bridge, symbolic algebra, verified evaluation)

## [1.0.0] - 2026-03-27

**Abaco's first stable release.** Public API is now frozen — no breaking changes without a major version bump.

### Added

- **Implicit multiplication** — `2(3+4)`, `2pi`, `(2)(3)`, `(3)4` all work naturally
- **Factorial** — `factorial(n)` function and `n!` postfix operator (0..170)
- **GCD / LCM** — `gcd(a, b)` and `lcm(a, b)` functions
- **Statistical functions** — `mean(...)`, `avg(...)`, `median(...)`, `stddev(...)`, `stdev(...)` with variable arity
- **LaTeX output** — `Value::to_latex()` renders fractions as `\frac{n}{d}`, complex as `a + bi`, large floats in scientific notation
- **Conversion history persistence** — `CalculationHistory::to_json()`, `from_json()`, `save_to_file()`, `load_from_file()`
- **Partial parse / live evaluation** — `Evaluator::eval_partial()` for live-as-you-type feedback with error recovery
- **`Token::Bang`** variant for `!` postfix factorial
- 37 new tests (320 total + 2 doctests)

### Changed

- `lib.rs` crate docs updated to reflect full 1.0 feature set
- Expression evaluator now supports 35+ functions (was 28+)

## [0.23.0] - 2026-03-27

### Added

- **4 new unit categories** (18 total, was 14):
  - **Fuel Economy**: km/L, mpg, L/100km with reciprocal conversion support
  - **Density**: kg/m³, g/cm³, g/mL, kg/L, lb/ft³
  - **Luminosity** (Illuminance): lux, foot-candle, lm/m², phot
  - **Viscosity** (Dynamic): Pa·s, mPa·s, poise, centipoise
- **Reciprocal unit conversion** — `Unit::new_inverse()` for units where `base = factor / value` (e.g., L/100km)
- **Unit aliases and abbreviation normalization** — 80+ aliases:
  - Temperature: °C, °F, degC, degF, centigrade
  - British spellings: metres, kilometres, litres, gramme
  - Common abbreviations: kph, kmh, sec, hrs, lbs, yrs
  - Area phrases: "sq m", "sq km", "square feet"
  - Speed phrases: "meters per second", "kilometers per hour"
- **Live currency exchange rates** via hoosh service (feature-gated: `ai`)
  - `CurrencyConverter` with configurable base URL and cache TTL
  - In-memory rate caching with TTL (default: 1 hour)
  - Offline fallback: uses stale cache when service is unreachable
  - `set_rates()` for manual/test rate injection
  - Cross-rate conversion (EUR→JPY goes through base currency)
- 30 new tests (283 total, was 253), 6 new benchmarks (56 total)

### Changed

- `Unit` struct gains `to_base_inverse: bool` field for reciprocal conversions
- `UnitCategory` enum: 4 new variants (FuelEconomy, Density, Luminosity, Viscosity)
- `AiError` enum: 2 new variants (CurrencyError, HttpError)
- Registry HashMap capacities increased for 120+ units + aliases
- `serde_json` and `uuid` dependencies removed (unused)
- `chrono` moved behind `ai` feature gate (was always-on)
- Default dependency count: 3 (serde, thiserror, tracing)

### Hardened (P-1 audit, pre-0.23)

- `#[non_exhaustive]` on all 7 public enums
- `#[must_use]` on all pure functions
- `#[inline]` on hot-path functions (tokenize, eval, find_unit, convert)
- Recursion depth limit (256) in expression evaluator — prevents stack overflow
- All dependencies updated to latest compatible versions

## [0.22.4] - 2026-03-22

### Added

- `dsp` module — pure numeric DSP math primitives for audio engines
  - Decibel conversions: `amplitude_to_db`, `db_to_amplitude` (f32 and f64 variants), `db_gain_factor`
  - MIDI: `midi_to_freq`, `freq_to_midi`, constants `A4_FREQUENCY`, `A4_MIDI_NOTE`, `SEMITONES_PER_OCTAVE`
  - Envelope: `time_constant` (one-pole smoothing coefficient from ms + sample rate)
  - Waveform: `poly_blep` (anti-aliasing correction), `angular_frequency` (biquad filter design)
  - Panning: `constant_power_pan` (sin/cos law), `equal_power_crossfade`
  - Utility: `sanitize_sample` (NaN/Inf → 0.0)
- 24 tests for dsp module
- 21 DSP criterion benchmarks (scalar + batch-4096)
- ROADMAP.md

### Performance

- dB conversions use `ln`/`exp` with precomputed constants instead of `log10`/`powf` — 42-62% faster
- MIDI-to-frequency uses `exp2` instead of `powf(2.0, x)`
- Pan/crossfade use single `sin_cos()` call instead of separate `sin()` + `cos()`

### Changed

- Benchmark script outputs both CSV history and 3-point tracking Markdown table
- 50 criterion benchmarks total (was 29), 242 tests

## [0.22.3] - 2026-03-22

### Performance

- Tokenizer rewritten to byte-level iteration: 43-62% faster expression evaluation
- Unit lookup indexed with HashMaps for O(1) symbol/name resolution: 94-98% faster lookups
- Registry creation pre-allocates HashMap capacity for 100+ units
- CalculationHistory switched from Vec to VecDeque for O(1) front eviction
- Function dispatch consolidated: arity check and dispatch in single match

### Added

- IEC binary data size units: KiB, MiB, GiB, TiB, PiB (powers of 1024)
- SI decimal data sizes corrected: kB, MB, GB, TB, PB now use powers of 1000
- Cross-conversion between SI and IEC (e.g. 1 GB = 0.931 GiB)
- 29 criterion benchmarks, 218 tests (all features), 99.4% line coverage
- CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md
- codecov.yml with 90% project target
- Example: examples/basic.rs
- CI: deny, MSRV, coverage, doc, benchmark, multi-platform test jobs
- Release workflow with crates.io publish and version verification

### Changed

- License aligned to AGPL-3.0-only across Cargo.toml, LICENSE, README
- Cargo.toml: added documentation, exclude fields
- deny.toml: added version fields, Unicode-DFS-2016
- Makefile: added coverage, test-all, doc with -D warnings
- CI: 8-job pipeline (was 4), multi-platform testing
- Release: library publish workflow (was binary packaging)
- .gitignore: comprehensive (was 6 lines)

### Fixed

- Bench-history script: handles criterion's wrapped benchmark name format

## [0.1.0] - 2026-03-22

### Changed — Flatten to shared math crate

- Refactored from multi-crate workspace to single flat library crate
- Extracted GUI and binary to [abacus](https://github.com/MacCracken/abacus)
- Feature-gated AI module behind `ai` feature flag
- Added rustls-tls to reqwest
- Removed binary deps (clap, anyhow, tracing-subscriber) — library only

### Modules

- `core` — Value types (Integer, Float, Fraction, Complex, Text), Unit, UnitCategory (14 categories), Currency
- `eval` — Tokenizer, recursive descent parser, evaluator with 28+ functions, variables, scientific notation, percentage shorthand
- `units` — Unit registry with 95+ built-in units across 14 categories, conversion engine
- `ai` — Natural language math parsing, calculation history (feature-gated)

[2.1.0]: https://github.com/MacCracken/abaco/compare/2.0.0...2.1.0
[0.22.4]: https://github.com/MacCracken/abaco/compare/0.22.3...0.22.4
[0.22.3]: https://github.com/MacCracken/abaco/compare/0.1.0...0.22.3
[0.1.0]: https://github.com/MacCracken/abaco/releases/tag/0.1.0
