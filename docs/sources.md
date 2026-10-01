# Abaco — Sources

Academic and domain citations for every algorithm, formula, and constant in
abaco. Required for a math crate: a reviewer should be able to trace any
result back to its origin and verify the implementation against the published
source. No magic numbers.

> **Completeness audit: done 2026-05-26 (2.2.2).** Every algorithm, formula, and
> constant in `src/` was cross-checked against a citation below. The audit added
> the synthesis/envelope entries (PolyBLEP, constant-power pan, time constant).

## Number theory — `src/ntheory.cyr`

- **Deterministic Miller–Rabin primality** with witness set
  {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37}. Proven correct for all
  *n* < ψ₁₂ = 318665857834031151167461 ≈ 3.187 × 10²³ (OEIS A014233), the
  smallest strong pseudoprime to all twelve bases; that covers the entire i64
  range (< 9.23 × 10¹⁸) exactly, not probabilistically. Through 2.4.8 these
  docs gave 3.317 × 10²⁴, which is ψ₁₃: it needs witness 41 as well.
  - Jaeschke, G. (1993). "On strong pseudoprimes to several bases."
    *Mathematics of Computation*, 61(204), 915–926. doi:10.1090/S0025-5718-1993-1192971-8
  - Sorenson, J. & Webster, J. (2017). "Strong pseudoprimes to twelve prime
    bases." *Mathematics of Computation*, 86(304), 985–1003. doi:10.1090/mcom/3134
    (ψ₁₂ and ψ₁₃).
  - OEIS A014233, "Smallest odd number for which Miller-Rabin primality test
    on bases <= n-th prime does not reveal compositeness."
  - Avoids the random-witness weakness of Albrecht et al. (2018),
    "Prime and Prejudice: Primality Testing Under Adversarial Conditions"
    (ACM CCS 2018) — abaco's witness set is fixed and exact.
- **Modular exponentiation / multiplication** — right-to-left binary method,
  delegated to stdlib `bayan_u64_powmod` / `bayan_u64_mulmod`. (The bare
  `u64_powmod` / `u64_mulmod` spellings named here through 2.4.3 are the
  pre-6.2.x `lib/u128.cyr` names; they survive only as back-compat aliases, and
  abaco has called the canonical `bayan_*` symbols since 2.3.0.)
  - Knuth, D. E. *The Art of Computer Programming, Vol. 2: Seminumerical
    Algorithms* (3rd ed.), §4.6.3.
- **Euler's totient φ(n)** via prime-factor product form.
  - Hardy, G. H. & Wright, E. M. *An Introduction to the Theory of Numbers*
    (6th ed.), Theorem 62.
- **Trial-division factorization** — classical; trial divisors 2 then odd
  *d* up to ⌊√n⌋, computed once by `_isqrt` (an f64 estimate corrected by at
  most a step each way, never squaring past ⌊√(2⁶³−1)⌋ = 3037000499) and again
  only when n shrinks. Through 2.4.8 the test was `d·d ≤ n`, which wraps once
  d passes 3037000499, so factor and totient of a prime near 2⁶³ never returned.
- **Binomial coefficient (`abaco_binomial`)** — the multiplicative recurrence
  C(n, i+1) = C(n, i) · (n − i) / (i + 1) after folding k to min(k, n − k),
  with the division done first: with g = gcd(C(n, i), i + 1), the cofactor
  (i + 1)/g is coprime to C(n, i)/g and so divides n − i, making both
  quotients exact and the only product formed C(n, i+1) itself. Overflow is
  tested on that product, so the result is exact iff it fits in i64. ganita's
  `binomial`, used through 2.4.8, forms C(n, i) · (n − i) first and refuses
  any result above i64_MAX / k (C(62, 31) onward).
  - Knuth, D. E. *TAOCP Vol. 1* (3rd ed.), §1.2.6 (the factorial form, the
    symmetry C(n, k) = C(n, n − k), and the step from C(n, k) to C(n, k+1)).
