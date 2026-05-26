---
name: Abaco Documentation Health
description: Living ledger of doc currency in the abaco repo — fresh / stale / read-through / archive / open-question, refreshed in place as docs are touched
type: state
---

# Documentation Health — abaco

> **Last refresh**: 2026-05-26 (2.2.1 — Cyrius 5.7.23 → 6.0.1 upgrade + first
> slot of the 2.2.x modernization arc. Doc tree brought current to the
> patra/sigil first-party shape: CLAUDE.md rewritten Cyrius-native, root docs
> de-Rusted, `cyrius.toml` → `cyrius.cyml` references swept, and the
> `docs/development/{roadmap,state}.md` + `docs/sources.md` surfaces scaffolded.)
> | **Refresh cadence**: opportunistic — update the affected row whenever a doc
> is touched, not on a timer.
>
> **Scope**: This repo only (`abaco`) — the entire `docs/` tree plus root-level
> docs (README, CHANGELOG, CLAUDE.md, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT,
> ROADMAP, VERSION). Stdlib deps are vendored (`lib/`, gitignored) and documented
> upstream in cyrius — not audited here.

This is a **ledger**, not a one-time audit. Rewrite-in-place as docs change.

---

## At a glance — 2026-05-26 inventory (2.2.1)

**17 markdown docs** (+3 this cycle: `docs/development/roadmap.md`,
`docs/development/state.md`, `docs/sources.md`; +1 self: this file).

