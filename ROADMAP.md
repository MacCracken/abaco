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
- [ ] **BLOCKER**: ai module — requires network/async (reqwest, tokio, chrono, serde_json); hoosh problem

## Ecosystem Rollout

- [ ] Audit shruti for duplicated math that should use `abaco::dsp`
- [ ] Audit jalwa for duplicated math
- [ ] Audit tarang for duplicated math
- [ ] Audit aethersafta for duplicated math
- [ ] Standardize all Agnos projects on abaco for shared math
