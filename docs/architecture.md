# Architecture

Abaco is a single Cyrius module set with a flat namespace. It has no
workspace, no sub-crates, no feature flags at link time — consumers
include exactly the `src/*.cyr` modules they need.

## Module layout

```
abaco
├── src/core.cyr      — Value, Unit, UnitCategory, Currency,
│                       ConversionResult, str_lower / str_upper
├── src/ntheory.cyr   — is_prime, next/prev_prime, factor, totient,
│                       fibonacci, binomial, mod_pow
├── src/dsp.cyr       — Windows, interpolation, dB, MIDI, chromagram,
│                       SIMD batch ops, samples↔ms, BPM↔Hz
├── src/eval.cyr      — Tokenizer + recursive-descent parser,
│                       Evaluator, 43+ built-in functions, variables
├── src/units.cyr     — UnitRegistry (vec + 2 hashmaps),
│                       reg_add / reg_add_inv / reg_alias,
│                       UnitRegistry_find / _convert / _list
├── src/ai.cyr        — NlParser, ParsedQuery, CalcHistory,
│                       CurrencyCache (set_rates / fetch / convert)
└── src/main.cyr      — Library entry point (includes all modules)
```

## Dependencies

```
ai.cyr     ──► eval.cyr ──► dsp.cyr ──► ntheory.cyr ──► core.cyr
               units.cyr ─► core.cyr
```

Everything depends on `core.cyr`. `ai.cyr` is the only module that
depends on the HTTP + JSON stdlib; everything else is pure compute.

## Key design decisions

### Cstrings are the default string form
Abaco operates on null-terminated byte pointers (`char*`), not the
stdlib `Str` struct. This matches how Cyrius internals already pass
strings and avoids round-tripping through `{data, len}` pairs at
every API boundary.

### No generics, no methods — prefix naming for struct-like types
Cyrius has no generics, traits, or method dispatch. Types that behave
like structs use a `Type_method` naming convention:
- `Evaluator_new` / `Evaluator_eval` / `Evaluator_set_variable`
- `UnitRegistry_new` / `UnitRegistry_find` / `UnitRegistry_convert`
- `Value_integer` / `Value_as_f64` / `Value_to_latex`
- `CalcHistory_new` / `CalcHistory_push` / `CalcHistory_get`

This is the stable contract — consumers build on these names.

### f64 as bit pattern through the public API
Cyrius treats `f64` as an i64 holding the IEEE-754 bit pattern;
arithmetic is done via `f64_add`/`f64_mul`/`f64_from`/`f64_to`. Every
abaco function that returns a "double" returns bit patterns. This is
explicit, not hidden behind a wrapper type.

### Error handling via tagged values or error-slot state
Two idioms coexist:
1. **Return tagged values** — `is_ok(r) / payload(r)` from
   `lib/tagged.cyr`, used by `ntheory::prev_prime`, `Value_as_f64`,
   and the `_nl_parse_f64` helper.
2. **Error slot on the struct** — `UnitRegistry` and `Evaluator` each
   hold an `err` field; callers check `reg_err(r)` / `eval_err(e)`
   after each operation.

The tagged form is preferred for new code; the error-slot form
exists because it was cheaper to port from the Rust `Result` model.

### Flat module namespace
Every public fn is reachable by its bare name (no `abaco::dsp::window_hann`).
This keeps call sites short and avoids a namespace layer Cyrius does
not have.

## Data flow — a unit conversion

```
1. user input            "5 km to miles"
                              │
2. NlParser (ai.cyr)          │
   nl_parse() returns         ▼
   PQ_CONVERSION tagged       ParsedQuery { kind=1, v=5.0,
                                            s1="km", s2="miles" }
                              │
3. UnitRegistry_convert       ▼
   (units.cyr)                UnitRegistry_find("km")    -> Unit ptr
                              UnitRegistry_find("miles") -> Unit ptr
                              base_val = value * factor + offset
                              result = (base_val - offset_to) / factor_to
                              │
4. Return f64 bits            ▼
                              3.10686 * f64
```

## Data flow — an expression

```
1. user input            "sqrt(144) + sin(pi/2)"
                              │
2. Evaluator_eval             ▼
   (eval.cyr)
   a. tokenize()     -> Token stream
   b. parse_expr()   -> recursive descent
                       → parse_term → parse_power → parse_unary
                       → parse_primary → call_function
   c. dispatch fn name → sqrt, sin, etc.
                              │
3. Return f64 bits            ▼
                              13.0 * f64
```

## Error categories

| Module  | Error type    | Values |
|---------|---------------|--------|
| units   | UnitErr       | UERR_NONE / UERR_UNKNOWN / UERR_INCOMPAT / UERR_CONVERT |
| eval    | EvalErr       | ERR_NONE / ERR_PARSE / ERR_INVALID / ERR_MATH / ERR_UNKNOWN_FN / ... |
| ai      | AiError       | AI_OK / AI_ERR_PARSE / AI_ERR_UNSUPPORTED / AI_ERR_CURRENCY / AI_ERR_HTTP |
| ntheory | tagged Result | Ok / Err (from `lib/tagged.cyr`) |

## Consumer layering

```
    ┌─── abacus (GUI) ─── dhvani (audio) ─── hisab (physics/high math) ───┐
    │        │                   │                        │                │
    │        └───────────────────┴────────────────────────┘                │
    │                            │                                         │
    │                         abaco                                        │
    │                            │                                         │
    │                      cyrius stdlib                                   │
    └──────────────────────────────────────────────────────────────────────┘
```

Abaco exposes a stable API that consumers depend on. It does not
reach upward into any consumer. New consumers add themselves by
depending on the `dist/abaco.cyr` bundle through their `cyrius.cyml`.
