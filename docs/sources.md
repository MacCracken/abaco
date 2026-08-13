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
  *n* < 3.317 × 10²⁴, which covers the entire i64 range exactly (not
  probabilistic).
  - Jaeschke, G. (1993). "On strong pseudoprimes to several bases."
    *Mathematics of Computation*, 61(204), 915–926. doi:10.1090/S0025-5718-1993-1192971-8
  - Sorenson, J. & Webster, J. (2015). "Strong pseudoprimes to twelve prime
    bases." *Mathematics of Computation*, 86(304), 985–1003. doi:10.1090/mcom/3134
  - Avoids the random-witness weakness of Albrecht et al. (2018),
    "Prime and Prejudice: Primality Testing Under Adversarial Conditions"
    (ACM CCS 2018) — abaco's witness set is fixed and exact.
- **Modular exponentiation / multiplication** — right-to-left binary method,
  delegated to stdlib `u64_powmod` / `u64_mulmod`.
  - Knuth, D. E. *The Art of Computer Programming, Vol. 2: Seminumerical
    Algorithms* (3rd ed.), §4.6.3.
- **Euler's totient φ(n)** via prime-factor product form.
  - Hardy, G. H. & Wright, E. M. *An Introduction to the Theory of Numbers*
    (6th ed.), Theorem 62.
- **Trial-division factorization** — classical; trial divisors 2 then odd
  *d* up to √n.

## DSP — `src/dsp.cyr`

- **12-tone equal temperament (12-TET), MIDI ↔ frequency.**
  `freq = 440 · 2^((m−69)/12)`. A4 = 440 Hz (ISO 16:1975); MIDI note 69 = A4.
- **C0 reference** = 16.3516 Hz (MIDI note 12), used for pitch-class /
  octave computation: `round(12 · log2(freq / C0))`.
  - `round` here is **ties away from zero** (`f64_round_half_away`), not the
    IEEE-754 round-half-to-even that the Cyrius 6.5.x `f64_round` builtin
    implements. Same rule as the expression evaluator's user-facing `round()`;
    the two modes are pinned apart by `test_round_ties_away` (2.3.4).
    - IEEE 754-2019 §4.3.1 (roundTiesToEven, the default) vs §4.3.3
      (roundTiesToAway, an explicitly sanctioned attribute).
- **Decibel** — `20 · log10(amplitude ratio)` (field/amplitude quantity);
  dBFS uses a 1.0 full-scale reference (float-audio norm).
- **Window functions.** Coefficients per the standard survey:
  - Harris, F. J. (1978). "On the Use of Windows for Harmonic Analysis with
    the Discrete Fourier Transform." *Proc. IEEE*, 66(1), 51–83. doi:10.1109/PROC.1978.10837
  - Hann / Hamming (a₀ = 0.54, a₁ = 0.46); Blackman (0.42, 0.5, 0.08);
    Kaiser window via the zeroth-order modified Bessel function I₀(β).
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

- **SI definitions and exact conversion factors.**
  - BIPM. *The International System of Units (SI)*, 9th ed. (2019).
  - NIST Special Publication 811 (2008), "Guide for the Use of the
    International System of Units (SI)" — exact factors (e.g. 1 in = 0.0254 m,
    1 mile = 1609.344 m, 1 nautical mile = 1852 m, 1 lb = 0.453592 kg).
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
- **IEEE 754 number parsing** with scientific notation; exponent clamped at
  308 (f64 overflows to +inf near 1.8 × 10³⁰⁸) so adversarial `1e999…`
  inputs terminate in bounded work.
  - IEEE 754-2019, *Standard for Floating-Point Arithmetic*.
- **Integer-exponent power bound (`ABACO_POW_EXACT_MAX = 1024`)** — `eval_pow`
  resolves whole exponents by repeated multiplication for exactness (`2^10` is
  1024, not a log/exp round-trip). f64 saturates to ±∞ at 2¹⁰²⁴ and to 0 below
  ~5 × 10⁻³²⁴, so no base can still be changing after 1024 steps; past the bound
  the O(1) `exp2(exp · log2|base|)` path yields the same value. Bounding it
  turns `2^1000000000000000000` from ~10¹⁸ multiplications into constant work.
  Same reasoning the Cyrius stdlib applied to its own decimal-exponent loop in
  `lib/math.cyr` at 6.4.69.
- **Totient domain cap (`ABACO_TOTIENT_MAX = 10¹²`)** — φ(n) is computed by
  trial division to √n, so the evaluator bounds n the way it already bounds
  `factorial` (170) and `fibonacci` (92); 10¹² keeps the loop under ~10⁶ steps.

## Constants

| Constant | Value | Source |
|----------|-------|--------|
| A4 frequency | 440.0 Hz | ISO 16:1975 |
| C0 frequency | 16.3516 Hz | 12-TET, MIDI note 12 |
| Semitones/octave | 12 | 12-TET |
| Cents/octave | 1200 | Ellis (1885) |
| MR witness bound | 3.317 × 10²⁴ | Sorenson & Webster (2015) |
| log2(10) | 3.321928… | for `10^x = exp2(x·log2 10)` |
