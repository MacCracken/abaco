# Abaco — State Snapshot

> Volatile state, refreshed every release. Durable rules live in
> [`../../CLAUDE.md`](../../CLAUDE.md); forward plan in [`roadmap.md`](roadmap.md);
> per-tag history in [`../../CHANGELOG.md`](../../CHANGELOG.md).

**Last updated:** 2026-08-22 (2.4.5 — CI-only. **2.4.4's security-scan hardening broke CI on the clean path**: `grep` exits 1 when it finds nothing, and splitting the `grep | awk` pipeline made the assignment inherit that status, which `bash -e` treats as fatal — so the step died at the first pattern with no output at all. Fixed with a `|| rc=$?` list, along with the same latent hazard on the Test step, which fires on the *failure* path and would have swallowed 2.4.4's new failure-count reporting. Every runnable `ci.yml` step is now verified under `/usr/bin/bash -e` against the real tree; all 12 pass. No source change — `dist/abaco.cyr` byte-identical to 2.4.4 bar the version header)

**Previous:** 2026-08-22 (2.4.4 — nine of the ten follow-ups 2.4.3's audit opened, closed. Headline: **abaco's CI could not fail on a failing assertion** — every `.tcyr` discarded `assert_summary()`'s return value, so a red suite exited 0 and shipped green; proven on the 2.4.3 tree and fixed twice over. Plus two more `eval_pow` silent wrong answers (`0^(0-1)` = `+0`, and any exponent >= 2^63 = NaN), a `Value_to_latex` that printed the integer part of every float, a security comment whose stated mechanism does not exist, and three release gates that were checking nothing. Suite 729 -> **813**. `dist/abaco.cyr` **not** byte-identical to 2.4.3)

**Previous:** 2026-08-22 (2.4.3 — Cyrius 6.5.35. A pin bump that surfaced two silent wrong answers in `eval_pow`: `(-2)^0.5` returned `+√2` and every non-finite `pow` row returned NaN, all with `eval_err = NONE`. Both fixed per IEEE 754-2019 §9.2 / C99 F.10.4.4, cited and pinned by 67 new asserts. A SIGFPE also leaves `mod_pow`'s public surface via bayan 1.5.2 — but that fix rides the *consumer's* pin, not abaco's tag. ⛔ Miller–Rabin primality is **1.53× slower**, the measured price of that same SIGFPE fix. `dist/abaco.cyr` is **not** byte-identical to 2.4.2)

**Previous:** 2026-08-14 (2.4.2 — Cyrius 6.5.27. Both issues abaco filed at 2.4.1 are fixed upstream: the typed-pointer warning was testing the wrong sign, and `<source>` diagnostic lines shifted by one per prepended line. The fresh-local workaround the first one forced is reverted. No abaco behaviour change; parse accuracy byte-identical over the same 5000-literal corpus)

## Versions

| What | Value |
|------|-------|
| abaco | **2.4.5** |
| Cyrius toolchain pin | **6.5.35** |
| License | GPL-3.0-only |

## Stdlib (6.2.x batching — unchanged through 6.5.x)

The 6.2.x stdlib re-batched several modules; abaco's `[deps].stdlib` list
adjusted accordingly at 2.2.5. **6.3.x (2.3.1) through 6.5.x (2.3.4, 2.4.1, 2.4.2, 2.4.3)
keep the same layout** — no re-batch, so the dependency list is unchanged and all
18 declared modules still exist; the bundles only grow internally. At 6.5.35 vs
6.5.27: `bayan` **+10,046 lines** (1.4.1 → 1.5.2 — 1.5.0's PDF writer/reader is
most of it), `ganita` +56 (1.1.0 → 1.1.4), `fmt` +24 (the 6.5.30 `fmt_float_buf`
carry fix), `syscalls_windows` +34 (a Windows-only `enum Stat`) — the other **28
of 32** vendored modules byte-identical, with **zero symbol removals** and every
back-compat alias intact. (6.5.27 vs 6.5.21 was `ganita` +146, `syscalls_windows`
+58, `syscalls_macos` +25, `syscalls_aarch64_linux` +19, `net` +8; and for
reference, 6.5.20 vs 6.4.66 was the larger step: `bayan`
+774, `syscalls_x86_64_agnos` +442, `alloc` +241, `io` +203, `vec` +187,
`bench` +185, …) As of 2.3.0 abaco calls the **canonical
`bayan_*` API directly** — no longer the deprecated back-compat aliases:

| Was (6.0.x) | Now (6.2.x) | Canonical symbols abaco uses |
|-------------|-------------|------------------------------|
| `json`, `u128` | `bayan` (batched bundle) | `bayan_json_{parse,key,value}`, `bayan_u64_powmod` |
| extended `math` | `ganita` (math bundle) | `f64_{a,}sinh/cosh/tanh`, `f64_asin/acos`, `f64_atan2`, `fibonacci`, `binomial` |
| `math` (slim) | `math` | basics: `f64_sin/cos/sqrt/log`, constants, aarch64 polyfills (`f64_pow` lives in `ganita`, not here) |

**6.5.x compiler builtins.** `f64_round` is now a *reserved* builtin (ties to
**even**), so a library may no longer define it. It was already an intrinsic
well before that: building the 2.3.3 tree against 6.2.11, 6.3.10 and 6.4.66 all
answer `round(2.5) = 2`, meaning the intrinsic had been silently shadowing
abaco's hand-rolled `fn f64_round` — which said ties-*away*-from-zero — for four
releases, with no diagnostic. 6.5.x reserving the name is what made the
divergence visible. abaco's version is now `f64_round_half_away` (ties **away
from zero**, the documented and cited behaviour) and the two modes are pinned
apart by `test_round_ties_away`. 6.5.x also enforces **call arity**, so
`print(msg)` must be `print(msg, len)`.

## Artifacts

| Artifact | Size | Notes |
|----------|------|-------|
| `build/abaco` | ~620 KB | DCE smoke binary (`src/main.cyr`) — x86_64 ELF (619,576 B at 2.4.4/6.5.35, 619,480 B at 2.4.3; was 403,968 B at 2.4.2/6.5.27, 353,408 B at 2.3.3/6.4.66). The +211 KB is the bayan 1.4.1 → 1.5.2 fold, not abaco: unreachable fns 1239 → 1609, NOPed 323,339 → 475,575 B, and all 367 functions bayan gained land in the dead list. A 2×2 of compiler × stdlib splits it as stdlib **+219,608 B**, compiler **−8,192 B** (6.5.35's codegen is a net size win), with the `eval_pow` fix the remaining 4,096 B |
| `dist/abaco.cyr` | ~153 KB (~3.97k lines) | Committed consumer bundle. 6.2.x distlib is profile-based: `cyrius distlib abaco` → `dist/abaco-abaco.cyr`, renamed to `dist/abaco.cyr`. **2.4.4: 156,430 B / 4,027 lines** — adds the `eval_pow` zero-base and huge-exponent rows and the rewritten `Value_to_latex`; **not** byte-identical to 2.4.3. (2.4.3: 141,990 B / 3,759 lines) — carries the `eval_pow` domain + non-finite fixes, so **not** byte-identical to 2.4.2 and consumers must re-vendor for *behaviour*. The `dist/abaco-abaco.deps` sidecar distlib now also emits is gitignored, not shipped (its name can never match the consumer path, and its 12-leaf list prunes `net`, which `lib/http.cyr` needs). 2.4.2: 137,343 B / 3,674 lines — carries the `f64_round_half_away` rename, the evaluator bound-check fixes and string-aware JSON scanning; **not** byte-identical to 2.4.1 (consumers must re-vendor, and a consumer calling the bundle's old `f64_round` now silently gets the ties-to-even builtin) |

## Tests

**813 asserts, 0 failures** across 7 `.tcyr` files:

| Suite | Asserts |
|-------|---------|
| `test_ai` | 118 |
| `test_dsp` | 109 |
| `test_eval` | 379 |
| `test_integration` | 51 |
| `test_ntheory` | 112 |
| `test_simd` | 10 |
| `test_units` | 34 |

- Fuzz harnesses: **4** (`fuzz/fuzz_{eval,ntheory,units,ai}.fcyr`) — clean at 20,000 iters
- Benchmarks: 3 (`bench`, `bench_eval`, `bench_units`)
- fmt / lint / vet: clean — fmt now checked across `tests/`, `benches/` and
  `fuzz/` as well as `src/`, which is how three long-dirty files were found

## Library surface

6 modules (bundled into `dist/abaco.cyr` in this order):
`core` → `ntheory` → `dsp` → `eval` → `units` → `ai`. `src/main.cyr` is the
smoke entry, excluded from the bundle.

## Consumers

**No live bundle consumer today.** The `dist/abaco.cyr` contract is verified
consumable (2.2.4 closeout), but wiring real consumers is the 2.3.x ecosystem
rollout. Intended:

| Consumer | Domain | Status |
|----------|--------|--------|
| Abacus | Desktop calculator app | planned — will import `dist/abaco.cyr` (user-confirmed) |
| dhvani | Audio DSP | planned — abaco's DSP was ported *from* dhvani; rollout audit pending |

**Not a consumer:** hisab is a *sibling* higher-math library (linear algebra,
geometry, calculus, numerical methods) — distinct domain, no abaco dependency.

## In flight

- **2.2.x modernization arc CLOSED** (2.2.1–2.2.4); **2.2.5** tracked the
  Cyrius 6.2.11 toolchain bump + stdlib re-batch. **2.3.0** opened the
  ecosystem-rollout minor and landed the two deferred API cleanups:
  - ✅ Migrated off the deprecated `bayan` back-compat aliases to canonical
    `bayan_*` (`bayan_u64_powmod`, `bayan_json_{parse,key,value}`).
  - ✅ Trimmed the semi-public uncalled helpers `mod_mul`, `reg_alias_exact`,
    `eval_has_more` (the 2.2.4 dead-code audit flag).
- **2.3.1** tracked the **Cyrius 6.3.10 toolchain bump** — pin 6.2.11 → 6.3.10,
  stdlib re-vendored. No re-batch, no source change, no functional change;
  `dist/abaco.cyr` byte-identical to 2.3.0 bar the version header. (6.3.x stdlib
  now ships `lib/tls.cyr` + `tls_native_*` — relevant to the open currency-cache
  TLS item below, but not wired in that patch.)
- **2.3.2** fixes the **DSP dB conversion constants**. `DB_SCALE`
  (20/ln10), `DB_EXP` (ln10/20), and `DB_GAIN_EXP` (ln10/40) had been encoded
  from slightly-wrong f64 bit patterns (shared corrupted mantissa
  `…764D5B4BCDB5`); `DB_SCALE * DB_EXP` = 0.99919, so the
  `amplitude_to_db`/`db_to_amplitude` round trip drifted ~0.081% of |db| and
  blew a 0.01 dB tolerance for |db| ≥ 20. Re-encoded to the correctly-rounded
  values; added `test_db_roundtrip` (|db| up to 60). **`dist/abaco.cyr` is no
  longer byte-identical to 2.3.0/2.3.1** — consumers (dhvani) must re-vendor.
  A Cyrius-port encoding regression, not an f32→f64 issue.
- **2.3.3** tracks the **Cyrius 6.4.66 toolchain bump** — pin
  6.3.10 → 6.4.66, stdlib re-vendored. **No stdlib re-batch** (module list
  unchanged; 6.4.x bundles grew internally only). Also namespaced the `EvalErr`
  error enum `ERR_*` → `ABACO_ERR_*` to clear the new 6.4.x `cyrlint`
  error-enum-namespace advisory (leaf libs must prefix `<LIB>_ERR_*` so flat
  enum constants don't collide with the sakshi base logger / sibling libs).
  Behaviour-neutral (same integer values 0–6); `dist/abaco.cyr` regenerated with
  the new names — **no longer byte-identical to 2.3.2**. Suite green (479
  asserts); fuzz 3/3; fmt/lint/vet clean; DCE build passes; benchmarks
  flat-to-faster vs 6.3.10.
- **2.3.4** tracks the **Cyrius 6.5.20 toolchain bump** — pin
  6.4.66 → 6.5.20, stdlib re-vendored. **No stdlib re-batch**, but unlike 2.3.1
  and 2.3.3 this bump was **not source-neutral** — 6.5.x broke the build twice:
  - **`f64_round` became a compiler builtin** (reserved keyword), so abaco's
    hand-rolled `fn f64_round` in `src/dsp.cyr` no longer parses. The builtin is
    not a drop-in: it ties to **even** (`round(2.5) = 2`), abaco's said ties
    **away from zero** (`round(2.5) = 3`). Renamed to `f64_round_half_away`.
    **This changes `round()` for exact-`.5` inputs** — because the hand-rolled
    body was dead code the intrinsic had been shadowing since 6.2.11, so 2.3.3
    shipped ties-to-even while its own source, tests and `docs/sources.md` all
    described ties-away-from-zero. The rename restores the documented behaviour;
    `test_round_ties_away` now pins both modes so they cannot silently swap
    again.
  - **Call arity is enforced** — a bare `print(cstr)` compiled at 6.4.66 despite
    `lib/io.cyr` declaring `print(msg, len)`; all three fuzz harnesses used the
    1-arg form and now route through a local `_fz_puts(s)` helper.
  - `cyrlint`'s **untracked-deferral gate** (a deferral marker needs a
    `CHANGELOG`/`roadmap`/`docs/`/`issue` reference **on the same line**, or
    `#skip-lint`) was finally acted on. **Not new in 6.5.x** — the 6.3.10 and
    6.4.66 binaries report the same two deferrals against the 2.3.3 tree. It went
    unnoticed because CI matched only `^\s*warn ` lines and because
    `cyrius lint src/*.cyr` lints only the **first** path (extra args are
    silently dropped), so "lint clean" at 2.3.1–2.3.3 meant "`src/ai.cyr` has no
    *warnings*". Both holes are closed: CI now matches `deferral line` too, and
    `CLAUDE.md` documents the per-file loop.

  It also carries six fixes from the **2026-08-13 audit**
  ([`../audit/2026-08-13-audit.md`](../audit/2026-08-13-audit.md)), all
  pre-existing and unrelated to the bump, all reproduced before being fixed:
  - **`tokenize()` wrote past the token array** — `ABACO_MAX_TOKENS` was
    allocated against but never checked. Because the bump allocator places the
    per-number `nend` cells right after the array, the overflow produced
    **silently wrong answers** (a 343-term sum returned 342) well before it
    SIGSEGVed. Both `tokenize` and `implicit_mul` now bound every write.
  - **`eval_pow` had an unbounded O(exponent) loop** — `2^1000000000000000000`
    was ~81 years of work from 20 bytes. Capped at `ABACO_POW_EXACT_MAX` (1024),
    above which the O(1) path returns the identical value; now 2 ms.
  - **`ABACO_MAX_DEPTH` did not bound unary/power recursion** — `----…--1` and
    `2^2^…^1` overflowed the native stack. Both now charge depth.
  - **`totient()` had no domain cap** over an O(√n) loop; capped at 10¹².
  - **`parse_number`'s i64 accumulators wrapped** — a ~19-digit literal went
    NEGATIVE, a ~19-digit fraction wrapped its divisor. Replaced with an exact
    18-digit mantissa plus a decimal exponent, which removes the wrap point,
    improves fractional rounding, and keeps whole numbers bit-exact.
  - **`CalcHistory_from_json` discarded the whole history** if any field held a
    `{` or `}` — object boundaries were found by counting raw braces without
    skipping string contents, so `save_to_file`/`load_from_file` silently lost
    everything. Found by the new `fuzz_ai` harness on its first run. All
    structural JSON scanning is now string-aware.

  Plus: parser limits namespaced (`MAX_TOKENS` collided with stdlib
  `patra.cyr`, and duplicate globals only *warn*); `fuzz_eval` extended to
  actually reach these paths (it hangs against the 2.3.3 evaluator); and a new
  `fuzz/fuzz_ai.fcyr` covering the previously unfuzzed `src/ai.cyr` — the only
  code in the crate that parses data off the network.

  Suite green (547 asserts, was 479); fuzz **4/4** (20,000 iters — `fuzz_ai` is
  new, closing the audit's P-1 gap); fmt/lint/vet clean; DCE build passes
  (395,712 B, ~42 KB larger than 6.4.66); `dist/abaco.cyr` regenerated
  (123,434 B — **not** byte-identical to 2.3.3).

- **2.3.5** is the **fix audit** — a second adversarial pass run
  against 2.3.4's fixes rather than the bugs they replaced
  ([`../audit/2026-08-13-fix-audit.md`](../audit/2026-08-13-fix-audit.md); 74
  findings raised, 39 confirmed). It found that:
  - **Two 2.3.4 fixes introduced new silent wrong answers.** The `eval_pow` cap
    turned `(±Inf)^n` into NaN for n > 1024 (2.3.3 correctly gave ±Inf), and a
    zero significand with a large exponent made `0e400` parse as NaN. Both had
    `eval_err = NONE`, i.e. exactly the failure class 2.3.4 set out to remove.
  - **`eval_pow` is now binary exponentiation** — O(log|exp|), ≤63
    multiplications for any i64 exponent. That removes the DoS *without* a cap,
    propagates Inf/0 per IEEE, and recovers the ~493 ulps the cap was costing
    across `0.5 < |base| < 2`, where its "behaviour-preserving" claim was false.
  - **The H-4 JSON fix was incomplete**: `_jf_get_object` was never made
    string-aware, `_ccy_json_depth_ok` could FALSE ACCEPT via `]` padding, and
    `CalcHistory_load_from_file` wrote one byte past its allocation.
  - **Six regression assertions were decorative** — green with the code they
    guarded deleted. The H-3 depth test's inputs exceeded the token cap so they
    never reached the parser; the key-confusion payload escaped its own
    lookalike; `fuzz_eval` discarded every result and so passed 50,000
    iterations with all token bounds removed; `fuzz_ai`'s CRLF assertion could
    never fire; the MED-7 probe checked 1 key in 26; the deep-nesting block
    asserted nothing. All now verified to fail when their fix is reverted.

  Suite 547 → **631 asserts**; fuzz 4/4 at 20,000 iters; fmt/lint/vet clean;
  DCE build 395,712 B; `dist/abaco.cyr` 129,046 B — **not** byte-identical to
  2.3.4, consumers must re-vendor.

- **2.4.0** **closes both residuals** the fix audit left open, and
  is a minor because one of them is closed by a user-visible cap raise:
  - **Decimal literals are correctly rounded.** `parse_number` scales through an
    error-compensated (double-double) factor and splits the mantissa too — the
    mantissa mattered because 18 digits exceeds f64's exact-integer limit, so
    `f64_from(mant)` alone already cost the last ulp. `1.7976931348623157e308`
    (DBL_MAX) now parses exactly where 2.3.4/2.3.5 returned +Inf. Over a
    5000-literal corpus: **99.74% bit-exact, worst 1 ulp** (2.3.3 averaged
    ~73000 ulp). A Clinger fast path — exact mantissa, |exponent| ≤ 22, one
    provably correct rounding — plus a **cached** exact power-of-ten table makes
    this *faster* than 2.3.5 rather than slower.
  - **`ABACO_MAX_TOKENS` 512 → 1024**, so `parse_power`'s depth guard is
    reachable at last: 257 `^` operators is 515 tokens, which the old cap could
    not express. Verified discriminating — reverting the guard now fails 2
    assertions.

- **2.4.1** pins **Cyrius 6.5.21** and adopts the **tuples abaco
  proposed at 2.4.0**. `_two_product` is `: (f64, f64)`, `_dd_pow10` is
  `: (f64, f64, i64)` (the arity-3 case 6.5.21 added, and the one their
  `crossos/multi_return.tcyr` names), `parse_number` returns `(value, end)` —
  removing an `alloc(8)` per numeric literal — and `tokenize` returns
  `(count, ok)`, retiring the negative-count sentinel.
  ⛔ The proposal's premise was partly **false**, and that was abaco's error: the
  2-value form had shipped at Cyrius v3.7.2 and this crate tested the wrong
  syntax, so 2.4.0's out-parameter workarounds were never needed. **Arity 3**
  (`_dd_pow10`) was the one genuine gap, alongside declared return types and a
  destructure contract.
  ⭐ Implementing it surfaced **three silent miscompiles** in Cyrius's own
  multi-value return, shipped since v3.7.2 with three lines of integer-only
  coverage. One corrupted this crate's exact shape: a `: f64` fn returning a
  tuple lost its FIRST value on x86/PE. `test_two_product_exact` covers it here,
  order-sensitively, rather than assuming the fix.
  Parse accuracy is byte-identical to 2.4.0 — verified, not assumed.

- **2.4.2** pins **Cyrius 6.5.27**. Both issues abaco filed at
  2.4.1 are fixed upstream, and the workaround one of them forced is reverted:
  - **The typed-pointer warning tested the wrong sign** — it gated on `lt > 0`,
    but a positive local type is a width or a float tag while pointer-like is
    stored negative. Inverted in both directions at once. abaco's 5 spurious
    warnings are gone and the fresh-local dance in `_dd_mul_d` / `_mul_pow10` /
    `_div_pow10` is reverted to the natural `e = f64_add(e, …)`.
  - **`<source>` lines shifted +1 per prepended line** — abaco's 18-module
    `[deps].stdlib` made that +17, pointing past EOF on short files. Fixed with
    a `#@srcline` marker; confirmed here.
  - ⚠ **Warnings now fire inside `lib/bayan.cyr`** — the corrected check
    reaches real typed-pointer locals, and assignment does not consult the
    callee's declared return type (`str_new` is `: Str`). Vendored stdlib, not
    abaco's to fix; filed upstream with a repro. Diagnostic only.
    ✅ **Closed at 2.4.3** — and the count recorded here was wrong. A controlled
    rebuild against the 6.5.27 compiler *and* 6.5.27 stdlib produces exactly
    **3**, at `lib/bayan.cyr:1447:77`, `:1451:77`, `:1456:65`, not 7. What closed
    them is bayan 1.5.x re-declaring `_toml_parse_str` /
    `_toml_parse_multiline_q` as `: Str`, **not** the 6.5.28 compiler fix — that
    one addressed pointer-declared callees, and abaco's three were `: i64`.

  Parse accuracy byte-identical to 2.4.1 over the same corpus (13/5000 at 1 ulp),
  so `ganita`'s +146 lines moved nothing abaco relies on.
  ⛔ **Benchmarks: neither the toolchain nor this release moves them.** The run
  read ~30% faster across everything including `registry_creation`; a three-way
  A/B on one idle box (2.4.1 src on 6.5.21 and on 6.5.27, vs 2.4.2 on 6.5.27)
  came out identical within noise, so the delta is machine state. This corrects
  2.4.1's claim that the removed per-literal `alloc(8)` was visible — it is not
  at this granularity. `bench-history.csv` rows either side of this boundary are
  not comparable.

- **2.4.3** pins **Cyrius 6.5.35**, and the bump turned into a
  correctness release because auditing it surfaced two silent wrong answers that
  were abaco's own, not the toolchain's:
  - **`eval_pow` returned `+√2` for `(-2)^0.5`** and `+2` for `(-8)^(1/3)`, with
    `eval_err = NONE`, since 2.2.x. The non-integer branch computes
    `exp2(exp · log2(|base|))` and never restored the sign — and the comment on
    that very line had named the problem and pointed at a "sign correction below"
    that was never written. Now a quiet NaN per IEEE 754-2019 §9.2 / C99
    F.10.4.4, agreeing with `sqrt(-1)`. **NaN, not `ABACO_ERR_MATH`**:
    `ABACO_ERR_MATH` here guards *integer* domains and `eval_pow` holds no
    evaluator handle. The integer path is untouched, sign parity included.
  - **Every non-finite `pow` row returned NaN.** `f64_to` saturates to `i64_MIN`
    for `+∞`, `-∞` and NaN alike, so the round-trip that classifies the exponent
    never succeeds for one and they all fell through to `exp2`, which is NaN for
    every non-finite argument — the same fact behind the 2.3.4 `(±Inf)^n`
    regression, closed then for integer exponents only. The C99 table is now
    answered explicitly in row order. ⚠ Still open: an exponent ≥ 2⁶³ misses the
    `is_int` round-trip though every such f64 is even, so `(-2)^1e300` is NaN
    where C99 wants `+∞`; widening the round-trip is the integer path's business.
  - **`mod_pow` no longer traps.** bayan 1.5.2 fixes `bayan_u64_mulmod` taking
    SIGFPE on ordinary inputs (x86 `DIV` raises `#DE` when the *quotient*
    overflows, not only on a zero divisor). `src/ntheory.cyr` passes straight
    through to it, so `mod_pow(x, y, 0)` and a base ≥ 2⁶³ killed the process at
    2.4.2. `mod_pow` had **no coverage at all**; `test_mod_pow_boundaries` now
    pins it, and is discriminating in the strongest sense — the identical test
    file dies with SIGFPE before printing one assertion against the 6.5.27
    stdlib. ⚠ **The fix rides the consumer's pin, not abaco's tag** —
    `dist/abaco.cyr` bundles no bayan, so a consumer on 6.5.27 is still exposed
    against the 2.4.3 bundle. Consumers must pin ≥ 6.5.35. `is_prime` was never
    at risk (small witnesses, positive pre-reduced `n`), which is why 107 asserts
    and 20,000 fuzz iterations stayed green over a primitive that could trap.
  - ⛔ **Benchmarks: Miller–Rabin is 1.53× slower** — 100,000 `is_prime` calls go
    264.6 ms → 405.8 ms (wall clock, 5 interleaved rounds); `is_prime_small`
    1.600 → 2.454 us, `is_prime_large` 4.089 → 5.967, `next_prime` 1.947 → 2.911.
    The other 74 benchmarks are flat. This is the price of the SIGFPE fix — two
    guards and two `%` reductions per `mulmod`, twice per exponent bit — and it
    is **accepted, not worked around**: hand-rolling a reduced-operand `mulmod`
    would re-introduce in abaco the hazard upstream just removed. A
    `bayan_u64_mulmod_reduced` fast path belongs upstream; `powmod` already
    pre-reduces, so the reduction is redundant on this call path specifically.
    ⚠ Unlike 2.4.2's uniform machine-state swing, this signature is *selective* —
    only the three benchmarks reaching `bayan_u64_powmod` moved. The wall-clock
    row is the trustworthy one: `benches/bench.bcyr` subtracts a per-run timer
    floor (~1.35 us) that is a large fraction of readings this short, and a first
    A/B taken on a loaded box had to be discarded.
  - **Verified unchanged before either fix:** 74 evaluator results bit-for-bit
    identical across both pins — every transcendental, literal parsing including
    `DBL_MAX` and 18-digit mantissas, rounding, number theory, dB/MIDI. That is
    what makes the two `pow` bugs attributable to abaco's logic rather than the
    fold. Zero symbol collisions from bayan's +10,046 lines; no reserved-name
    collision this window; 6.5.28's decimal-literal fix cannot reach abaco, whose
    executable code holds zero bare decimal float literals (every f64 constant is
    a hex bit pattern — the habit the 2.3.2 dB fix established).

- **2.4.4** closes **nine of the ten** follow-ups 2.4.3's audit
  opened. A deliberate cleanup batch rather than the usual one-change-at-a-time
  release; every fix is discrimination-proven with a named, counted revert.
  - ⛔ **CI could not fail on a failing assertion.** `assert_summary()` RETURNS
    `_assert_fail`, and all seven `.tcyr` called it then `return 0;`, throwing
    that value away — so a red suite exited 0 and `ci.yml`, which gates on the
    exit code and otherwise greps only for `passed`, reported success with the
    failure text inside a collapsed `::group::`. Proven on the committed 2.4.3
    tree: one injected failing assertion gives `10 passed, 1 failed` and
    **exit 0**; the same probe on 2.4.4 gives **exit 1**. Fixed at the source in
    all seven files AND backed by an independent second mechanism in CI, which
    now parses the `N failed` count and treats a missing summary line as a
    failure. Compile errors, SIGSEGVs and timeouts always did gate.
  - **`0^(0-1)` returned `+0`.** The zero-base test ran ahead of any sign test
    on the exponent, so every `0^y` answered `+0` with `eval_err = NONE` — while
    an overflowing `2^10000` already returned `+Inf`, so it was internally
    inconsistent too. Now the full C99 F.10.4.4 table, negative-zero sign rows
    included (`(-0)^(0-3)` = `-Inf`, `(-0)^3` = `-0`). Not `ABACO_ERR_DIV_ZERO`
    despite `1/0` raising it: `eval_pow` holds no evaluator handle, per the
    2.4.3 precedent, and the asymmetry is pinned by its own assertion.
  - **Any exponent >= 2^63 answered NaN.** `f64_to` saturates, so the round-trip
    that classifies the exponent failed and 2.4.3's negative-base guard claimed
    them — yet every f64 at or above 2^53 is an even integer. `(0-2)^1e300` is
    `+Inf` again. Closes the gap 2.4.3 filed against itself.
  - **`Value_to_latex` printed the integer part of every float** — `0.25` as
    `0`, `-0.25` as `0` with the sign gone, `+/-Inf` and NaN as the literal
    `-9223372036854775808`, `1e300` as that same token. Zero tests
    and zero call sites, but exported in the bundle. Finite floats now delegate
    to `fmt_float_buf`; non-finite get LaTeX symbols; magnitudes outside
    `[1e-6, 1e16)` go to `m \times 10^{e}`. The COMPLEX branch had the same
    defect plus a sign test reading a TRUNCATED i64, so any imaginary part in
    `(-1, 0)` printed `0 + 0i`. ⭐ The mantissa renormalisation needs both bounds
    display-aware and the arms mutually exclusive — the obvious `[1, 10)` version
    cancels itself and leaves `10.000000 \times 10^{299}`.
  - **A security comment whose mechanism does not exist.** `src/ai.cyr`'s MED-4
    guard called itself a stack-exhaustion guard against a recursive
    `bayan_json_parse`. That parser is a FLAT single-pass scanner in both the
    6.5.27 and 6.5.35 stdlib — five `while` loops, no self-call. The recursive
    one is `_jp_parse_value_a`, which abaco never calls and which has capped
    itself at 128 since before 6.5.27. The guard stays (a flat fiat-rate map
    nested past 8 is not a rates object, and this is the only network-facing
    data in the crate); the rationale is rewritten in all three places it
    appeared. Same failure shape state.md already records at 2.4.1 — a
    load-bearing claim nobody re-checked.
  - **Three release gates were checking nothing:** CI's fmt loop covered
    `src/*.cyr` only, so three files sat dirty across releases (now widened, and
    all three formatted — verified whitespace-only); `release.yml`'s
    `[ -s body.md ]` could not see an unfilled 36-byte CHANGELOG stub (now
    strips headings and blank lines, and FAILS rather than substituting a
    placeholder); and `scripts/version-bump.sh` warned about `SECURITY.md` only
    on a MAJOR bump though that table is minor-granularity — which is why it
    still read `2.3.x` four releases after 2.4.0 — while self-healing the
    consumer tag in `README.md` alone, letting the guide drift.
  - Coverage: `test_pow_zero_base` (15), `test_pow_huge_integer_exponent` (14),
    `test_transcendental_coverage` (23 — the nine ganita transcendentals that
    had none, with anchors AND identities, since neither alone suffices), and
    `test_value_to_latex` (19). Suite 729 -> **813**.
  - ⭐ **An adversarial review of this release's own diff confirmed 16 findings;
    six were real defects, all fixed before tagging.** `pow(+/-0, NaN)` returned
    `+0` instead of NaN (the rewritten zero-base block returns ahead of the
    general NaN rows, and a NaN exponent passed every test inside it); an
    exponent of exactly `-(2^63)` — the one magnitude >= 2^63 that still
    round-trips through i64, so it meets the `i64_MIN` peel — had its parity
    flipped by peeling ONE, so `(0-0.5)^(0-2^63)` answered **-Inf** where C99
    answers `+Inf`; an infinite complex part emitted `\inftyi`, an undefined
    LaTeX control sequence, because TeX scans control words greedily over
    letters; a negative-zero imaginary part printed `+ -0.000000i`; the two
    smallest subnormals printed as `inf` because `f64_pow(10, e)` underflows for
    `e <= -324`; and **three of the six CI security-scan patterns had never
    matched anything**, dying with `Unmatched ( or \(` under grep's default BRE
    with the error swallowed by `2>/dev/null`. The first two are pre-existing;
    the middle three are this release's own; the last has never missed anything
    because `src/` contains only `sys_exit`.
  - ⭐ **Two more defects in this release's first cut, caught earlier in
    self-review.** The `Value_to_latex` rewrite fixed only the FLOAT branch,
    leaving COMPLEX still printing `i64_MIN` for a 1e300 part — the very bug
    being closed, left in half the function. And returning the raw failure count
    as the exit code reintroduces silent-green at EXACTLY 256 failures, an exit
    code being 8 bits; it is collapsed to 1. Also fixed in passing: the TEXT
    branch's `memcpy` was unbounded against its 128-byte buffer, pre-dating this
    release.
  - ⚠ **Not closed:** filing `bayan_u64_mulmod_reduced` upstream, which would
    recover 2.4.3's 1.53x Miller-Rabin regression. Outward-facing; left to the
    maintainer.

- **2.4.5** (this release) is **CI-only**, and exists because 2.4.4 broke CI.
  - ⛔ **The 2.4.4 security-scan hardening failed on every clean run.** To
    inspect grep's exit status it split the step's `grep | awk` pipeline into
    `hits=$(grep ...); rc=$?` — but grep exits **1 when it finds nothing**,
    which is the passing case, and GitHub runs every step under `bash -e`, so
    the assignment took that failing status as its own and the shell exited at
    the first pattern. No `FAIL:` line, no `::error::`, not even the closing
    `security scan complete` — just `Process completed with exit code 1`. The
    old form was accidentally safe: its pipeline ended in `awk`, which exits 0
    regardless. Now `rc=0; hits=$(grep ...) || rc=$?`, which `bash -e` exempts.
  - **The same hazard on the Test step**, pre-dating 2.4.4 and firing on the
    mirror case: `cyrius test` exits non-zero for a RED suite, so the step died
    at the assignment before `rc` was captured, before the per-suite `FAIL:`
    line, and before the failure-count gate 2.4.4 added could run. The job still
    went red, but silently. Both paths now verified: green exits 0, and one
    injected failure exits 1 printing BOTH gates.
  - ⭐ **The lesson, and the process change.** 2.4.4 verified the scan's *logic*
    — patterns against fixtures, in an ordinary shell — and never ran the
    *step* under the interpreter GitHub uses. `bash -e` was the entire defect.
    Every runnable step's `run:` block is now extracted from the YAML and
    executed under `/usr/bin/bash -e` against the real tree; all 12 pass. That
    is the check that would have caught this, and it is cheap.
  - `/bench-results.txt` gitignored — the `Bench (non-fatal)` step tees there in
    the repo root, dirtying a local tree and liable to ride a `git add -A`. Same
    class as the `dist/*.deps` sidecar ignored at 2.4.3.

- **2.3.x still open** (external — needs consumer repos, not actionable from
  abaco alone): wire the first real consumers (Abacus, dhvani) to
  `dist/abaco.cyr`; audit consumers for duplicated math that should use
  `abaco::dsp`; `lib/tls.cyr` for the currency cache once the stdlib TLS API
  stabilizes. See [`roadmap.md`](roadmap.md).
