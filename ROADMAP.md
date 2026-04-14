# Abaco Roadmap

## DSP Module Expansion

- [ ] Window functions — Hann, Hamming, Blackman, Kaiser (currently inline in dhvani's FFT/STFT)
- [ ] Interpolation math — linear lerp, cubic, windowed sinc kernel (used in resamplers, delay lines)
- [ ] Chromagram helpers — `freq_to_pitch_class`, `C0` constant, semitone mapping (currently inline in dhvani analysis)

## Audio Unit Conversions

- [ ] Add audio-specific units to `UnitRegistry`: dBFS, samples, BPM, semitones
- [ ] BPM ↔ Hz conversion (`registry.convert(120.0, "bpm", "Hz")`)
- [ ] Samples ↔ milliseconds (sample-rate-aware)

## Cyrius Port — unlocks hisab

- [x] Port core module (Value struct, Unit struct, constants)
- [x] Port ntheory module (pure i64, no deps)
- [x] Port dsp module (all 23 functions, Cyrius 1.7.8 transcendentals)
- [x] Port eval module (tokenizer + recursive descent parser, 43+ functions)
- [x] Port units module (80+ units, 18 categories, hashmap registry)
- [x] SIMD batch DSP — f64v_add/sub/mul, batch_add/sub/mul/scale/mac (1us/4096 elements)
- [x] Parity audit — to_latex, eval_partial, list_units, missing units/aliases, hyperbolic trig
- [ ] ai module — NL parsing, calculation history, currency conversion (unblocked 2026-04-14: hoosh 2.0 + ai-hwaccel 2.0 now Cyrius-ported)

## Cyrius Port — Known Gaps (intentional or blocked)

- **f32 variants** — Cyrius is f64-only, no f32 type. All DSP uses f64. Not a gap.
- **Token enum not public** — Tokenizer is internal to eval. Consumers use Evaluator_eval.
- **Tuple returns** — pan/crossfade use output pointers instead. Cyrius has no multi-return.
- **asin/acos/atan** — Implemented via sin/cos division (stopgap). Cyrius builtins requested.
- **sinh/cosh/tanh** — Now use `lib/math.cyr` (`f64_sinh/cosh/tanh`) as of 2026-04-14.
- **u128 / is_prime perf** — mod_mul binary method (18-33x vs Rust). Blocked on Cyrius u128.
- **256 function limit** — Tests must exclude eval to fit units tests. Cyrius raised to 1024 in v1.9+.

## Ecosystem Rollout

- [ ] Audit shruti for duplicated math that should use `abaco::dsp`
- [ ] Audit jalwa for duplicated math
- [ ] Audit tarang for duplicated math
- [ ] Audit aethersafta for duplicated math
- [ ] Standardize all Agnos projects on abaco for shared math
