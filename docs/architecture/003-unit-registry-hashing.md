# 003 — Unit registry: two maps + lookup order

`UnitRegistry` (`src/units.cyr`) keeps **two** cstr-keyed hashmaps, not one:

- `reg_sym` (offset `+8`) — **case-sensitive** symbol map. Holds the canonical
  symbol (`"m"`, `"km"`, `"Hz"`) where case is significant (`m` vs `M`).
- `reg_low` (offset `+16`) — **lowercased-name** map. Holds full names and
  aliases, lowercased (`"meter"`, `"meters"`, `"kilometre"`).

`UnitRegistry_find(r, query)` resolves in a fixed order — **order matters**:

1. Exact hit in `reg_sym` (preserves case-sensitive symbols).
2. Else lowercase the query (only if it has uppercase, to skip the alloc) and
   hit `reg_low`.
3. Else, if the lowercased query ends in `s` (ASCII 115) and is longer than one
   char, strip the trailing `s` and retry `reg_low` — this is the naive plural
   fallback (`"meters"` → `"meter"`).

Both maps are the stdlib cstr-keyed open-addressing hashmap (content hash, not
pointer identity), so a user-supplied query string matches a stored literal by
bytes. Lookups are O(1) amortized.

Consequences:

- A new unit must be registered into the **correct** map(s): case-significant
  symbol → `reg_sym`; names/aliases → `reg_low` (lowercased). `reg_add` handles
  this; don't `map_set` a map directly.
- The plural strip is byte-naive (`"gas"` → `"ga"`); it only matters when the
  stripped form happens to be a registered unit, so collisions are benign
  misses. Regression coverage: `test_units.tcyr::test_registry_integrity` and
  `test_hashmap_collisions` (LOW-9b).
