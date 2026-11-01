# Benchmark Results — Last 3 Runs

| Benchmark | 26-10-31 (36c8a6b-dirty) | 26-10-31 (61ac787-dirty) | 26-11-01 (8bda2cf-dirty) |
|-----------|------|------|------|
| is_prime_small | 3.14 us | 3.15 us | 2.96 us |
| is_prime_large | 8.05 us | 8.17 us | 7.72 us |
| next_prime | 3.85 us | 3.82 us | 3.66 us |
| factor_small | 146 ns | 145 ns | 135 ns |
| factor_large | 3.56 us | 3.53 us | 3.38 us |
| totient | 63 ns | 63 ns | 59 ns |
| factor_semiprime62 | 933.63 us | 924.03 us | 887.63 us |
| totient_semiprime62 | 941.27 us | 922.88 us | 887.79 us |
| mod_inverse | — | 34 ns | 36 ns |
| jacobi | — | 156 ns | 153 ns |
| crt3 | — | 138 ns | 134 ns |
| mobius | — | 3.34 us | 3.19 us |
| sum_divisors | — | 3.37 us | 3.25 us |
| carmichael | — | 3.40 us | 3.28 us |
| primitive_root | — | 40.82 us | 39.21 us |
| iroot3 | — | 258 ns | 252 ns |
| is_perfect_power | — | 558 ns | 542 ns |
| fibonacci | 115 ns | 109 ns | 108 ns |
| abaco_binomial | 134 ns | 131 ns | 131 ns |
| poly_blep_mid | 12 ns | 12 ns | 12 ns |
| poly_blep_edge | 15 ns | 15 ns | 14 ns |
| angular_frequency | 7 ns | 7 ns | 7 ns |
| sanitize_finite | 7 ns | 6 ns | 6 ns |
| sanitize_nan | 6 ns | 6 ns | 7 ns |
| hypot | 36 ns | 35 ns | 32 ns |
| lerp | 9 ns | 9 ns | 8 ns |
| clamp | 8 ns | 7 ns | 7 ns |
| round | 12 ns | 12 ns | 11 ns |
| sqrt | 3 ns | 3 ns | 3 ns |
| abs | 4 ns | 3 ns | 3 ns |
| amplitude_to_db | 52 ns | 50 ns | 47 ns |
| db_to_amplitude | 22 ns | 23 ns | 21 ns |
| db_gain_factor | 22 ns | 22 ns | 22 ns |
| midi_to_freq | 39 ns | 38 ns | 35 ns |
| freq_to_midi | 33 ns | 34 ns | 31 ns |
| pitch_class | 63 ns | 65 ns | 62 ns |
| time_constant | 49 ns | 47 ns | 45 ns |
| pan_center | 73 ns | 97 ns | 66 ns |
| crossfade_mid | 73 ns | 71 ns | 66 ns |
| window_hann | 66 ns | 64 ns | 60 ns |
| window_hann_periodic | 63 ns | 63 ns | 60 ns |
| window_hamming | 65 ns | 62 ns | 61 ns |
| window_hamming_periodic | 64 ns | 62 ns | 61 ns |
| window_blackman | 105 ns | 104 ns | 102 ns |
| window_blackman_periodic | 106 ns | 103 ns | 99 ns |
| window_kaiser | 322 ns | 319 ns | 300 ns |
| window_kaiser_periodic | 325 ns | 320 ns | 299 ns |
| window_nuttall | — | — | 155 ns |
| window_blackman_harris | — | — | 155 ns |
| window_flattop | — | — | 191 ns |
| window_bartlett | — | — | 14 ns |
| window_tukey | — | — | 74 ns |
| window_gaussian | — | — | 48 ns |
| window_enbw_1024 | — | — | 15.33 us |
| window_scalloping_1024 | — | — | 109.85 us |
| biquad_design_peak | — | — | 176 ns |
| biquad_response | — | — | 272 ns |
| biquad_process_256 | — | — | 2.95 us |
| category_name | 6 ns | 6 ns | 6 ns |
| category_from_str | 897 ns | 882 ns | 871 ns |
| value_integer | 15 ns | 15 ns | 15 ns |
| value_as_f64 | 22 ns | 21 ns | 26 ns |
| sanitize_4096 | 15.14 us | 15.58 us | 15.05 us |
| poly_blep_4096 | 32.60 us | 32.06 us | 31.38 us |
| hypot_4096 | 67.35 us | 66.32 us | 64.20 us |
| sqrt_4096 | 9.67 us | 9.52 us | 9.16 us |
| round_4096 | 33.90 us | 33.25 us | 32.47 us |
| simd_add_4096 | 1.63 us | 1.64 us | 1.52 us |
| simd_mul_4096 | 1.68 us | 1.62 us | 1.52 us |
| simd_sub_4096 | 1.68 us | 1.62 us | 1.54 us |
| simd_mac_4096 | 9.35 us | 8.93 us | 11.81 us |
| simd_div_4096 | 2.43 us | 2.45 us | 2.34 us |
| simd_sqrt_4096 | 4.79 us | 4.77 us | 4.54 us |
| simd_abs_4096 | 1.28 us | 1.21 us | 1.11 us |
| kaiser_fill_1024 | 158.03 us | 156.39 us | 153.81 us |
| kaiser_periodic_fill_1024 | 158.31 us | 156.48 us | 152.62 us |
| addition | 441 ns | 415 ns | 410 ns |
| mixed_ops | 644 ns | 618 ns | 612 ns |
| parentheses | 857 ns | 813 ns | 800 ns |
| division | 412 ns | 405 ns | 412 ns |
| eval_sqrt | 833 ns | 817 ns | 807 ns |
| sin | 871 ns | 841 ns | 841 ns |
| log2 | 872 ns | 853 ns | 834 ns |
| pow | 1.35 us | 1.27 us | 1.26 us |
| min | 1.06 us | 1.01 us | 1.01 us |
| lcm | 1.04 us | 1.00 us | 1.00 us |
| choose | 1.15 us | 1.11 us | 1.10 us |
| tan | 789 ns | 772 ns | 775 ns |
| mean8 | 2.75 us | 2.67 us | 2.66 us |
| stddev8 | — | — | 3.29 us |
| mean_ovf | — | — | 6.10 us |
| factorial170 | 1.90 us | 1.83 us | 1.79 us |
| mixed_ops_funcs | 2.17 us | 1.60 us | 1.60 us |
| nested_parens | 2.32 us | 2.06 us | 2.05 us |
| trig_chain | 2.05 us | 1.99 us | 1.95 us |
| long_addition | 2.10 us | 2.03 us | 2.00 us |
| sci_add | 515 ns | 505 ns | 500 ns |
| sci_mul | 465 ns | 434 ns | 455 ns |
| lit_long40 | 860 ns | 816 ns | 780 ns |
| lit_near_mid | 1.54 us | 1.46 us | 1.47 us |
| lit_tie_1e23 | 592 ns | 586 ns | 577 ns |
| lit_sci_const | 559 ns | 532 ns | 523 ns |
| lit_mid_worst | 26.52 us | 25.56 us | 25.35 us |
| tok_simple | 864 ns | 764 ns | 743 ns |
| tok_complex | 1.82 us | 1.74 us | 1.71 us |
| nl_parse_convert | 1.98 us | 1.94 us | 1.87 us |
| ccy_http_split | 2.52 us | 2.29 us | 2.31 us |
| history_push_full | 35 ns | 33 ns | 32 ns |
| km_to_miles | 342 ns | 330 ns | 323 ns |
| celsius_to_fahr | 318 ns | 303 ns | 302 ns |
| bytes_to_gb | 311 ns | 296 ns | 294 ns |
| bytes_to_gib | 324 ns | 458 ns | 309 ns |
| gb_to_gib_cross | 339 ns | 476 ns | 320 ns |
| same_unit_id | 313 ns | 303 ns | 298 ns |
| exact_symbol | 133 ns | 129 ns | 127 ns |
| by_name | 315 ns | 429 ns | 439 ns |
| case_insensitive | 331 ns | 318 ns | 349 ns |
| plural | 320 ns | 398 ns | 228 ns |
| miss | 163 ns | 269 ns | 152 ns |
| alias_kph | 202 ns | 365 ns | 325 ns |
| mpg_to_l100km | 551 ns | 502 ns | 474 ns |
| density_g_to_kg | 452 ns | 432 ns | 427 ns |
| viscosity_cp_pas | 348 ns | 526 ns | 331 ns |
| lux_to_fc | 330 ns | 318 ns | 314 ns |
| registry_creation | 189.98 us | 182.70 us | 189.28 us |

_Generated by `scripts/bench-history.sh` — Cyrius port benchmarks_