- **`mod_pow` with a negative exponent** — the exponent is the u64 bit pattern
  e (as `bayan_u64_powmod`'s contract reads it): x^e = x^(e mod 2⁶³) · (x^(2⁶²))²
  mod m, so the top bit is applied as two squarings rather than dropped.

## DSP — `src/dsp.cyr`

- **12-tone equal temperament (12-TET), MIDI ↔ frequency.**
  `freq = 440 · 2^((m−69)/12)`. A4 = 440 Hz (ISO 16:1975); MIDI note 69 = A4.
- **C0 reference** = 440 / 2^(57/12) = 16.351597831287414 Hz (MIDI note 12;
  `DSP_C0_FREQ` = `0x4030_5A02_50C2_B956`, the correctly rounded double), used
  for pitch-class / octave computation: `round(12 · log2(freq / C0))`. Through
  2.4.7 the constant encoded 16.703125 Hz instead — see CHANGELOG 2.4.8.
  - `round` here is **ties away from zero** (`f64_round_half_away`), not the
    IEEE-754 round-half-to-even that the Cyrius 6.5.x `f64_round` builtin
    implements. Same rule as the expression evaluator's user-facing `round()`;
    the two modes are pinned apart by `test_round_ties_away` (2.3.4).
    - IEEE 754-2019 §4.3.1 (roundTiesToEven, the default) vs §4.3.3
      (roundTiesToAway, an explicitly sanctioned attribute).
- **Decibel** — `20 · log10(amplitude ratio)` (field/amplitude quantity);
  dBFS uses a 1.0 full-scale reference (float-audio norm). Computed as
  `(20 / ln 10) · ln(a)`; `DB_SCALE`, `DB_EXP` (ln 10 / 20) and `DB_GAIN_EXP`
  (ln 10 / 40) are the correctly rounded doubles of those quotients (2.4.9;
  they were each an ulp or more off), checked bit-for-bit in
  `test_db_constants_exact` against a 50-digit computation.
  - IEC 60027-3:2002, *Letter symbols ... Logarithmic and related quantities*
    (the field-quantity level, 20 log10).
- **RMS of two values** — `sqrt((a² + b²) / 2)` evaluated as
  `hypot(a, b) · (1/√2)`, so neither square overflows or underflows (the
  direct form lost everything below ~1e-154 and overflowed above ~1e154).
  - Blue, J. L. (1978). "A Portable Fortran Program to Find the Euclidean
    Norm of a Vector." *ACM TOMS* 4(1), 15–23 (the scaled-norm technique
    `hypot` implements).
- **Compensated summation (`batch_sum`)** — Neumaier's improvement of Kahan
  summation: the running compensation also catches the case where the new
  term is larger than the sum, so the error is O(ε) independent of n for
  well-conditioned sums (plain left-to-right summation is O(nε)).
  - Kahan, W. (1965). "Further remarks on reducing truncation errors."
    *Comm. ACM* 8(1), 40. doi:10.1145/363707.363723
  - Neumaier, A. (1974). "Rundungsfehleranalyse einiger Verfahren zur
    Summation endlicher Summen." *ZAMM* 54(1), 39–51. doi:10.1002/zamm.19740540106
