# 003 — Unit registry: two maps + lookup order

`UnitRegistry` (`src/units.cyr`) keeps **two** cstr-keyed hashmaps, not one:

- `reg_sym` (offset `+8`) — **exact-case** map. Holds each unit's canonical
  symbol (`"m"`, `"km"`, `"Hz"`, `"MW"`, `"mW"`) plus a fixed list of
  lowercase spellings of SI symbols registered with `reg_spelling` (`"hz"`,
  `"ml"`, `"kw"`, `"c"`). These match only exactly as written.
- `reg_low` (offset `+16`) — **case-insensitive** map. Holds full names and
  aliases, lowercased (`"meter"`, `"kilometre"`, `"kph"`), including the
  non-SI abbreviations whose case means nothing (`"mi"`, `"lb"`, `"gal"`,
  `"mph"`). Symbols are **not** folded into it.

`UnitRegistry_find(r, query)` resolves in a fixed order — **order matters**:

1. `NULL` → not found.
2. Exact hit in `reg_sym`.
3. Else lowercase the query (only if it has uppercase, to skip the alloc) and
   hit `reg_low`.
4. Else, if the lowercased query ends in `s` (ASCII 115) and at least three
   bytes remain without it, strip the `s` and retry `reg_low` — the plural
   fallback (`"meters"` → `"meter"`, `"tonnes"` → `"tonne"`).

A `reg_low` key that two different units claim is stored as **0** (ambiguous)
and refused, rather than answering with whichever unit registered last.
`test_units::test_lookup_policy` asserts that no built-in key is ambiguous, so
a clashing new alias fails the suite.

Both maps are the stdlib cstr-keyed open-addressing hashmap (content hash, not
pointer identity), so a user-supplied query string matches a stored literal by
bytes. Lookups are O(1) amortized.

## Why symbols are not case-folded (2.4.9)

Through 2.4.8 `_reg_index` also stored every symbol lowercased in `reg_low`.
SI prefix symbols differ only by case (m milli / M mega, p pico / P peta), and
b (bit) differs from B (byte), so folding:

- **overwrote keys**: `"mw"` held milliwatt (registered after megawatt) and
  `"kn"` kilonewton, so `MW` in any other case was 10⁹ too small and `kN`
  became a knot. `nl_parse` lowercased its whole input, so every natural-
  language query took that path.
- **resolved units abaco does not have** onto ones it does: `mHz` → MHz,
  `Mm` → mm, `Mb` → MB, `b` → B, `Cal` → cal, all with `UERR_NONE`.

And the plural strip, which ran on folded symbols too, turned `"ms"` into
metre, `"ns"` into newton and `"Ws"` into watt.

The policy now follows the standards (BIPM SI Brochure Table 7; IEEE 1541 /
IEC 80000-13): a symbol matches only as written; names and aliases match in
any case; non-SI abbreviations are aliases; and a lowercase spelling of an SI
symbol is accepted only from an explicit list, and only exactly as written,
when no other case variant of it is a different unit. Left out on purpose:
`b`, `kb`, `mb`, `gb`, … (bits), `mhz` (mHz), `mj` (mJ), `mw` (MW or mW), `p`
(pico), `t` (tesla).

## Consequences

- A new unit's symbol goes to `reg_sym` and its name to `reg_low` — `reg_add`
  does both; don't `map_set` a map directly. A new alias goes through
  `reg_alias` (any case) or, for a lowercase SI spelling, `reg_spelling`
  (exact case). Never add an alias whose case variants include another unit.
- The plural strip is byte-naive (`"gauss"` would try `"gaus"`); it only
  applies to names and aliases, so a strip that lands on nothing is a benign
  miss.
- Regression coverage: `test_units.tcyr::test_lookup_policy`,
  `test_registry_integrity`, `test_hashmap_collisions` (LOW-9b), and
  `fuzz_units` (`"<letter>s"` never resolves).
