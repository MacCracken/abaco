# Abaco — Roadmap

> Planned work, pinned to releases. What shipped is in
> [`../../CHANGELOG.md`](../../CHANGELOG.md); live state is in
> [`state.md`](state.md). An item leaves this file when it ships.
>
> Every minor opens with the P(-1) hardening pass and closes with the work
> loop's documentation step ([`../../CLAUDE.md`](../../CLAUDE.md)). Every new
> algorithm cites its source in [`../sources.md`](../sources.md), is tested
> against an oracle that does not call the code under test, and has a bench row
> before and after.

## 2.9.0 — evaluator: functions

- [ ] **Γ, ln Γ, erf, erfc.** None is in ganita. Port fdlibm 5.3's
      `e_lgamma_r.c` and `s_erf.c` into ganita, as `tan` went; abaco exposes
      them here. `gamma(x)` comes from ln Γ with the reflection formula, and
      `x!` for a non-integer x becomes Γ(x + 1). Today `3.5!` is
      `ABACO_ERR_MATH`. Accuracy is pinned against mpmath at 400 bits, with a
      ulp budget per function.
- [ ] **From the stdlib**: `cbrt` and `hypot` (ganita), `clamp` and `lerp`
      (math).
- [ ] **Arity by name**:
      - `log(x, b)` beside `log(x)`;
      - variadic `min`, `max`, `gcd` and `lcm` (today `min(1, 2, 3)` is
        `ABACO_ERR_ARITY`);
      - `perm(n, k)`, exact like `abaco_binomial`;
      - sample `svar` and `sstdev` beside the population `stddev`.
      `_fn_arity` becomes a range.
- [ ] **`mod(a, b)`**: floored modulo as a function. `%` reads as percent
      when a sign follows it, so `7 % -3` is 7 % − 3 = −2.93. `mod(7, -3)` is
      −2, unambiguously. The README documents the `%` rule.

## 2.10.0 — evaluator: literals and operators

- [ ] **Integer literals** in base 16, 8 and 2 (`0x1F`, `0o17`, `0b1011`),
      and `_` digit separators (`1_000_000`). Exact up to 2^53, and
      `ABACO_ERR_MATH` past it. Today `0x10` reads as 0 · `x10` and fails
      with `ABACO_ERR_UNKNOWN_VAR`.
- [ ] **Comparison and logic**: `<`, `<=`, `>`, `>=`, `==`, `!=`, `&&`, `||`
      and `not`, giving 1 or 0, plus `if(c, a, b)`. The tokenizer must tell
      `!=` from the postfix `!`, and the precedence table goes in
      `docs/architecture/`. Today `2 < 3` is `ABACO_ERR_PARSE`.
- [ ] **Bitwise on exact integers**: `&`, `|`, `<<`, `>>` and `xor(a, b)`
      (`^` stays power). Refused for a non-integer or for |x| ≥ 2^53.
- [ ] **Fuzz**: `fuzz_eval`'s grammar gains every new token, and asserts
      that precedence matches a reference parser written in the harness.

## 2.11.0 — evaluator: user-defined functions

- [ ] **`Evaluator_define(e, "f(x, y) = x^2 + y")`** stores the token span
      and parameter names on the evaluator, and `f(3, 4)` then calls it like
      a built-in. Parameters shadow variables. Recursion and nesting count
      against `ABACO_MAX_DEPTH`. Definitions are per evaluator, with no
      globals, so one evaluator per thread stays safe. A built-in name cannot
      be redefined.
- [ ] **Fuzz**: random definitions and calls. A self-recursive definition
      ends in a depth error, never a stack overflow.

## 2.12.0 — units: SI prefixes and new quantities

- [ ] **Any SI prefix on any SI unit**, from quecto to quetta (2022 SI),
      parsed as prefix + registered symbol only when the whole string is not
      a registered unit, so `min`, `Pa` and `mi` keep their meanings. IEC
      binary prefixes apply to bit and byte only. Sources: BIPM SI Brochure,
      9th ed. (2019), and CGPM 27 (2022), Resolution 3.
- [ ] **New categories**, with exact factors from NIST SP 811 (2008),
      Appendix B:
      - electric potential, current, resistance, charge (C, A·h, mA·h),
        capacitance and inductance;
      - magnetic flux density (T, G);
      - acceleration (m/s², g₀, Gal);
      - torque (N·m, lbf·ft);
      - volumetric flow (m³/s, L/min, gal/min);
      - amount of substance (mol);
      - radioactivity (Bq, Ci) and dose (Gy, rad, Sv, rem).
- [ ] **Fuzz**: `fuzz_units` composes random prefix + symbol strings and
      checks the factor against 10^k exactly.

## 2.13.0 — units: dimensional analysis

- [ ] **A unit as a factor plus an exponent vector** over the seven SI base
      dimensions. Compound units then parse and convert: `km/h`, `N·m` /
      `N*m`, `kg/m^3`, `m/s^2`, `W/(m^2·K)`.
      - Any two dimensionally equal units convert.
      - A dimension mismatch is `UERR_CONVERT`.
      - Affine units (°C, °F) convert only on their own.
- [ ] **Units inside expressions**: `5 km + 300 m`, `3 kWh / 2 h`,
      `60 mi/h to m/s`. Evaluator values carry dimensions, and `+` / `-` on
      unequal dimensions is an error.

## 2.14.0 — currency fetch

- [ ] **`abaco_tls_init()` from any thread, and a fetch that warms on first
      use.** sigil 3.13.6 made every lazy init race-free, and
      `tls_init_main()` is safe from any thread, so the main-thread rule no
      longer guards a crash. It needs a cold-process test of concurrent first
      fetches with no init. The libssl refusal stays.
- [ ] **IPv6 hosts**: today `_ccy_https_split` refuses `[`, and
      `net_resolve_ipv4` is IPv4-only.
- [ ] **TLS 1.2-only servers**: offer 1.3, then fall back to 1.2. Today
      such a server is `AI_ERR_TLS`.

## Upstream asks

- **lib/net.cyr: a name lookup that takes a deadline.** `net_resolve_ipv4`
  gives up only after 2 tries of up to 8 replies, 2 s each (~32 s). From 2.6.0
  abaco's HTTPS deadline counts the lookup, but cannot cut it short.
- **ganita: `ldexp` / `frexp` (or `scalbn` / `ilogb`).** abaco carries two
  private copies of exact power-of-two scaling and exponent extraction:
  `_pow_scale` / `_frexp1` in `eval` (mean, stddev, literals) and
  `_win_ldexp` / `_win_max_exp` in `dsp` (window metrics, 2.8.0). When ganita
  ships them, abaco switches and drops both.

**bayan.** Filed in bayan's `docs/development/issues/`. When a Cyrius release
ships one, abaco re-measures:
- `bayan_u64_powmod` shifts its exponent with a call on every bit, ~28 %
  slower since bayan 1.5.10;
- a `bayan_u64_mulmod_reduced` fast path would recover Miller–Rabin's 1.53×;
- a way to include bayan's JSON and u128 without its ~318 KB of Unicode
  tables.