- **Window functions.** Coefficients per the standard survey:
  - Harris, F. J. (1978). "On the Use of Windows for Harmonic Analysis with
    the Discrete Fourier Transform." *Proc. IEEE*, 66(1), 51–83. doi:10.1109/PROC.1978.10837
  - Hann / Hamming (a₀ = 0.54, a₁ = 0.46); Blackman (0.42, 0.5, 0.08);
    Kaiser window via the zeroth-order modified Bessel function I₀(β).
  - abaco's windows are the **symmetric** form (denominator size − 1), the
    filter-design convention. Since 2.4.10 they are evaluated on the centred
    phase φ = π(2n − (N−1))/(N−1) (cos θ = −cos φ for θ = 2πn/(N−1)), which is
    exactly negated at N−1−n, so w(n) and w(N−1−n) are the same double and an
    FIR built from them is exactly linear-phase. For spectral analysis with overlap-add, Harris
    tabulates the **periodic** (DFT-even) form: evaluate with size + 1 and
    drop the last sample. An index outside [0, size) is 0 and a 1-point
    window is 1 (2.4.9; out-of-range indices used to wrap onto the cosine).
  - I₀ is summed from its power series until a term no longer changes the
    sum (capped at 2000 terms). Through 2.4.8 it stopped at a fixed 29 terms;
    the terms grow until k ≈ x/2, so I₀ and every Kaiser window were wrong for
    β above about 20.
    - Abramowitz, M. & Stegun, I. A. (1964). *Handbook of Mathematical
      Functions*, eq. 9.6.12 (the series for I₀).
  - Kaiser, J. F. (1974). "Nonrecursive digital filter design using the
    I₀-sinh window function." *Proc. IEEE ISCAS*, 20–23.
- **Windowed-sinc interpolation kernel.**
  - Shannon, C. E. (1949). "Communication in the Presence of Noise."
    *Proc. IRE*, 37(1), 10–21 (Whittaker–Shannon interpolation).
- **Cubic (Catmull–Rom) interpolation** between control points b and c using
  neighbors a, d.
  - Catmull, E. & Rom, R. (1974). "A class of local interpolating splines."
    In *Computer Aided Geometric Design*, 317–326. doi:10.1016/B978-0-12-079050-0.50020-5
- **Samples ↔ milliseconds** — sample-rate-aware: `ms = 1000 · n / fs`.

## DSP — synthesis & envelopes (`src/dsp.cyr`)

- **PolyBLEP (polynomial band-limited step)** — anti-aliasing correction for
  oscillator discontinuities (`poly_blep(t, dt)`).
  - Välimäki, V. & Huovilainen, A. (2007). "Antialiasing Oscillators in
    Subtractive Synthesis." *IEEE Signal Processing Magazine*, 24(2), 116–125.
    doi:10.1109/MSP.2007.323276
- **Constant-power pan / equal-power crossfade** — the −3 dB sine/cosine law:
  `gain_L = cos(θ)`, `gain_R = sin(θ)` with `θ = (pan+1)·π/4`, preserving total
  power across the sweep (`constant_power_pan`, `equal_power_crossfade`).
  - Standard mixing-console pan law; see Roads, C. *The Computer Music Tutorial*
    (1996), ch. on spatialization.
- **Exponential envelope time constant** — `coeff = exp(-1 / (τ · fs))`, the
  one-pole smoothing coefficient for a given time constant τ (`time_constant`).
  - Standard one-pole/RC analogue: `y[n] = coeff·y[n-1] + (1-coeff)·x[n]`.
- **Modified Bessel function I₀** — `_bessel_i0`, used to weight the Kaiser
  window (cited under Window functions above; Kaiser 1974).

## Units — `src/units.cyr`

