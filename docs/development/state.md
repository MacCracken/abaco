# Abaco — State Snapshot

> Volatile state, refreshed every release. Durable rules live in
> [`../../CLAUDE.md`](../../CLAUDE.md); forward plan in [`roadmap.md`](roadmap.md);
> per-tag history in [`../../CHANGELOG.md`](../../CHANGELOG.md).

**Last updated:** 2026-08-13 (2.4.0 — closes both residuals the 2.3.5 fix audit left open. Decimal literals are now correctly rounded via error-compensated double-double scaling (DBL_MAX parses exactly; 99.74% of a 5000-literal corpus bit-exact, worst 1 ulp), and the token cap rose 512 → 1024, which makes `parse_power`'s depth guard reachable and testable for the first time)

## Versions

| What | Value |
|------|-------|
| abaco | **2.4.0** |
| Cyrius toolchain pin | **6.5.20** |
| License | GPL-3.0-only |

## Stdlib (6.2.x batching — unchanged through 6.5.x)

The 6.2.x stdlib re-batched several modules; abaco's `[deps].stdlib` list
adjusted accordingly at 2.2.5. **6.3.x (2.3.1), 6.4.x (2.3.3) and 6.5.x (2.3.4)
keep the same layout** — no re-batch, so the dependency list is unchanged and all
18 declared modules still exist; the newer bundles only grew internally (at
6.5.20: `bayan` +774 lines, `syscalls_x86_64_agnos` +442, `alloc` +241, `io`
+203, `vec` +187, `bench` +185, `syscalls_aarch64_linux` +138,
`syscalls_linux_common` +92, `syscalls_macos` +65, `fmt` +61, `syscalls` +51,
`syscalls_windows` +43, `atomic` +21, `ganita` +20, `syscalls_x86_64_linux` +18,
`math` +8, `assert` +6, `string` +4 vs 6.4.66; `alloc_*`, `args_*`, `hashmap`,
`http`, `net`, `str` byte-identical). As of 2.3.0 abaco calls the **canonical
`bayan_*` API directly** — no longer the deprecated back-compat aliases:

| Was (6.0.x) | Now (6.2.x) | Canonical symbols abaco uses |
|-------------|-------------|------------------------------|
| `json`, `u128` | `bayan` (batched bundle) | `bayan_json_{parse,key,value}`, `bayan_u64_powmod` |
| extended `math` | `ganita` (math bundle) | `f64_{a,}sinh/cosh/tanh`, `f64_asin/acos`, `f64_atan2`, `fibonacci`, `binomial` |
| `math` (slim) | `math` | basics: `f64_sin/cos/sqrt/log`, `f64_pow`, constants |

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
| `build/abaco` | ~400 KB | DCE smoke binary (`src/main.cyr`) — x86_64 ELF (399,880 B at 2.4.0/6.5.20; was 353,408 B at 2.3.3/6.4.66, 357,736 B at 2.3.1/6.3.10). The 6.5.x stdlib both NOPs more (1216 unreachable fns, 321,508 B) and leaves ~38 KB more resident |
| `dist/abaco.cyr` | ~136 KB (~3.66k lines) | Committed consumer bundle. 6.2.x distlib is profile-based: `cyrius distlib abaco` → `dist/abaco-abaco.cyr`, renamed to `dist/abaco.cyr`. 2.4.0: 135,736 B / 3,661 lines — carries the `f64_round_half_away` rename, the evaluator bound-check fixes and string-aware JSON scanning; **not** byte-identical to 2.3.5 (consumers must re-vendor, and a consumer calling the bundle's old `f64_round` now silently gets the ties-to-even builtin) |

## Tests

**643 asserts, 0 failures** across 7 `.tcyr` files:

| Suite | Asserts |
|-------|---------|
| `test_ai` | 118 |
| `test_dsp` | 109 |
| `test_eval` | 238 |
| `test_integration` | 27 |
| `test_ntheory` | 107 |
| `test_simd` | 10 |
| `test_units` | 34 |

- Fuzz harnesses: **4** (`fuzz/fuzz_{eval,ntheory,units,ai}.fcyr`) — clean at 20,000 iters
- Benchmarks: 3 (`bench`, `bench_eval`, `bench_units`)
- fmt / lint / vet: clean

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

- **2.4.0** (this release) **closes both residuals** the fix audit left open, and
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

- **2.3.x still open** (external — needs consumer repos, not actionable from
  abaco alone): wire the first real consumers (Abacus, dhvani) to
  `dist/abaco.cyr`; audit consumers for duplicated math that should use
  `abaco::dsp`; `lib/tls.cyr` for the currency cache once the stdlib TLS API
  stabilizes. See [`roadmap.md`](roadmap.md).
