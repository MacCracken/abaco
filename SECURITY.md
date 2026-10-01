# Security Policy

## Scope

Abaco is the AGNOS math engine (Cyrius). It has no network I/O in
the core compute modules. The `ai` module's `CurrencyCache_fetch` talks to a
configurable rates endpoint over one of two transports: plaintext
`lib/http.cyr::http_get` to a `http://localhost` / `127.0.0.1` server only,
or — from 2.4.12, opt-in — HTTPS through `lib/tls.cyr`'s native backend, compiled
only into a consumer that builds with `-D ABACO_TLS` (and lists `tls` in its own
stdlib). A build without the define links no TLS code and refuses `https://`
with `AI_ERR_TLS`. Consumers that don't call `fetch` never open a socket. The
history save/load pair reads and writes the file path the consumer passes, and
`CurrencyCache_set_ca_file` names a CA bundle the HTTPS fetch reads.

## Attack surface

| Area | Risk | Mitigation |
|------|------|------------|
| Expression parsing | Stack overflow via deep nesting | `eval_depth` bounded at `ABACO_MAX_DEPTH`; **every** recursive path charges depth — parens, call arguments, unary signs and `^` chains (the last two were unbounded before 2.3.4) |
| Expression parsing | Token-array overrun | `tokenize` / `implicit_mul` bound every write against `ABACO_MAX_TOKENS` (1024 as of 2.4.0), returning `ABACO_ERR_PARSE` (unchecked before 2.3.4 — see the 2026-08-13 audit) |
| Expression parsing | Algorithmic-complexity DoS | `eval_pow` uses binary exponentiation (O(log n); an integer power whose result is past ±2^1100 returns ±Inf or 0 at once), `totient` capped at `ABACO_TOTIENT_MAX`, a literal's decimal exponent saturates at `ABACO_DEC_EXP_MAX` (10^9) and is applied through at most `ABACO_POW10_MAX` (340) scalings, `factorial` at 170, `fibonacci` at 92. `Evaluator_eval_partial` parses at most `1 + ABACO_PARTIAL_TRIES` (5) times (through 2.4.8 it retried once per token, about 88 s on a 1000-token input). `%` scales its divisor in exact steps, so a subnormal divisor terminates (a first 2.4.9 cut looped forever on `1e300 % 5e-324`) |
| Expression parsing | Out-of-range f64 → integer | Every user-controlled conversion goes through `abaco_f64_to_i64`, which refuses NaN, ±Inf and \|x\| ≥ 2^63 — the integer functions then answer `ABACO_ERR_MATH`. Before 2.4.7 the evaluator leaned on x86's old `f64_to` answer; on Cyrius ≥ 6.6.8 (and aarch64 always) `nextprime(1e300)` never returned, and `lcm(-1e300, 1e300)` raised SIGFPE on x86 — as `lcm(-(2^63), 5)` did on every toolchain. aarch64's `sdiv` does not trap, so there the same calls returned a wrong value (-9.22e18) with no error |
| Expression parsing | Integer overflow in numeric literals | `parse_number` uses an exact 18-digit mantissa plus a decimal exponent — no accumulator can wrap. Both exponents are genuinely combined before clamping as of 2.3.5 (2.3.4 still saturated the written exponent separately, preserving the bug its own comment claimed to have removed) |
| History JSON | Structure smuggled through string values | All structural scanning skips string contents via `_jf_skip_string`; keys match only in key position. Before 2.3.4 a `{`, `}` or `]` in any field silently destroyed the whole history on reload |
| Expression parsing | Bytes the grammar does not know | `tokenize` refuses any byte it does not recognise (`ABACO_ERR_PARSE`). Through 2.4.8 it skipped them, so a pasted U+2212 minus or `²` vanished: `−5+3` was 8 and `3²` was 3, with no error. `nl_parse` maps U+2212 to `-` before tokenising |
| Division by zero | Undefined / inf propagation | `/` and `%` by zero set `ABACO_ERR_DIV_ZERO`; a reciprocal unit (L/100km) refuses a zero, negative, non-finite or overflowing amount with `UERR_CONVERT` |
| NaN / Infinity | Silent propagation | **Not trapped.** NaN and ±Inf are ordinary IEEE values: `sqrt(-1)` is NaN and `1e308*10` is +Inf with `ABACO_ERR_NONE`. A consumer that must not show them tests the result bits. In DSP, `sanitize_sample` scrubs a sample when the consumer calls it; nothing calls it implicitly. The integer functions (`isprime`, `factor`, `gcd`, …) refuse NaN, ±Inf and non-integers: `isprime` answers 0, the others set `ABACO_ERR_MATH` |
| Unit lookup | Malformed query | Hashmap-based, constant work per lookup; unknown, ambiguous or NULL → `UERR_UNKNOWN`, never panics (a NULL query was a SIGSEGV through 2.4.8) |
| Unit lookup | Wrong unit from case folding | Symbols match only as written, as SI and IEC define them, and names and aliases in any case. Through 2.4.8 every symbol also matched case-folded, so `MW` read in lowercase was milliwatt (10^9 too small), `mHz` was MHz and `Mb` (megabit) was MB, all with no error |
| AI currency fetch | Plaintext fetch / rate poisoning | `_ccy_validate_url` accepts `https://`, or plaintext only when `localhost` / `127.0.0.1` is the **whole** host (then `:`, `/` or end). Through 2.4.6 a prefix match let `http://localhost.<domain>` through — inert until Cyrius 6.6.9's `http_get` began connecting. Control bytes are refused anywhere in the URL (audit §4.1) |
| AI currency fetch (HTTPS) | Impersonation / MITM | Native backend only, fail closed: the chain must reach a trusted root (the system store, or only the cache's CA file — one that is unreadable, empty, holds no PEM certificate or more than 300 is `AI_ERR_TLS` before any socket, one whose certificates do not parse is `AI_ERR_TLS` at the handshake; never a fallback) **and** the leaf's subjectAltName must match the host: a DNS name against dNSName entries (lib/tls.cyr), an IPv4 literal against iPAddress entries **only** (RFC 9525 §6.3). lib/tls.cyr in Cyrius 6.6.12 also matches an IP literal against a dNSName that spells it, wildcards included (`DNS:127.0.0.1`, `DNS:*.0.0.1`), so abaco re-reads the verified leaf and requires the iPAddress itself (`_ccy_cert_ip_san`; cyrius issue 2026-10-01-tls-ip-literal-dnsname). The libssl backend is refused — at build time under `-D CYRIUS_TLS_LIBSSL`, at runtime unless `tls_get_backend()` is native (checked before connecting and again in the connect hook) — because in Cyrius 6.6.12 it never checks the host name (cyrius issue 2026-09-30, High). No redirect is followed. TLS 1.3 only: the native client offers 1.2 only when pinned to it, which abaco does not do, so a TLS 1.2-only server is refused |
| AI currency fetch (HTTPS) | Truncated / smuggled response | `tls_read` reports any alert, fatal ones included, as end of stream, so the body is framed by `Content-Length` only: required, `1*DIGIT` and equal when repeated; reading stops once head + `Content-Length` bytes are in, so a body cut short is refused, bytes past it that arrive in the same read are refused, and bytes the server sends later are never read; `Transfer-Encoding`, obs-fold, bare CR / LF and control bytes in the head are refused (`_ccy_http_head`) |
| AI currency fetch (HTTPS) | Stalled fetch (slowloris, on-path stall) | `SO_RCVTIMEO` / `SO_SNDTIMEO` (10 s) bound each socket operation, not the fetch: a peer dripping a byte every few seconds, or anyone on the path injecting plaintext ChangeCipherSpec records (lib/tls.cyr skips them without a limit, during and after the handshake; no key or certificate needed), held a fetch, and its thread, forever. Connect, handshake and response now run under a 30 s deadline: a per-fetch watchdog thread (Linux; its state is one heap block per fetch, no global) shuts the socket's read side and swaps it for `/dev/null`, and the fetch returns `AI_ERR_HTTP`. Name resolution comes before it, with lib/net.cyr's own bound (2 tries, 2 s per read). On a target other than Linux only the per-operation timeouts apply (cyrius issue 2026-10-01-tls-native-no-deadline) |
| AI currency fetch (HTTPS) | Resource exhaustion | Response ≤ 64 KB, request ≤ 2048 B (sized before allocating), at most 32 `tls_read` calls per fetch (each native read retains ~33 KB on the no-free heap, so 1-byte records cannot cost 64 Ki × 33 KB), 10 s per socket operation and 30 s per fetch; every exit closes the TLS ctx and the socket. Retained heap per fetch ~0.5 MB, worst case ~1.5 MB; a CA file adds ~1.5× its size per fetch (its decoded and parsed roots — lib/tls.cyr parses the system store first anyway) plus, once per cache, a read buffer that grows to ≤ 3× the file and is reused after (measured with a 186 KB bundle: 1,314,848 B for the first fetch through a cache, 794,592 B for each after). A bundle of more than 300 certificates (lib/tls.cyr's `TLS_CA_MAX_ROOTS`) is refused before any socket, unparsed |
| AI currency fetch (HTTPS) | Threads racing the TLS stack's first use | `abaco_tls_init()` warms sigil's lazy tables and the system-CA cache on the main thread (it refuses any other thread); a fetch refuses with `AI_ERR_TLS` until it has run. Unwarmed, two concurrent first handshakes broke every later one, and a worker-first handshake crashed the main thread (cyrius issue 2026-09-30-tls-first-use-thread-race) |
| AI currency fetch | Malicious response body | Every extractor bounds-checks its offsets against the body length. `base` and `rates` are read from the top-level object only (through 2.4.8 a `"base"` inside `"rates"` won); the rates object must be flat; each value is trimmed of JSON whitespace and parsed as one strict literal; a rate must be finite, > 0 and ≤ 10^6. Malformed → `AI_ERR_CURRENCY`, no crash (covered by `fuzz_ai` and explicit tests) |
| AI currency convert | Overflowing cross-rate | `CurrencyCache_convert` refuses a rate or result that is not finite, or a rate that is not > 0, with `AI_ERR_CURRENCY` — including rates installed through `set_rates`, which are not vetted. Through 2.4.8 a 0 rate or a tiny source rate came back as +Inf with `AI_OK` |
| History file | Lost or corrupted history | `save_to_file` writes a temp file and renames it over the target, so a failed save leaves the old file intact. Save and load share one 16 MiB bound. `from_json` checks the whole array before storing any entry, so a malformed file leaves the history unchanged (through 2.4.8 it answered success for non-JSON and kept a prefix on error) |
| Natural-language parse | Adversarial input | `fuzz_ai.fcyr` runs 20k+ inputs through `nl_parse`, `CalcHistory_*`, `_ccy_load_body` and `_ccy_validate_url`, asserting the MED-7 rate and §4.1 URL invariants directly |
| ntheory primality | Timing side-channel | **Not constant-time.** `mod_pow` (via `bayan_u64_powmod`) branches on exponent bits and Miller–Rabin returns at the first witness that proves a composite. abaco is not for secret-dependent arithmetic |

## Fuzz coverage

- `fuzz/fuzz_eval.fcyr`    — random expression strings (up to 1200 bytes) → `Evaluator_eval` + `Evaluator_eval_partial`, plus targeted adversarial shapes every 4th iteration: long exponent digit-runs, deep unary-sign chains, deep `^` chains, and token-array overruns
- `fuzz/fuzz_ntheory.fcyr` — random i64 → `is_prime`, `factor`, `totient`, `next_prime`; cross-checks `is_prime` against trial division for n < 10⁶
- `fuzz/fuzz_units.fcyr`   — random cstrings → `UnitRegistry_find`, `UnitRegistry_convert`; checks sampled factors bit-for-bit against the exact definitions, that two spellings of one unit convert as the identity, and that `"<letter>s"` never resolves
- `fuzz/fuzz_ai.fcyr`      — `nl_parse` on random bytes and grammar-shaped phrases; `CalcHistory` ring-buffer bounds and JSON round-trip; `_ccy_load_body` on adversarial rate payloads and deep nesting; `_ccy_validate_url`; and (2.4.12) the HTTPS parsers `_ccy_https_split`, `_ccy_https_request` and `_ccy_http_split`, each against a grammar written out in the harness, in both directions, plus truncation, extra-byte and never-read-past-`len` checks; the IP-literal SAN check `_ccy_cert_ip_san` against generated certificates whose answer is known by construction (length forms, critical flags, other extensions and GeneralName kinds, a second SAN), with truncation, never-read-past-`len`, mutation and raw-byte checks; and `_ccy_pem_cert_count` against placed markers and near-misses, on every prefix. Asserts, with oracles that do not call the code under test, that every cached rate is finite/positive/≤ 10⁶ and every must-drop rate is absent; that `% of` is rewritten exactly when both operands are strict literals; that `_nl_parse_f64` accepts exactly the strict literals, with the evaluator's value; that no URL with a control byte is ever accepted; and that loopback is accepted only as the whole host

Run with `./fuzz/run.sh [iters]`. Each harness has passed 20k+ iterations with
no crashes or invariant violations.

> The 2026-08-13 audit found the pre-2.3.4 `fuzz_eval` could not reach any of
> the four defects it reported: a 48-byte input cap left a 10× margin against
> the 512-token limit that the generator could never close, and `^` appeared
> only via a ~0.04%/byte wild-byte path. Passing iteration counts are not
> coverage. The extended harness hangs against the 2.3.3 evaluator.

## Supported versions

| Version | Supported |
|---------|-----------|
| 2.4.x   | Yes (current — Cyrius 6.6.x) |
| 2.3.x   | Security fixes only |
| 2.0.x – 2.2.x | Security fixes only |
| 1.x     | No (Rust crate, unmaintained) |

⚠ The `mod_pow` SIGFPE fixed at 2.4.3 arrives through the **stdlib**, not
through abaco's own source: `dist/abaco.cyr` bundles no `bayan`, so a consumer
supplies it from their own `[deps].stdlib`. Pinning abaco 2.4.x does not by
itself close it — the consumer's own `cyrius.cyml` must pin **≥ 6.5.34**, where
bayan 1.5.2 lands (`lib/bayan.cyr` is byte-identical between 6.5.34 and 6.5.35,
so 6.5.34 is the true floor). abaco's own pin is `[package].cyrius` in
`cyrius.cyml`.

## Reporting vulnerabilities

- Email: security@agnos.dev
- Please do not open public issues for security bugs.
- 48-hour acknowledgment SLA.
- 90-day coordinated disclosure timeline.

## Design principles

- **Pure compute, headless.** No filesystem / network / process side
  effects in core compute modules. ai's HTTP fetch is opt-in.
- **Cyrius-level safety.** No raw C, no `unsafe` escape hatches — the
  language itself forbids pointer arithmetic outside what stdlib
  helpers expose.
- **Fuzz-tested.** Four harnesses guard invariants, not just
  happy-path behaviour. New parser / lookup code extends the
  harnesses as part of the PR.
- **Deterministic.** No hidden RNG, no clocks in the compute path.
  `CalcHistory` takes timestamps as caller-supplied strings.