- **SI definitions and exact conversion factors.** Every factor is built as one
  ratio of two integers that are exact in f64, so it is the correctly rounded
  value of the exact definition; `test_units::test_exact_factors` checks all
  of them bit-for-bit. Through 2.4.8 the US/imperial factors were 6-digit
  truncations (lb = 0.453592, so 1 cup = 15.999946 tbsp and torr = mmHg).
  - BIPM. *The International System of Units (SI)*, 9th ed. (2019).
  - NIST Special Publication 811 (2008), "Guide for the Use of the
    International System of Units (SI)", Appendix B — e.g. 1 in = 0.0254 m,
    1 mile = 1609.344 m, 1 nautical mile = 1852 m.
  - International yard and pound (1959; *Federal Register* 24, 5348): 1 yd =
    0.9144 m, **1 lb = 0.45359237 kg**. oz = lb/16, stone = 14 lb,
    ft² = 0.09290304 m², acre = 43560 ft².
  - US liquid gallon = 231 in³ = **3.785411784 L**; qt, pt, cup, tbsp, tsp =
    gal/4, /8, /16, /256, /768 (the US customary cup, 8 fl oz).
  - Standard gravity g₀ = 9.80665 m/s² (3rd CGPM, 1901): kgf = 9.80665 N,
    lbf = lb · g₀ = 4.4482216152605 N, psi = lbf / in².
  - Pressure: atm = 101325 Pa (10th CGPM, 1954); torr = atm / 760;
    mmHg = 133.322387415 Pa, the conventional 13.5951 g/cm³ · g₀ · 1 mm (NIST
    SP 811 App. B). Different units: they differ in the seventh digit.
  - Energy: thermochemical calorie = 4.184 J; British thermal unit (IT) =
    1055.05585262 J (NIST SP 811 App. B).
  - Power: horsepower is mechanical, 550 ft·lbf/s = 745.69987158227022 W.
  - Time: year is the Julian year, 365.25 d = 31557600 s (IAU).
  - Speed: knot = 1852 m/h; mph = 0.44704 m/s.
  - Mass: `ton` is the **metric tonne** (1000 kg). The US short ton
    (2000 lb = 907.18474 kg) and UK long ton (2240 lb = 1016.0469088 kg) are
    registered by name (`short_ton`, `long_ton`; 2.4.10).
  - Temperature: nothing is below absolute zero — 0 K = −273.15 °C =
    −459.67 °F (SI Brochure §2.3.1, the kelvin). A conversion from below it is
    `UERR_CONVERT` (2.4.10), with a 1e-9 K margin for rounding.
- **Symbol case.** SI prefix and unit symbols are case-sensitive (m milli vs M
  mega; mHz vs MHz; Mm vs mm), and b is the bit where B is the byte, so a
  symbol matches only as written. Names, aliases and non-SI abbreviations
  (mi, lb, gal, psi, …) match in any case, and a fixed list of lowercase
  spellings of SI symbols ("hz", "ml", "kw") matches exactly as written.
  - BIPM SI Brochure, 9th ed., §3 and Table 7 (the prefix symbols, which
    differ by case: m/M, p/P, …) and §5.2 (unit symbols are case-sensitive).
  - IEEE 1541-2002 and IEC 80000-13:2008 (b for bit, B for byte; Ki, Mi, …).
- **Pitch units (semitone / cent / octave).** A cent is 1/1200 of an octave:
  `cents = 1200 · log2(f₂/f₁)`.
  - Ellis, A. J. (1885). Appendix XX to Helmholtz, *On the Sensations of Tone*.
- **BPM ↔ Hz** — `Hz = BPM / 60`.

## Expression evaluation — `src/eval.cyr`

- **Recursive-descent parsing** with operator precedence (standard technique).
  - Aho, Lam, Sethi, Ullman. *Compilers: Principles, Techniques, and Tools*
    (2nd ed.), §4.4 (predictive parsing).
- **Parser depth bound (`ABACO_MAX_DEPTH`)** — guards against stack-exhaustion DoS
  from deeply nested input, in the spirit of the SandboxJS recursion-limit
  class of fixes. Documented inline in `eval.cyr`.
- **Single rounding of literals (2.4.10).** A literal with up to 36 significant
  digits is held as a double-double mantissa (two 18-digit halves), scaled by a
  double-double 10^k, and rounded once: a normal result at 53 bits, a
  subnormal one straight to the subnormal grid (hi rounded to the grid in one
  multiply, then nudged by the sign of its residual plus lo). Digits past the
  36th set a sticky bit (counted as half a unit of the last kept digit). An
  exact tie written out past 36 digits needs arbitrary precision to decide.
  - Clinger 1990 (above) — the AlgorithmR / bignum comparison abaco does not
    implement.
- **Literal grammar.** Digits with at most one '.', and an exponent only when
  a digit follows the e/E (and its optional sign): a second '.' ends the
  literal, so `1.2.3` is a parse error rather than 1.23, and `2e` is 2 times
  the constant e. A byte the tokenizer does not recognise is a parse error
  (through 2.4.8 it was skipped, so a pasted `−5+3` with U+2212 was 8).
  The NL layer's `_nl_parse_f64` checks the same grammar and takes its value
  from `parse_number`, so a literal means the same in a sentence as in an
  expression.