| Bucket | Count | What it means |
|---|---|---|
| ✅ **Fresh / touched this cycle** | 11 | README, CHANGELOG, CLAUDE.md, CONTRIBUTING, SECURITY, VERSION, ROADMAP (re-scoped to completed-archive), docs/architecture, docs/development.md, + the 3 new surfaces (roadmap/state/sources) |
| 🟡 **Stale — refresh in place** | 2 | `bench-latest.md` (pre-6.0.1 numbers); `docs/README.md` (index predates the `docs/development/` + `sources.md` additions) |
| 🟠 **Read-through outstanding** | 1 | `docs/mcp-tools.md` — last touched 2026-03-16; verify the tool surface still matches `src/ai.cyr` |
| 🔵 **Evergreen / dated artifact** | 3 | `CODE_OF_CONDUCT.md`, `LICENSE`, `docs/audit/2026-04-14.md` (timestamped audit — supersede, don't edit) |
| 📦 **Archive — frozen by design** | 0 | None yet |
| ❓ **Open strategic question** | 0 | None |

Numbers roll up from the per-tier tables below.

**Why now**: doc-health convention adopted from
[`agnosticos/docs/doc-health.md`](https://github.com/MacCracken/agnosticos/blob/main/docs/doc-health.md)
during the 2.2.1 modernization. Abaco's CHANGELOG is canonical and `state.md`
now refreshes every release, but the *aggregate* doc currency had no surface —
this file is that surface.

---

## Tier 1 — Structural docs (root)

| File | Last touched | Status | Action |
|---|---|---|---|
| `README.md` | 2026-05-26 | ✅ Fresh | De-Rusted: Cyrius pin reference, `cyrius deps`/`distlib` quick-start, consumer block now uses `dist/abaco.cyr`. |
| `CHANGELOG.md` | 2026-05-26 | ✅ Fresh | **Source of truth.** Through 2.2.1. Refreshed every release. |
| `CLAUDE.md` | 2026-05-26 | ✅ Fresh | **Rewritten Cyrius-native** this cycle (was Rust-era: cargo/clippy/tracing). Durable rules only; volatile state delegated to `docs/development/state.md`. |
| `CONTRIBUTING.md` | 2026-05-26 | ✅ Fresh | Prereqs now reference the `cyrius.cyml` pin + `cyrius deps`; module-add checklist mentions `[lib] modules`. |
| `SECURITY.md` | 2026-05-26 | ✅ Fresh | Supported-versions table refreshed to 2.2.x / Cyrius 6.0.x. |
| `CODE_OF_CONDUCT.md` | 2026-03-22 | 🔵 Evergreen | Standard CoC; no drift surface. |
| `LICENSE` | — | 🔵 Evergreen | GPL-3.0-only. |
| `VERSION` | 2026-05-26 | ✅ Fresh | Single source of truth — `2.2.1`. `cyrius.cyml` derives via `${file:VERSION}`. |
| `ROADMAP.md` | 2026-05-26 | ✅ Fresh | Re-scoped: forward pointer to `docs/development/roadmap.md` at top; body is the completed-work record. |
| `bench-latest.md` | (pre-6.0.1) | 🟡 Stale | Snapshot predates the 6.0.1 upgrade. Refresh at the 2.2.3 benchmark CSV pass. |

---

## Tier 2 — Docs root (`docs/`)

| File | Last touched | Status | Action |
|---|---|---|---|
| `docs/README.md` | 2026-04-14 | 🟡 Stale | Docs index — predates `docs/development/` + `docs/sources.md` + this file. Add the new entries at next doc touch. |
| `docs/architecture.md` | 2026-05-26 | ✅ Fresh | Consumer-dependency note updated to `dist/abaco.cyr` via `cyrius.cyml`. |
| `docs/development.md` | 2026-05-26 | ✅ Fresh | Prereqs + upgrade flow rewritten around the `cyrius.cyml` pin and `cyrius deps`; manifest tree shows `cyrius.cyml`. |
| `docs/mcp-tools.md` | 2026-03-16 | 🟠 Read-through | Verify the documented MCP tool surface still matches `src/ai.cyr`. Oldest doc in the tree. |
| `docs/sources.md` | 2026-05-26 | ✅ Fresh | **NEW.** Citations for ntheory / DSP / units / eval algorithms + constants. Completeness audit pinned to 2.2.2. |

---

## Tier 3 — Development (`docs/development/`)

> `roadmap.md` (forward plan) and `state.md` (live snapshot) are the canonical
> operational surface. CLAUDE.md delegates volatile state to `state.md`.

| File | Last touched | Status | Action |
|---|---|---|---|
| `roadmap.md` | 2026-05-26 | ✅ Fresh | **NEW.** Forward 2.2.x modernization arc + 2.3.x ecosystem rollout. |
| `state.md` | 2026-05-26 | ✅ Fresh | **NEW. Rotates every release.** Versions, artifact sizes, 452-assert test breakdown, consumers. |

---

## Tier 4 — Audits (`docs/audit/`)

Periodic, timestamped reports — supersede with a new dated doc, don't edit in place.

| File | Last touched | Status |
|---|---|---|
| `2026-04-14.md` | 2026-04-14 | 🔵 Dated artifact |

Open items from this audit (MED-4 json depth cap, LOW-8 hashmap seed, LOW-9b
regression tests) are tracked in `docs/development/roadmap.md` under 2.2.3. The
next full audit is pinned **before the 2.3.0 cut** (closeout boundary).

---

## Refresh procedure

When docs are touched:

1. Find the affected row in the relevant tier table.
2. Update **Last touched** to the new date.
3. Update **Status** if the bucket changed.
4. Update **Action** if the next step changed.
5. If a doc moved or was archived, update its row.
6. Re-anchor "Last refresh" in the header; refresh the at-a-glance table when any
   bucket count drifts by more than ~2.

Cadence is **opportunistic** (touched when other docs are touched), not periodic.

---

## What this file is NOT

- Not a substitute for [`development/state.md`](development/state.md) (live metrics + consumers).
- Not a CHANGELOG (which records what shipped, not what's stale).
- Not a TODO list (open work lives in [`development/roadmap.md`](development/roadmap.md)).
- Not a per-doc review log (this is where each doc *stands*, not the per-doc reasoning).

---

## Forward doc-policy commitments

Scheduled doc decisions, surfaced so they aren't forgotten when the trigger arrives.

| # | Commitment | Trigger | Notes |
|---|---|---|---|
| 1 | `docs/sources.md` completeness audit — every formula/constant cross-checked against a citation | 2.2.2 | Initial pass shipped 2.2.1; covers the main algorithms. |
| 2 | Benchmark surface refresh — re-run under 6.0.1, refresh `bench-latest.md` + CSV (3-point trend) | 2.2.3 | `bench-latest.md` currently pre-6.0.1. |
| 3 | `docs/README.md` index refresh — add `development/`, `sources.md`, `doc-health.md` rows | next doc touch | Keeps the index aligned with the tree. |
| 4 | `docs/mcp-tools.md` read-through vs `src/ai.cyr` | 2.2.2 (docs depth) | Oldest doc; verify tool surface. |
| 5 | Full security re-audit → new `docs/audit/YYYY-MM-DD-audit.md` | before 2.3.0 cut | Closes the open items from 2026-04-14. |

---

*Initial scaffold: 2026-05-26 (2.2.1). First inventory caught: CLAUDE.md
Rust-era drift (full rewrite), 6× `cyrius.toml` references across root + docs,
stale Cyrius `4.8.x` prereqs, and a 2.0.x SECURITY.md support table. Refresh in
place when docs are touched.*
