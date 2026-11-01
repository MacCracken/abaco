# Consuming abaco

How to depend on abaco from another Cyrius project (hisab, dhvani, the Abacus
desktop app, …). abaco ships a single self-contained bundle, `dist/abaco.cyr` —
that file *is* the public surface (see
[ADR 0001](../adr/0001-dist-bundle-distribution.md)).

## 1. Declare the dependency

In your project's `cyrius.cyml`:

```toml
[deps.abaco]
git = "https://github.com/MacCracken/abaco.git"
tag = "2.6.0"                 # pin to a released tag, never a branch
modules = ["dist/abaco.cyr"]  # the bundle is the only file you name
```

`cyrius deps` resolves it into your `lib/`.

## 2. Provide the stdlib surface

From 2.4.11 each tag ships `dist/abaco.deps`, the stdlib modules the bundle
needs; your `cyrius deps` reads it beside the bundle and vendors them, so you
need not list them. From 2.4.13 the bundle also opens with a `# Requires` block
of one `include "lib/<leaf>.cyr"` per sidecar leaf (Cyrius 6.6.18), so it
compiles when included alone; through 2.4.12 it carried no `include` lines. For
a tag before 2.4.11, or if you include `dist/abaco.cyr` without `cyrius deps`
(and before 2.4.13), list the modules in your own `[deps]`. Repeating them on a
later tag is harmless:

```toml
[deps]
stdlib = ["string", "fmt", "alloc", "vec", "str", "syscalls", "tagged",
          "hashmap", "fnptr", "math", "ganita", "io", "net", "http", "bayan"]
```

That is the 2.4.12 list. The 2.4.13 sidecar is the 12 leaves a compile proves
the bundle needs — `math string result alloc ganita fmt bayan vec hashmap str
io http` — and they reach `syscalls`, `fnptr` and `net` themselves.

(Before 2.4.11 you could drop `net` / `http` if you never touch the
currency-cache path; the sidecar always lists them, and DCE removes them from
a binary that never fetches. `bayan` is needed regardless, for
`bayan_u64_powmod` in `ntheory`.)