- **IEEE 754 number parsing** with scientific notation. Significant digits go
  into an exact 18-digit i64 mantissa (the largest count that cannot overflow on
  `mant * 10 + digit`); everything else moves a decimal exponent, so no
  accumulator can wrap at any input length. The literal's own decimal exponent
  and the written exponent are combined and *then* clamped — clamping them
  separately lets a long fraction and a large exponent cancel into the wrong
  magnitude. The clamp is +340 above (10³⁰⁹ is already +Inf) and
  −(340 + 18) below, the mantissa headroom that keeps genuine subnormals from
  being rounded up to nonzero. Adversarial `1e999…` inputs terminate in bounded
  work.
  - IEEE 754-2019, *Standard for Floating-Point Arithmetic*.
- **Integer exponents via binary exponentiation** — `eval_pow` resolves whole
  exponents by square-and-multiply, O(log |exp|): at most 63 steps for any i64
  exponent, so `2^1000000000000000000` is constant work with no cap needed.
  Since 2.4.9 (`_pow_int_dd`) the running product is a double-double pair
  (Dekker's two-product, below) with a separate binary exponent, renormalised
  to [1, 2) at every step so no intermediate overflows or underflows; a
  negative exponent takes one Newton step on the pair's reciprocal; the result
  is scaled by 2^e once at the end. Against a 50-digit reference over 2990
  cases (including 10^k for k in −323..308) every normal result was correctly
  rounded and one subnormal was 1 ulp off. Plain f64 squaring, used through
  2.4.8, doubles its relative error at each step: up to 2.7 × 10⁹ ulp
  (1.0000000001^2⁴⁰), `10^k` disagreed with the literal `1ek`, and a negative
  power that is a representable subnormal came back 0.
  - Knuth, D. E. *The Art of Computer Programming, Vol. 2* (3rd ed.), §4.6.3
    (evaluation of powers; right-to-left binary method).
  - 2.3.4 instead capped a linear loop at 1024 and handed the tail to
    `exp2(exp · log2|base|)`. That was wrong twice over: the band
    0.5 < |base| < 2 is still changing well past 1024 (so the cap cost up to
    ~493 ulps in a function whose purpose is exactness), and `exp2(log2(+Inf))`
    is NaN, so `(±Inf)^n` silently stopped being ±Inf. See
    `docs/audit/2026-08-13-fix-audit.md` R-1 / P-1.
- **`pow` domain — negative base, non-integer exponent.** `eval_pow`'s
  non-integer branch evaluates `exp2(exp · log2(|base|))`; `log2` requires a
  positive argument, so the magnitude is taken. `x^y` for real `x < 0` and
  non-integer `y` has **no real value** — the principal complex root
  `|x|^y · e^(iπy)` is off the real line for every non-integer `y` — so
  returning the magnitude's root is a wrong answer, not an approximate one.
  The branch returns a quiet NaN.
  - IEEE 754-2019, *Standard for Floating-Point Arithmetic*, §9.2 (`pow`:
    qNaN for a negative base with a finite non-integer exponent).
  - ISO/IEC 9899:1999 (C99), Annex F.10.4.4 — the same domain rule. Its
    non-finite rows are a separate matter and are cited below.
  - Chosen over an `ABACO_ERR_MATH` evaluator error for internal consistency:
    `sqrt(-1)` already yields NaN, and `pow(x, 0.5)` is the same operation.
    `ABACO_ERR_MATH` in `src/eval.cyr` guards integer domains only.
  - Through 2.4.2 the guard was missing: `(-2)^0.5` returned `+√2` and
    `(-8)^(1/3)` returned `+2`, both with `eval_err = NONE`.
- **`pow` non-finite operands — the C99 special-value table.** A non-finite
  operand is *not* a domain error: `pow` is defined on the extended reals by
  continuity, and the answers are finite infinities and zeros, not NaN. The
  non-integer branch answers the table explicitly before it reaches
  `exp2(exp · log2(|base|))`, because that expression cannot produce them —
  `f64_exp2` returns NaN for every non-finite argument in this toolchain, and
  `log2(+∞) = +∞` feeds it one. Nor can these operands reach the exact integer
  path, where square-and-multiply would propagate ∞ and 0 correctly: `f64_to`
  saturates to `i64_MIN` for `+∞`, `-∞` and NaN alike, so the
  `f64_from(f64_to(y))` round-trip that classifies the exponent never succeeds
  for one.
  - ISO/IEC 9899:1999 (C99), Annex F.10.4.4 — the normative table.
    `pow(x, ±∞)` splits on |x| against 1: `+∞` when the exponent's sign agrees
    with |x| > 1, `+0` when it does not. `pow(±∞, y)` splits on the sign of y.
    `pow(-1, ±∞) = 1` and `pow(+1, y) = 1` for every y, NaN included, are
    listed ahead of the NaN-propagation row, as is `pow(x, ±0) = 1`.
  - IEEE 754-2019 §9.2 — `pow` is one of the recommended operations; the same
    special values, with `pow(-1, ±∞) = 1` justified as a limit of magnitude:
    the modulus is 1 for every exponent along the way, so it converges even
    though the sign does not.
  - Why the `pow(-∞, y)` odd-integer rows (`-∞` for odd y > 0, `-0` for odd
    y < 0) need no code in that branch: an odd integer has magnitude below
    2⁵³, since every f64 at or above 2⁵³ has an ulp of 2 or more and is
    therefore even. 2⁵³ is far inside i64, so an odd-integer exponent always
    survives the round-trip and takes the integer path, where the sign follows
    from the multiplication chain. What reaches the non-integer branch is a
    non-integer or an even integer too large to round-trip — hence
    `(-∞)^1e300 = +∞`, on the even row.
  - Goldberg, D. (1991). "What Every Computer Scientist Should Know About
    Floating-Point Arithmetic," *ACM Computing Surveys* 23(1), §"Infinity" —
    on why closing these cases with infinities rather than NaN is what makes
    ∞ arithmetic useful rather than merely defined.
  - Through 2.4.2 all six of `(-∞)^±2.5`, `(-2)^±∞` and `(-0.5)^±∞` returned
    NaN with `eval_err = NONE`, as did `1^±∞` and `1^NaN`. Same underlying
    `f64_exp2` fact as the 2.3.4 `(±∞)^n` regression above, which binary
    exponentiation closed for integer exponents only.
- **Decimal scaling — error-compensated (double-double).** The scale factor
  10^k is carried as a two-term (hi, lo) pair giving ~106 bits of significand,
  built with Dekker's error-compensated product; the 18-digit mantissa is split
  the same way, since it exceeds f64's exact-integer limit of 2⁵³ and would
  otherwise spend the last ulp before scaling even begins. Measured over a
  5000-literal corpus: 99.74% bit-exact, worst case 1 ulp.
  - Dekker, T. J. (1971). "A floating-point technique for extending the
    available precision." *Numerische Mathematik*, 18(3), 224–242.
    doi:10.1007/BF01397083 (the splitting and two-product algorithms).
    Implemented as `_two_product(a, b): (f64, f64)` — a genuine two-valued
    function since 2.4.1, on the Cyrius 6.5.21 tuples abaco proposed for it.
  - Clinger, W. D. (1990). "How to read floating point numbers accurately."
    *ACM SIGPLAN PLDI*, 92–101. doi:10.1145/93542.93557 — source of the fast
    path: when the mantissa is below 2⁵³ and |exponent| ≤ 22 both operands are
    exact in f64, so a single multiplication is provably correctly rounded and
    no compensation is needed.
  - IEEE 754-2019 §3.3 (the exactly-representable decimal range).
  - Reaching 100% correctly-rounded requires a bignum fallback
    (Clinger's slow path, or Eisel–Lemire); not implemented — the residual is
    bounded at 1 ulp and occurs only at the rounding boundary.
- **tan (`_eval_tan`, 2.4.10)** — fdlibm 5.3 `s_tan.c` + `k_tan.c` (FreeBSD
  msun): Cody–Waite / Payne–Hanek reduction by the stdlib's own fdlibm port
  `_f64_rem_pio2` (`lib/math.cyr`), then a 13-term odd minimax polynomial on
  [−π/4, π/4], folding |x| ≥ 0.6744 onto π/4 − x, and −1/tan for odd quadrants
  with the division error compensated. Measured: ≤ 1 ulp, correctly rounded at
  97.6% of 30,008 points against mpmath (sin/cos division: ≤ 2 ulp, 69.7%).
  - Sun Microsystems fdlibm 5.3 (1993), `k_tan.c` (T[0..12], pio4, pio4lo).
- **n! (`_eval_factorial`, 2.4.10)** — exact i64 product to 20!, then a
  double-double product (Dekker 1971) over chunks of consecutive factors whose
  product stays below 2^53, rounded once; every n ≤ 170 matches the exact value.
- **mean (`_eval_mean`, 2.4.10)** — arguments scaled by the power of two that
  brings max |x| into [1, 2) (exact), Neumaier-compensated sum, then one
  division of the renormalised pair corrected by the remainder (Dekker
  two-product). Correctly rounded in 3,000 of 3,000 random cases against exact
  rationals. **stddev / stdev** is the population form √(Σ(x − x̄)²/n) on the
  same scaled arguments (no overflow or underflow of the deviations or squares).
  - Higham, N. J. (2002). *Accuracy and Stability of Numerical Algorithms*,
    2nd ed., SIAM, §1.9 (two-pass variance) and ch. 4 (summation).
- **log10 of an exact power of ten (`_eval_log10`)** — `ln(x) / ln(10)`
  carries three roundings, so `log(1000)` was 2.9999999999999996 through 2.4.8.
  When x is exactly the double that the literal `1ek` reads as (−323 ≤ k ≤
  308, compared with the correctly rounded `_pow_int_dd(10, k)`), the answer
  is k; elsewhere the quotient stands, within ~2 ulp.
  - fdlibm `e_log10.c` (Sun Microsystems, 1993) guarantees log10(10ᴺ) = N
    for N = 0..22, the exactly representable powers; abaco extends the exact
    answer to every k whose literal `1ek` is a double.
- **Floored modulo, exact (`_eval_mod`)** — `%` follows `a − ⌊a/b⌋·b`
  (the result takes b's sign) but is computed by binary long division: each
  step subtracts the largest |b|·2^k ≤ r, and that subtraction is exact by
  Sterbenz's lemma, so the remainder is exact for every finite pair. Through
  2.4.8 the formula itself was evaluated, so `2^60 % 10` was 0 (exact: 6). A
  finite `a % ±∞` is the floored limit: a when a is 0 or shares b's sign,
  else b (Python agrees; it was NaN).
  - Sterbenz, P. H. (1974). *Floating-Point Computation*, Prentice-Hall
    (Sterbenz's lemma: y/2 ≤ x ≤ 2y ⇒ x − y is exact). Here t = |b|·2^k is
    the largest such multiple ≤ r, so r/2 < t ≤ r.
  - Knuth, *TAOCP Vol. 1* (3rd ed.), §1.2.4 (x mod y = x − y⌊x/y⌋).
- **Integer arguments (`_int_arg`)** — the evaluator's integer functions
  (`factorial`/`!`, `isprime`, `nextprime`, `prevprime`, `totient`, `fib`,
  `gcd`, `lcm`, `binomial`) take an argument only when it is an integer inside
  i64; a fraction, NaN or ±∞ is `ABACO_ERR_MATH` (`isprime` answers 0).
  Through 2.4.8 a fraction was truncated silently: `3.9!` was 6.
- **Totient domain cap (`ABACO_TOTIENT_MAX = 10¹²`)** — φ(n) is computed by
  trial division to √n, so the evaluator bounds n the way it already bounds
  `factorial` (170) and `fibonacci` (92); 10¹² keeps the loop under ~10⁶ steps.
- **Checked f64 → i64 conversion (`abaco_f64_to_i64`, `src/core.cyr`)** — an
  integer domain is decided by range, `-2^63 ≤ x < 2^63`, using two ordered
  comparisons (both false for NaN), never by what an out-of-range conversion
  returns. No standard fixes that value, and Cyrius changed it at 6.6.8 — from
  the x86 "integer indefinite" `0x8000000000000000` to saturation by sign with
  NaN → 0.
  - ISO/IEC 9899:2011 (C11) §6.3.1.4 ¶1 (an unrepresentable integral part is
    undefined behaviour) and Annex F.4 (infinite, NaN or out-of-range: the
    result is unspecified, and "invalid" is raised).
  - IEEE 754-2019 §5.8 — `convertToInteger` signals invalid for NaN, infinite
    and out-of-range operands; it does not specify the delivered integer.
  - Intel® 64 and IA-32 Architectures SDM, Vol. 2, `CVTTSD2SI` — returns the
    integer indefinite `80000000_00000000H` when the invalid exception is
    masked.
- **Integer results must fit i64.** `binomial` / `choose` use
  `abaco_binomial` (see Number theory), which refuses a negative argument or a
  result past i64. `lcm` tests
  `a / gcd(a, b) ≤ ⌊i64_MAX / b⌋` before forming `(a / gcd) · b`, which for
  positive operands is exactly the condition that the product fits.
  - Knuth, *TAOCP Vol. 2* (3rd ed.), §4.5.2 — `lcm(u, v) = u·v / gcd(u, v)`.

## Natural language, history, currency — `src/ai.cyr`

- **History JSON.** `to_json` escapes `"`, `\` and every byte U+0000–U+001F
  (as `\b \f \n \r \t` or `\u00XX`); `from_json` decodes all of RFC 8259's
  escapes, `\uXXXX` surrogate pairs included, to UTF-8, and refuses an unknown
  escape, a lone surrogate or `\u0000` (a cstring cannot hold NUL).
  - RFC 8259 (2017), *The JavaScript Object Notation (JSON) Data Interchange
    Format*, §7 (strings) and §8.1 (UTF-8).
  - RFC 3629 (2003), *UTF-8*, §3 (encoding).
  - RFC 2781 (2000), *UTF-16*, §2.2 (decoding a surrogate pair:
    0x10000 + (hi − 0xD800)·0x400 + (lo − 0xDC00)).
- **Ring buffer.** `CalcHistory` keeps max_entries slots and a head index;
  entry i (0 = oldest) is slot (head + i) mod len — O(1) per push.
  - Knuth, *TAOCP Vol. 1* (3rd ed.), §2.2.2 (sequential allocation, circular
    queues).
- **U+2212 MINUS SIGN** (UTF-8 E2 88 92) is mapped to ASCII `-` by `nl_parse`:
  it is the minus of typeset text (Unicode code chart *Mathematical Operators*,
  U+2200–U+22FF).

## Constants

| Constant | Value | Source |
|----------|-------|--------|
| A4 frequency | 440.0 Hz | ISO 16:1975 |
| C0 frequency | 16.351597831287414 Hz | 12-TET: 440 / 2^(57/12), MIDI note 12 |
| Semitones/octave | 12 | 12-TET |
| Cents/octave | 1200 | Ellis (1885) |
| MR witness bound | ψ₁₂ = 318665857834031151167461 ≈ 3.187 × 10²³ | Sorenson & Webster (2017); OEIS A014233 |
| 1 lb | 0.45359237 kg (exact) | 1959 yard-and-pound agreement |
| 1 US gallon | 3.785411784 L (exact) | 231 in³ |
| g₀ | 9.80665 m/s² (exact) | 3rd CGPM (1901) |
| log2(10) | 3.321928… | for `10^x = exp2(x·log2 10)` |
