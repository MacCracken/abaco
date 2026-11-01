# Benchmark Results — Last 3 Runs

| Benchmark | 26-10-31 (61ac787-dirty) | 26-11-01 (8bda2cf-dirty) | 26-11-01 (17c653c-dirty) |
|-----------|------|------|------|
| is_prime_small | 3.15 us | 2.96 us | 3.10 us |
| is_prime_large | 8.17 us | 7.72 us | 7.98 us |
| next_prime | 3.82 us | 3.66 us | 3.81 us |
| factor_small | 145 ns | 135 ns | 144 ns |
| factor_large | 3.53 us | 3.38 us | 3.54 us |
| totient | 63 ns | 59 ns | 64 ns |
| factor_semiprime62 | 924.03 us | 887.63 us | 920.44 us |
| totient_semiprime62 | 922.88 us | 887.79 us | 919.01 us |
| mod_inverse | 34 ns | 36 ns | 33 ns |
| jacobi | 156 ns | 153 ns | 160 ns |
| crt3 | 138 ns | 134 ns | 136 ns |
| mobius | 3.34 us | 3.19 us | 3.32 us |
| sum_divisors | 3.37 us | 3.25 us | 3.40 us |
| carmichael | 3.40 us | 3.28 us | 3.40 us |
| primitive_root | 40.82 us | 39.21 us | 41.07 us |
| iroot3 | 258 ns | 252 ns | 258 ns |
| is_perfect_power | 558 ns | 542 ns | 556 ns |
| fibonacci | 109 ns | 108 ns | 111 ns |
| abaco_binomial | 131 ns | 131 ns | 133 ns |
| poly_blep_mid | 12 ns | 12 ns | 13 ns |
| poly_blep_edge | 15 ns | 14 ns | 15 ns |
| angular_frequency | 7 ns | 7 ns | 7 ns |
| sanitize_finite | 6 ns | 6 ns | 6 ns |
| sanitize_nan | 6 ns | 7 ns | 7 ns |
| hypot | 35 ns | 32 ns | 34 ns |
| lerp | 9 ns | 8 ns | 9 ns |
| clamp | 7 ns | 7 ns | 7 ns |
| round | 12 ns | 11 ns | 12 ns |
| sqrt | 3 ns | 3 ns | 3 ns |
| abs | 3 ns | 3 ns | 3 ns |
| amplitude_to_db | 50 ns | 47 ns | 54 ns |
| db_to_amplitude | 23 ns | 21 ns | 22 ns |
| db_gain_factor | 22 ns | 22 ns | 23 ns |
| midi_to_freq | 38 ns | 35 ns | 39 ns |
| freq_to_midi | 34 ns | 31 ns | 33 ns |
| pitch_class | 65 ns | 62 ns | 62 ns |
| time_constant | 47 ns | 45 ns | 47 ns |
| pan_center | 97 ns | 66 ns | 71 ns |
| crossfade_mid | 71 ns | 66 ns | 71 ns |
| window_hann | 64 ns | 60 ns | 66 ns |
| window_hann_periodic | 63 ns | 60 ns | 62 ns |
| window_hamming | 62 ns | 61 ns | 62 ns |
| window_hamming_periodic | 62 ns | 61 ns | 63 ns |
| window_blackman | 104 ns | 102 ns | 104 ns |
| window_blackman_periodic | 103 ns | 99 ns | 106 ns |
| window_kaiser | 319 ns | 300 ns | 320 ns |
| window_kaiser_periodic | 320 ns | 299 ns | 321 ns |
| window_nuttall | — | 155 ns | 162 ns |
| window_blackman_harris | — | 155 ns | 161 ns |
| window_flattop | — | 191 ns | 204 ns |
| window_bartlett | — | 14 ns | 14 ns |
| window_tukey | — | 74 ns | 81 ns |
| window_gaussian | — | 48 ns | 51 ns |
| window_enbw_1024 | — | 15.33 us | 16.14 us |
| window_scalloping_1024 | — | 109.85 us | 113.67 us |
| biquad_design_peak | — | 176 ns | 186 ns |
| biquad_response | — | 272 ns | 277 ns |
| biquad_process_256 | — | 2.95 us | 3.04 us |
| fft_twiddles_1024 | — | — | 28.85 us |
| fft_ifft_1024 | — | — | 103.69 us |
| rfft_1024 | — | — | 31.38 us |
| stft_frame_1024 | — | — | 38.73 us |
| magnitude_spectrum_513 | — | — | 16.03 us |
| power_spectrum_513 | — | — | 639 ns |
| category_name | 6 ns | 6 ns | 5 ns |
| category_from_str | 882 ns | 871 ns | 886 ns |
| value_integer | 15 ns | 15 ns | 15 ns |
| value_as_f64 | 21 ns | 26 ns | 30 ns |
| sanitize_4096 | 15.58 us | 15.05 us | 15.59 us |
| poly_blep_4096 | 32.06 us | 31.38 us | 31.96 us |
| hypot_4096 | 66.32 us | 64.20 us | 66.05 us |
| sqrt_4096 | 9.52 us | 9.16 us | 9.51 us |
| round_4096 | 33.25 us | 32.47 us | 33.36 us |
| simd_add_4096 | 1.64 us | 1.52 us | 1.62 us |
| simd_mul_4096 | 1.62 us | 1.52 us | 1.62 us |
| simd_sub_4096 | 1.62 us | 1.54 us | 1.58 us |
| simd_mac_4096 | 8.93 us | 11.81 us | 8.97 us |
| simd_div_4096 | 2.45 us | 2.34 us | 2.43 us |
| simd_sqrt_4096 | 4.77 us | 4.54 us | 4.78 us |
| simd_abs_4096 | 1.21 us | 1.11 us | 1.20 us |
| kaiser_fill_1024 | 156.39 us | 153.81 us | 156.09 us |
| kaiser_periodic_fill_1024 | 156.48 us | 152.62 us | 156.19 us |
| addition | 415 ns | 410 ns | 412 ns |
| mixed_ops | 618 ns | 612 ns | 612 ns |
| parentheses | 813 ns | 800 ns | 805 ns |
| division | 405 ns | 412 ns | 407 ns |
| eval_sqrt | 817 ns | 807 ns | 819 ns |
| sin | 841 ns | 841 ns | 857 ns |
| log2 | 853 ns | 834 ns | 851 ns |
| pow | 1.27 us | 1.26 us | 1.27 us |
| min | 1.01 us | 1.01 us | 1.02 us |
| lcm | 1.00 us | 1.00 us | 1.02 us |
| choose | 1.11 us | 1.10 us | 1.11 us |
| tan | 772 ns | 775 ns | 776 ns |
| mean8 | 2.67 us | 2.66 us | 2.67 us |
| stddev8 | — | 3.29 us | 3.37 us |
| mean_ovf | — | 6.10 us | 6.12 us |
| factorial170 | 1.83 us | 1.79 us | 1.78 us |
| mixed_ops_funcs | 1.60 us | 1.60 us | 1.61 us |
| nested_parens | 2.06 us | 2.05 us | 2.06 us |
| trig_chain | 1.99 us | 1.95 us | 2.00 us |
| long_addition | 2.03 us | 2.00 us | 2.03 us |
| sci_add | 505 ns | 500 ns | 502 ns |
| sci_mul | 434 ns | 455 ns | 455 ns |
| lit_long40 | 816 ns | 780 ns | 802 ns |
| lit_near_mid | 1.46 us | 1.47 us | 1.46 us |
| lit_tie_1e23 | 586 ns | 577 ns | 587 ns |
| lit_sci_const | 532 ns | 523 ns | 526 ns |
| lit_mid_worst | 25.56 us | 25.35 us | 25.57 us |
| tok_simple | 764 ns | 743 ns | 743 ns |
| tok_complex | 1.74 us | 1.71 us | 1.75 us |
| nl_parse_convert | 1.94 us | 1.87 us | 1.90 us |
| ccy_http_split | 2.29 us | 2.31 us | 2.40 us |
| history_push_full | 33 ns | 32 ns | 34 ns |
| km_to_miles | 330 ns | 323 ns | 328 ns |
| celsius_to_fahr | 303 ns | 302 ns | 307 ns |
| bytes_to_gb | 296 ns | 294 ns | 298 ns |
| bytes_to_gib | 458 ns | 309 ns | 372 ns |
| gb_to_gib_cross | 476 ns | 320 ns | 384 ns |
| same_unit_id | 303 ns | 298 ns | 302 ns |
| exact_symbol | 129 ns | 127 ns | 129 ns |
| by_name | 429 ns | 439 ns | 303 ns |
| case_insensitive | 318 ns | 349 ns | 320 ns |
| plural | 398 ns | 228 ns | 422 ns |
| miss | 269 ns | 152 ns | 154 ns |
| alias_kph | 365 ns | 325 ns | 532 ns |
| mpg_to_l100km | 502 ns | 474 ns | 749 ns |
| density_g_to_kg | 432 ns | 427 ns | 504 ns |
| viscosity_cp_pas | 526 ns | 331 ns | 406 ns |
| lux_to_fc | 318 ns | 314 ns | 363 ns |
| registry_creation | 182.70 us | 189.28 us | 179.79 us |

_Generated by `scripts/bench-history.sh` — Cyrius port benchmarks_