`tls` is never in the sidecar. Only a consumer that fetches currency rates over
HTTPS adds it, together with `-D ABACO_TLS` — see the 2.4.12 note under
[Upgrading](#upgrading), and the 2.5.0 note for what has changed since.

The bundle needs Cyrius ≥ 6.6.13, and a `-D ABACO_TLS` build ≥ 6.7.9 (2.5.0
note under [Upgrading](#upgrading)).

> **Changed at 6.2.x** — the standalone `json` and `u128` modules were folded
> into **`bayan`**, and the extended transcendentals abaco calls
> (`f64_asinh`, `f64_atan2`, `fibonacci`, `binomial`, …) moved from `math` into
> **`ganita`**. If you copied the old `[…, "json", "u128"]` list from a
> pre-2.2.5 README, `cyrius deps` will fail to resolve those two names and the
> `ganita` symbols will come up undefined at link time.

## 3. Use the API

A consumer includes the bundle (or, in-repo, the `src/` modules) and calls the
public functions directly.

```cyr
fn main() {
    alloc_init();

    # Expression evaluation
    var e = Evaluator_new();
    Evaluator_set_variable(e, "x", f64_from(7));
    var v = Evaluator_eval(e, "2 * x + 1");        # -> 15.0

    # Unit conversion
    var r = UnitRegistry_new();
    var km = UnitRegistry_convert(r, f64_from(5), "mi", "km");  # 5 mi -> km

    # Number theory
    var p = is_prime(1000000007);                  # -> 1

    # DSP math
    var db = amplitude_to_db(f64_from(1));         # -> 0.0 dBFS
    return 0;
}
```

## Surface map

| Domain | Entry points | Module |
|--------|-------------|--------|
| Expressions | `Evaluator_new`, `Evaluator_set_variable`, `Evaluator_eval`, `Evaluator_eval_partial` | `eval` |
| Units | `UnitRegistry_new`, `UnitRegistry_convert`, `UnitRegistry_find`, `UnitRegistry_list` | `units` |
| Number theory | `is_prime`, `next_prime`, `prev_prime`, `factor`, `totient`, `mod_pow`, `mod_inverse`, `crt`, `jacobi`, `isqrt`, `iroot`, `is_perfect_power`, `mobius`, `num_divisors`, `sum_divisors`, `carmichael`, `primitive_root`, `abaco_binomial` | `ntheory` |
| DSP | windows (`window_hann` … symmetric, for FIR design; `window_hann_periodic` … DFT-even, for FFT / STFT), `amplitude_to_db`, MIDI↔freq, interpolation, chromagram, batch ops — every value argument an **f64 bit pattern** (`time_constant(f64_from(10), f64_from(48000))`, not `48000`), a sample count included (`samples_to_ms(f64_from(441), f64_from(44100))`; `ms_to_samples` returns an f64 too); the plain integers are window indices and sizes (`window_hann(n, size)`), batch lengths `n`, buffer and out pointers, pitch-class numbers and the octave `freq_to_octave` returns | `dsp` |
| Values | `Value_*`, `Unit`, `ConversionResult` | `core` |
| NL / history / currency | `nl_parse`, `CalcHistory_*`, `CurrencyCache_*` (`CurrencyCache_set_ca_file` to pin a CA); over `https://` also `abaco_tls_init` — opt-in, `-D ABACO_TLS` plus `tls` in your stdlib | `ai` |

## Upgrading

Bump the `tag` and run `cyrius deps`. abaco follows SemVer (post-1.0): patch and
minor bumps are source-compatible; a major bump documents breaking changes in
[`CHANGELOG.md`](../../CHANGELOG.md) with a migration section.

### 2.6.0 — number theory, and the P(-1) review's fixes

- **New in the library:** `mod_inverse`, `crt`, `jacobi`, `isqrt`, `iroot`,
  `is_perfect_power`, `mobius`, `num_divisors`, `sum_divisors`, `carmichael`
  and `primitive_root`. The fallible ones (`mod_inverse`, `crt`, `jacobi`,
  `isqrt`, `iroot`, `sum_divisors`, `primitive_root`) return `Ok` / `Err`. A new
  name can collide with one you define yourself; rename yours if so.
- **New in the evaluator:** `modinv`, `modpow` (three arguments; a negative
  exponent inverts first), `jacobi`, `mobius`, `numdiv`, `sigma`, `carmichael`
  and `isqrt`. They take integers only; out of domain is `ABACO_ERR_MATH`.
- **`mean` results can change**, in the last bit or by much more: it is now
  correctly rounded in every case. Through 2.5.2, `mean(1e308, -1e308, 1e-300)`
  was 0, a mean of 2e121 beside ±1.8e308 terms was 0, and subnormal means were
  1 ulp off. `mean(-0, -0)` is −0.
- **`stddev` results can change** in the last bits: the corrected two-pass sum
  is within 9 ulp on clustered data, where it was up to 443,769 ulp off.
- **A call with more than 32 arguments is `ABACO_ERR_ARITY`**, not
  `ABACO_ERR_PARSE`.
- **HTTPS:** the 30 s deadline counts the name lookup. A lookup that uses it
  up is `AI_ERR_HTTP` once it returns; the lookup itself still cannot be cut
  (~32 s at worst).
- **Also:** `sinc_kernel` is 0 at the edge of a subnormal half-width and
  never NaN, and −1e-9 K is refused as below absolute zero.

### 2.5.2 — fast factorisation

- **`factor` and `totient` factor by Pollard–Brent rho** past trial division
  to 8191. Two 31-bit primes took 2.1 s through 2.5.1 and now take ~1 ms; any
  i64 takes a few ms at most. Results are unchanged, and small n run as fast
  as before.
- **The evaluator's `totient` takes any i64.** Through 2.5.1, n > 10¹² was
  `ABACO_ERR_MATH`. The cap and its global `ABACO_TOTIENT_MAX` are gone.

### 2.5.1 — Cyrius 6.7.18

abaco's source is unchanged; the bundle differs from 2.5.0 only in its version
header. What changes comes from the 6.7.18 stdlib:

- **Binaries are ~340 KB larger.** bayan 1.9.0 and later (Cyrius 6.7.13+)
  includes Unicode case-fold, category and normalisation tables for its
  Markdown reader, and dead-code elimination does not remove data. abaco needs
  bayan (`bayan_u64_powmod`, the rates JSON). A program calling
  `amplitude_to_db`, `window_hann` and the evaluator is 164,864 B on 6.7.12,
  510,096 B on 6.7.13 and 507,064 B on 6.7.18.
- **A rates body that lists a currency twice is `AI_ERR_CURRENCY`.** bayan
  1.11.1 refuses a duplicate JSON object key, compared decoded
  (`"\u0045UR"` is `"EUR"`). A repeated top-level `base` or `rates` still loads,
  and the first one wins, because abaco reads those two itself.
- **HTTPS is stricter with servers that break the handshake rules**: a
  duplicate or unsolicited extension, trailing bytes, or an ALPN the client did
  not offer is `AI_ERR_TLS`. A conformant server is unaffected.
- **Security fixes under abaco's fetch and history paths:**
  - **CYRIUS-2026-0040:** once a process had used more than 63 crypto threads,
    they shared lanes, and an on-path attacker could get a forged
    ChaCha20-Poly1305 record accepted. The 2.4.12 note said shared lanes only
    made fetches fail; that was wrong.
  - **CYRIUS-2026-0042:** chain verification ignored nameConstraints.
  - **CYRIUS-2026-0049:** `http_get` had no deadline. The plaintext loopback
    fetch now gives up after 5 s without progress or 30 s per exchange, as
    `AI_ERR_HTTP`.
  - **CYRIUS-2026-0047:** `CalcHistory_save_to_file`'s temp file was briefly
    0644 before taking a 0600 file's mode.

### 2.5.0 — Cyrius 6.7.9: abaco uses the stdlib's tan, deadline and IP check

No symbol, result or error code changed. 2.5.0 is a minor because it
**raises the minimum Cyrius version**:

- **Cyrius ≥ 6.6.13 to compile the bundle.** `tan` is now the stdlib's
  `f64_tan` (ganita 1.2.11). On an older pin the build fails naming
  `f64_tan` (checked: 6.6.12 fails, 6.6.13 compiles).
- **Cyrius ≥ 6.7.9 with `-D ABACO_TLS`.** The fetch calls `sock_set_nodelay`
  (6.7.9), `tls_set_deadline` and `tls_init_main` (6.6.13). On an older pin the
  build fails naming `sock_set_nodelay` (checked: 6.7.8 fails).
- **Results:** none moved. A 97,944-record differential is identical to
  2.4.13's, `tan` included: ganita's kernel is the same fdlibm port abaco
  carried. `dist/abaco.deps` is the same 12 leaves.
- **HTTPS (`-D ABACO_TLS`) — same bounds, same codes, less machinery:**
  - The deadline is lib/tls.cyr's own (`tls_set_deadline`), not a watchdog
    thread per fetch. A fetch now starts no thread, and the deadline holds on
    macOS and Windows too: through 2.4.13 it was Linux only, and elsewhere a
    fetch had only the 10 s per-operation timeouts. The bounds are unchanged
    with one tightening. Each step must finish within 10 s: the connect, the
    handshake, writing the request, and each response read. The whole fetch
    must finish within 30 s. The tightening: the handshake must now finish
    within 10 s *as a whole*, where through 2.4.13 each read inside it had 10 s.
    A cut is `AI_ERR_HTTP`, as before.
  - An IPv4-literal host is matched against the certificate's IP addresses only
    by lib/tls.cyr itself (Cyrius 6.6.13). abaco's own re-check of the leaf is
    gone, and the verdicts are the same.
  - `abaco_tls_init()` warms through `tls_init_main()`. It is still required,
    still main-thread only, and now also returns `AI_ERR_TLS` when the libssl
    backend is selected, where warming would load libssl.
  - `TCP_NODELAY` is set on macOS and Windows too; it was Linux only.
  - The 2.4.12 notes on watchdog threads no longer apply: they took no crypto
    lane, and the first one switched the allocator to its locked mode. abaco
    calls no `thread_create` now.

### 2.4.13 — Cyrius 6.6.20: eight functions more accurate, a smaller sidecar

No symbol changed and no error code changed outside the opt-in HTTPS path.
Re-vendor (`cyrius deps`) and check the rows that apply to you:

- **Results move in eight evaluator functions:** `asin`, `sinh`, `cosh`,
  `tanh`, `asinh`, `acosh`, `atanh`, `atan2` (ganita 1.2.14, compiled with
  abaco's pin — your own pin's ganita decides what *your* build computes).
  Every result is now within 2 ulp. The worst old errors were ~1e-8 relative
  just above each function's small-argument cutoff (`asinh(1.47e-8)` was
  99,566,017 ulp off), and up to 495 ulp for `sinh` / `cosh` between 709 and
  710.48. `atan2` follows C99 for signed zeros: `atan2(-0, -1)` is now −π (was
  +π), `atan2(0, -0)` π (was 0), `atan2(-0, 0)` −0 (was +0). Nothing else in
  a 97,944-record differential moved. If you snapshot calculator output or
  compare it bit for bit, expect these rows to change.
- **`dist/abaco.deps` lists 12 leaves, not 15.** `syscalls`, `tagged`, `fnptr`
  and `net` left it and `result` joined (Cyrius 6.6.18 derives the sidecar from
  what the bundle needs to compile). Followed transitively, abaco no longer
  brings you `tagged` or `boxed`. If your code uses `Option` / `tagged_*` /
  `boxed_*` and you never declared `tagged`, add it to your `[deps] stdlib`.
  dhvani and jalwa declare `tagged`, `fnptr` and `syscalls` already (jalwa
  also `net`).
- **The bundle includes its own stdlib** (a `# Requires` block, above). If you
  include those files too, nothing changes: include-once is keyed on the path.
- **`-D ABACO_TLS` builds:** need Cyrius ≥ 6.6.13 (`abaco_tls_init` no longer
  calls `_xp_init`, which sigil 3.13.6 removed). A peer that sends
  ChangeCipherSpec records out of place, a fatal alert after the handshake, or
  an oversized record now gives **`AI_ERR_TLS`** at once. The CCS flood
  was `AI_ERR_HTTP` at the fetch deadline, and the other two `AI_ERR_HTTP`.
- **Fixed:** a refused allocation while parsing a currency response took
  SIGSEGV with bayan ≥ 1.5.10; it is `AI_ERR_CURRENCY`.

### 2.4.12 — HTTPS currency fetch (opt-in), and one error code for `https://`

Nothing changes for a consumer that never calls `CurrencyCache_fetch` — dhvani
and jalwa included: without `-D ABACO_TLS` the bundle compiles no TLS code (a
DSP-only DCE binary grows 136 B, string data the unused parsers leave
behind), and `dist/abaco.deps` is the same 15 leaves.

- **Changed:** an `https://` base URL in a build **without** `-D ABACO_TLS` is
  now refused with **`AI_ERR_TLS` (5)**, "built without TLS", before any
  lookup. Through 2.4.11 it was `AI_ERR_HTTP` (that build had no TLS client,
  so it never connected). `AI_ERR_TLS` is appended to `AiError`; every earlier
  value is unchanged — add it if you map codes to messages. A *malformed*
  `https://` URL is still `AI_ERR_HTTP`, now under a stricter grammar: the host
  is 1–253 bytes of letters, digits, `-` and `.` (no userinfo, IPv6 literal,
  `_` or `%`), the port 1–65535, and the path visible ASCII with no space or
  `#`.
- **New, opt-in: HTTPS.** To fetch rates from an `https://` endpoint:
  1. add `"tls"` to your `[deps] stdlib` and run `cyrius deps`;
  2. build every unit that includes the bundle with `-D ABACO_TLS` (or put
     `#define ABACO_TLS` before the include);
  3. call **`abaco_tls_init()` once, on the main thread, at startup, before
     any thread fetches** — or uses `lib/tls.cyr` itself. It returns 0, or
     `AI_ERR_TLS` when called off the main thread (it then touches nothing)
     or when the build cannot do HTTPS. Until it has run, every `https://`
     fetch is `AI_ERR_TLS`. Why: Cyrius
     6.6.12's TLS stack builds its crypto tables lazily and without locks — two
     threads whose first handshakes overlap break every later handshake in the
     process, and a first handshake on a worker crashes the main thread's next
     one. `abaco_tls_init()` builds them all on the main thread (sigil's
     prewarm contract), and the test suite checks that no fetch after it
     builds another;
  4. optionally `CurrencyCache_set_ca_file(c, path)`: trust only the PEM
     bundle at `path` instead of the system store (the path is copied; the
     file is read at each fetch, into a buffer the cache keeps and reuses).
     Missing, unreadable, empty, no `BEGIN CERTIFICATE` block, or more than
     300 certificates (lib/tls.cyr's `TLS_CA_MAX_ROOTS`, which refuses a
     larger bundle whole) → `AI_ERR_TLS` before any socket; certificates that
     do not parse → `AI_ERR_TLS` at the handshake. Never a fallback to the
     system store. Path 0 restores the system store.
- **What is verified, fail closed:** the chain to a trusted root, and the host
  against the certificate's subjectAltName: a DNS name against its DNS names,
  an IPv4 literal against its IP addresses **only** — never a DNS name that
  spells the address, wildcard or not (RFC 9525 §6.3; Cyrius 6.6.12's
  lib/tls.cyr accepts those, so abaco checks the leaf itself). The libssl
  backend is **refused** — at build time under `-D CYRIUS_TLS_LIBSSL`, at
  runtime unless `tls_get_backend()` is `TLS_BACKEND_NATIVE` — because in
  Cyrius 6.6.12 it never checks the host name. Do not switch backends while
  fetches run.
- **TLS 1.3 only:** a rates server that offers only TLS 1.2 is refused with
  `AI_ERR_TLS` (the native client offers 1.3 unless pinned to 1.2, and abaco
  does not pin it).
- **What the response must be:** a status line `HTTP/1.x NNN`, a
  `Content-Length` (required — the TLS layer reports a fatal alert as a clean
  end of stream, so the length is what proves the body whole) equal to the
  body's size, no `Transfer-Encoding`, at most 64 KB, in at most 32 TLS reads.
  Reading stops at `Content-Length`: extra bytes in the same read are refused,
  bytes the server sends later are never read. A non-200 (3xx included — no
  redirect is followed) is `AI_ERR_HTTP`; a body that is not a rates object is
  `AI_ERR_CURRENCY`, as before.
- **How long:** 10 s per socket operation, and 30 s for connect, handshake
  and response together; name resolution comes first, with lib/net.cyr's own
  bound (2 tries, 2 s per read). The 30 s holds against a peer, or anyone on
  the path, that keeps every read just under 10 s — a byte at a time, or
  plaintext ChangeCipherSpec records, which the TLS layer skips without limit:
  on Linux each fetch runs under a watchdog thread that cuts the socket at the
  deadline. On other targets only the per-operation timeouts apply. Either way
  the calling thread is blocked meanwhile: do not fetch on a UI thread.
- **Errors:** `AI_ERR_TLS` — the TLS layer refused: chain or name, a malformed
  or undecryptable record, an alert in place of a handshake message, the CA
  file, or HTTPS unavailable (above). `AI_ERR_HTTP` — the network or HTTP
  failed: the URL, name resolution (IPv4 only), connect, a reset, an early end
  of stream or a timeout at any stage (handshake included), the 30 s deadline,
  the status, or the framing. So `AI_ERR_HTTP` is worth a later retry, and
  `AI_ERR_TLS` is a configuration or security problem worth reporting. (One
  overlap, from lib/tls.cyr: a record longer than TLS allows reads as an I/O
  failure, `AI_ERR_HTTP`.)
- **Threads:** after `abaco_tls_init()` any number of threads may fetch at
  once, each with its own `CurrencyCache` (a cache is not locked, and must
  not fetch on two threads at once). Read the fetch's return value, not
  `ai_last_err()`: that slot is one process-wide global. The TLS stack gives
  each thread that ever uses it a crypto lane, and after 63 such threads in a
  process's lifetime lanes are shared and concurrent fetches can fail (closed,
  never accepting a bad certificate): fetch from a reused thread or pool.
  (⚠ Corrected at 2.5.1: shared lanes could also let a forged
  ChaCha20-Poly1305 record through, CYRIUS-2026-0040, fixed in Cyrius 6.7.18
  — see the 2.5.1 note.) The
  watchdog threads never touch TLS, so they take no lane; each lives as long
  as its fetch (2 MiB of stack address space, unmapped as it exits, nothing
  kept on the heap). The first one switches Cyrius's allocator to its locked,
  thread-safe mode for the rest of the process, as any `thread_create` does.
- **Cost** (x86_64, Cyrius 6.6.12), measured on a DSP-only consumer
  (`amplitude_to_db` + `window_hann`, DCE): 93,656 B without `tls` (93,520 B
  at 2.4.11); 557,360 B with `tls` listed, whether or not anything fetches
  (sigil's static tables); 606,568 B with `tls` and `-D ABACO_TLS` but no
  fetch (the connect hook is passed by address, so DCE keeps it and the
  certificate parser it reaches); 897,408 B for a consumer that fetches.
  Listing `tls` also adds ~0.85 s to every compile (590 → 1,434 ms) and two
  warnings from `lib/sigil.cyr` to every build (the stdlib's, not abaco's).
  `abaco_tls_init()`: ~0.5 s and ~2.6 MB once. A fetch: ~30 ms against
  OpenSSL's `s_server` on loopback, ~0.5 MB of heap kept (the allocator does
  not free: ~268 KB of it is the system store, which lib/tls.cyr re-parses on
  every connect even when a CA file replaces it), ~1.5 MB at worst. A CA file
  adds about 1.5× its size to every fetch (decoded and parsed roots), and the
  cache's read buffer once — it grows by doubling, so up to ~3× the file, and
  is reused by every later fetch through that cache (measured with 300 copies
  of a 611-byte CA, 183 KB: 1,297,288 B for the first fetch, 777,040 B each
  after, against 486,824 B with no CA file). Fetch through one long-lived
  cache rather than a new one each time.
- Also: the plaintext loopback path no longer dereferences a null response
  when `http_get` cannot allocate (`AI_ERR_HTTP`).

### 2.4.11 — periodic windows (additive), and literals correctly rounded

- **New:** `window_hann_periodic`, `window_hamming_periodic`,
  `window_blackman_periodic`, `window_kaiser_periodic` and
  `window_kaiser_periodic_fill` — the periodic (DFT-even) form, denominator
  `size` (scipy's `fftbins=True`), for FFT frames and STFT. Hann and Hamming
  overlap-add to a constant at hop `size / 2` when `size` is even, Blackman at
  `size / 3` when 3 divides `size`; the symmetric forms do not, and Kaiser has
  no hop at which it does. Arguments as for the symmetric ones: `n` and `size`
  are plain integers, Kaiser's β an f64.
- If you built a periodic window by hand as `window_hann(n, size + 1)` for
  `n < size`, the new function gives the same bits for `size ≥ 2`; at
  `size == 1` it is `[1]`, where that recipe gave the first sample of a
  2-point window (0 for Hann, 0.08 for Hamming).
- **Nothing moves:** `window_hann`, `window_hamming`, `window_blackman`,
  `window_kaiser` and `window_kaiser_fill` stay the symmetric form, for FIR
  design, and return the same bits as 2.4.10.
- **`dist/abaco.deps` ships at the tag.** Your `cyrius deps` now vendors the 15
  stdlib modules abaco needs on its own; a `[deps] stdlib` list that already
  names them changes nothing. abaco's test-only modules (`assert`, `bench`,
  `args`) are not in it.
- **Literals** in an expression are now correctly rounded at any length. A
  literal on or within a hair of the midpoint between two doubles can come out
  1 ulp different from 2.4.10 — 2.4.10 was the one that was wrong. The edge
  cases are the same rule: the exact midpoint between `DBL_MAX` and 2^1024 is
  now +Inf, anything above 2^-1075 is now the smallest subnormal. A literal
  off the fast path (an exponent past ±22, or more than ~15 significant
  digits) costs ~40–50 ns more per literal (`1.602176634e-19`: ~0.48 µs per
  evaluation).

### 2.4.10 — one new error code, and a few more correct last bits

- **`ABACO_ERR_ARITY` (7)** is new: a known builtin with the wrong number of
  arguments (`sin(1, 2)`, `max(1)`, `mean()`). It used to be
  `ABACO_ERR_UNKNOWN_FN`. If you map error codes to messages, add one; every
  other code keeps its value.
- **Below absolute zero is refused** with `UERR_CONVERT` (−300 C, −1 K, −500 F),
  including a conversion to the same unit.
- **Results that move in the last bits**, all toward correct: `tan` (now
  ≤ 1 ulp), `factorial` / `n!` (now exact for n ≤ 170), `mean` (now correctly
  rounded, and no overflow), `stddev` (no overflow or underflow), subnormal
  and > 18-digit literals, and the window functions (now bit-exact symmetric,
  values within a few ulp of before). dhvani and jalwa call no window function
  through abaco, so nothing there moves.
- New units `short_ton` and `long_ton`; bare `ton` is still the metric tonne.

### 2.4.9 — a full audit: re-vendor, then check the rows that apply to you

2.4.9 fixes what a project-wide audit found (`docs/audit/2026-09-30-audit.md`).
Re-vendor in any case: **every `batch_*` op wrote one f64 past `dst` (and read
past `src`) when `n` was odd** — heap corruption with no error — and
`batch_scale` / `batch_mac` leaked `n * 8` bytes per call. Then:

**dhvani (and jalwa through it).**
- `time_constant` takes the sample rate as an f64: pass `f64_from(sr)`. dhvani's
  compressor and limiter pass the integer, which reads as a subnormal ~1e-319 Hz,
  so every attack and release collapsed to the one-sample `exp(-1)` = 0.368. From
  2.4.9 that mistake answers **NaN** instead of a plausible coefficient, so fix
  the call sites (`compressor.cyr`, `limiter.cyr`) before you re-vendor.
- `amplitude_to_db(NaN)` is NaN, not -Inf. A buffer with a NaN in it no longer
  reports silence from its RMS (`dynamics.cyr`).
- The dB constants are correctly rounded: `amplitude_to_db(1000)` is 60, not
  59.999999999999986. Other dB results move by an ulp or two.
- `f64_round_half_away(0.49999999999999994)` is 0 (was 1) and odd integers
  ≥ 2^52 stay put; no dhvani call site reaches either input.
- (The 2.4.8 note told dhvani to re-vendor for `DSP_C0_FREQ`; dhvani does not
  call the pitch-class functions, so that line did not apply to it.)

**Units** — results and lookups change:
- Factors are exact to the definition (1 lb = 0.45359237 kg, US gallon =
  3.785411784 L, …): US/imperial results move in the 7th significant digit,
  `cup → tbsp` is 16 and `torr` and `mmHg` differ.
- **Symbols match only as written.** `MW` / `mW`, `kN` / `kn`, `Mm` / `mm` are
  different units, and a symbol abaco does not register is `UERR_UNKNOWN` rather
  than its case-folded neighbour: `mHz`, `Mb`, `Gb`, `kb`, `b` (bits), `Cal`,
  `ML`. Lowercase `kb` / `mb` / `gb` no longer mean bytes — use `kB` / `MB` /
  `GB` or the names. A fixed list of lowercase spellings still works (`hz`,
  `khz`, `ghz`, `ml`, `l`, `kw`, `kwh`, `kpa`, `c`, `f`, `k`), as do names and
  aliases in any case (`Kilometers`, `LBS`, `MPH`).
- `st` is the stone; the `st` → semitone alias is gone (it was dead for the
  lowercase form anyway). `ms`, `ns`, `ks`, `Ws` are unknown (were metre,
  newton, kelvin, watt). `metre`, `kilometre`, `centimetre`, `millimetre` and
  `tonne` resolve.
- `L/100km` and other reciprocal units refuse a zero, negative, non-finite or
  overflowing amount with `UERR_CONVERT`. A `NULL` unit is `UERR_UNKNOWN` (was a
  SIGSEGV). `category_from_str` accepts `category_name`'s own spellings.

**Natural language** — `nl_parse` keeps the case of unit words and
expressions (keywords still match in any case), so `"convert 5 MW to kW"` is
5000. Evaluator names are case-sensitive, so `"what is SQRT(16)"` is now an
unknown function where the old lowercasing hid it. One leading verb is stripped
before every form, so `"convert 100 usd to eur"` is a currency query; one
trailing `?` or `.` is dropped; U+2212 `−` becomes `-`. A currency query is
exactly four words, and every number is read strictly and correctly rounded
(`1.5.3`, `1e` are refused).

**Evaluator.**
- A byte outside the grammar is `ABACO_ERR_PARSE` (`?`, `[`, `²`, `×`, U+2212
  in an expression): through 2.4.8 it was skipped, so `−5+3` (U+2212) was 8
  and `[2+3]*2` was 8, with no error.
- `1.2.3` is a parse error (it was 1.23). An exponent needs a digit, so `1e`
  and `2e` are 1 and 2 times the constant e (`1e` was 1). An identifier longer
  than 4096 bytes is a parse error.
- Integer functions refuse a fraction: `3.9!`, `factorial(2.5)`, `gcd(7.5, 5)`
  are `ABACO_ERR_MATH`, and `isprime(2.9)` is 0 (they truncated).
- `binomial` / `choose` give the exact value whenever it fits in i64:
  `C(62, 31)` and `C(66, 33)` are values (2.4.8 refused them with
  `ABACO_ERR_MATH`; before that ganita's `-1` came through as a result).
- More exact answers, so last bits change: integer powers are double-double
  (`10^k` equals the literal `1ek`); `log(1000)` is 3; `%` is exact for every
  finite pair (`2^60 % 10` is 6, was 0) and `5 % Inf` is 5 (was NaN).
- NaN propagates: `sign(NaN)`, `min`/`max`/`atan2` with a NaN argument and
  `median` with any NaN argument are NaN.
- An error inside parentheses or an argument keeps its code when more follows
  it: `2*(1/0+1)` is `ABACO_ERR_DIV_ZERO`, `(171!+1)` `ABACO_ERR_MATH` and
  `(x+1)*2` `ABACO_ERR_UNKNOWN_VAR` (all were `PARSE`).
- `Evaluator_eval_partial` re-parses at most a fixed few times (once, plus a
  bounded back-off out of a call still being typed: `2+max(7` previews 2) and
  returns a semantic error (division by zero, domain) as is; it used to retry
  once per token.
- **Threads:** one `Evaluator` per thread is safe (each owns its scratch
  arena); do not share one `Evaluator` between threads at the same time.
- Memory: each evaluator reuses one scratch arena, so evaluation stops growing
  the heap (16-33 KB per call before). `Evaluator_set_variable` copies the name.

**Number theory** — `factor`, `totient` and `next_prime` return for every
i64 (near 2^63 they never did); `next_prime(n)` is 0 when no i64 prime is
above n (n ≥ 2^63 − 25); `totient(n)` is 0 for n ≤ 0; `mod_pow` honours a
negative exponent as the u64 bit pattern instead of returning 1; `is_prime`
allocates nothing.

**DSP, other** — `batch_fmadd` is unfused on every target (aarch64 was
fused, so the two disagreed); `batch_sum` is compensated (Neumaier); windows
answer 0 outside `[0, size)` and 1 for `size == 1`; Kaiser windows are right
for β above ~20; `amplitude_to_dbfs` takes `|x|`; `f64_rms2` no longer
overflows above 1e154; `sinc_kernel` validates `half_width`.

**History and currency** — `CalcHistory` is a real ring buffer: O(1) push,
`CalcHistory_get` out of range is 0, a capacity ≤ 0 stores nothing. `from_json`
refuses anything but a whole, well-formed array and then leaves the history
unchanged; it decodes every JSON escape, and `to_json` escapes every control
byte. `save_to_file` is atomic, and save and load share a 16 MiB bound (a
history over 64 KiB used to save and then fail to load). The history keeps your
string pointers — keep them alive. `CurrencyCache_convert` refuses a
non-finite rate or result, or a rate that is not > 0; the loader reads `base` from the top
level only, wants a flat `rates` object and tolerates CRLF bodies;
`CurrencyCache_new` copies the URL.

### 2.4.8 — pitch classes move; two functions now report errors

- **`freq_to_pitch_class` / `freq_to_octave` answer differently for off-centre
  tones** — correctly now. The C0 reference was encoded as 16.703125 Hz rather
  than 16.3516 Hz, so every result was 0.368 semitone flat and a tone more than
  ~0.13 semitone flat of a note came back as the note below (435 Hz as G#, not
  A). In-tune notes are unchanged. If you compensated for the old skew — a
  tolerance, an offset — remove it. (This said dhvani should re-vendor for it;
  dhvani does not call these two functions — see 2.4.9.)
- **`binomial` / `choose` and `lcm` set `ABACO_ERR_MATH`** where they used to
  return a value with no error: `binomial` for a negative argument or a result
  ganita cannot hold (it answered -1), and `lcm` for a result past i64 (it
  wrapped). ganita also refuses some results that fit — any C(n, k) above
  `i64_MAX / k`, from C(62, 31) — so those are errors too until ganita lifts
  the limit. Every other result is unchanged.

### 2.4.7 — re-vendor, whatever your Cyrius pin

The 2.4.6 bundle gives wrong answers — and can hang or trap — when you compile
it with Cyrius 6.6.8 or later, because that toolchain changed what `f64_to`
returns for NaN, ±Inf and |x| ≥ 2^63: it saturates now, where x86 used to
answer `i64_MIN` for all of them. `nextprime(1e300)` never returns,
`lcm(-1e300, 1e300)` dies with SIGFPE on x86, `(-1)^(2^63)` is -1 and
`(sqrt(-1))!` is 1. 2.4.7 is correct under either convention. On aarch64, where
`f64_to` has always saturated, these were live at every pin — except that
aarch64's integer divide does not trap, so there `lcm` returned a wrong value
(-9.22e18) with no error rather than a SIGFPE.

One behaviour change to handle: `gcd`, `lcm`, `binomial` and `choose` now set
`ABACO_ERR_MATH` for a NaN, ±Inf or out-of-range argument (|x| ≥ 2^63, which
for `gcd` / `lcm` includes -(2^63)), where they used to return garbage with no
error. In-range results are unchanged. And `CurrencyCache_fetch` refuses a base
URL whose host only *starts* with `localhost` or `127.0.0.1`
(`http://localhost.example.net`) with `AI_ERR_HTTP`.

Two further changes come from the toolchain, not the bundle, so they follow
your own pin: from Cyrius 6.6.9, `sin` / `cos` are within 1 ulp for every
argument (x86 used to answer `sin(1e300) = 1e300`), and the evaluator's `tan`,
their quotient, within ~2 ulp; below 6.6.9 the pan, crossfade and sinc helpers
inherit the old large-argument error too. From 6.6.8 unary minus on zero gives
-0, so `(-0)^(-1)` is -Inf as C99 requires. Pin the toolchain in abaco's own
`cyrius.cyml` to match the one its gate runs.

### 2.4.0 — larger expressions, correctly-rounded literals

Two user-visible changes, both improvements — nothing to migrate:

- **Expressions up to 1024 tokens** parse (was 512). Costs 16 KB per evaluation
  instead of 8 KB. Nothing that parsed before stops parsing.
- **Decimal literals are correctly rounded.** `1.7976931348623157e308` (DBL_MAX)
  now parses to DBL_MAX rather than `+Inf`, subnormals like `1e-320` come out
  right, and 99.74% of a 5000-literal corpus matches the correctly-rounded
  reference (worst case 1 ulp). If you were compensating for the old parser's
  error, stop. (Corrected at 2.4.9: not every subnormal is bit-exact — a few are
  rounded twice, and digits past the 18th are dropped without a sticky bit, so
  the residual goes both ways. It stayed within 1 ulp until 2.4.11, which
  rounds every literal correctly; see its note.)

### 2.3.4 — one rename you may need to act on

2.3.4 renamed the bundle's `f64_round` to **`f64_round_half_away`**, because
Cyrius 6.5.x turned `f64_round` into a compiler builtin and a library may no
longer define it. This is the one source-visible change in an otherwise
compatible patch — it was unavoidable, and it is called out here because the
failure mode is silent rather than a compile error:

| You call | Before 2.3.4 | From 2.3.4 |
|----------|--------------|------------|
| `f64_round(x)` | abaco's — ties **away from zero** (`2.5 → 3`) | the 6.5.x builtin — ties to **even** (`2.5 → 2`) |
| `f64_round_half_away(x)` | — | abaco's — ties **away from zero** (`2.5 → 3`) |

Your code still compiles either way; only the tie-breaking changes. If you
depend on ties-away-from-zero (the evaluator's user-facing `round()` rule),
switch the call to `f64_round_half_away`. If you want the IEEE-754 default,
keep calling `f64_round` and you now get it from the toolchain.

Because consumers vendor the bundle rather than link it, this only takes effect
when you re-vendor — and 2.3.4 requires a re-vendor regardless (`dist/abaco.cyr`
is not byte-identical to 2.3.3).
