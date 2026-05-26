---
name: Abaco Documentation Health
description: Living ledger of doc currency in the abaco repo — fresh / stale / read-through / archive / open-question, refreshed in place as docs are touched
type: state
---

# Documentation Health — abaco

> **Last refresh**: 2026-05-26 (2.2.3 — P(-1) hardening + convention alignment.
> Added `docs/audit/2026-05-26-audit.md` (MED-4 fixed, LOW-8 upstream),
> `docs/benchmarks.md` (3-point trend), and ADR 0004 (defer-sakshi, user-
> ratified); fuzz harnesses renamed to `.fcyr` (refs swept in SECURITY.md /
> CLAUDE.md / CI). Prior: 2026-05-26 (2.2.2 docs-depth; 2.2.1 scaffold — footer).
> | **Refresh cadence**: opportunistic — update the affected row whenever a doc
> is touched, not on a timer.
>
> **Scope**: This repo only (`abaco`) — the entire `docs/` tree plus root-level
> docs. Vendored stdlib (`lib/`, gitignored) is documented upstream in cyrius,
> not here.

This is a **ledger**, not a one-time audit. Rewrite-in-place as docs change.

---

## At a glance — 2026-05-26 inventory (2.2.3)

**33 markdown docs** (+3 since 2.2.2: `docs/audit/2026-05-26-audit.md`,
`docs/benchmarks.md`, `docs/adr/0004-error-handling-defer-sakshi.md`).

| Bucket | Count | What it means |
|---|---|---|
| ✅ **Fresh / touched this cycle** | 28 | All root docs except CoC/LICENSE; every `docs/` file except the two dated audits; 4 ADRs; the new audit + benchmarks docs |
| 🟡 **Stale — refresh in place** | 0 | — |
| 🟠 **Read-through outstanding** | 0 | — |
| 🔵 **Evergreen / dated artifact** | 5 | `CODE_OF_CONDUCT.md`, `LICENSE`, `docs/adr/template.md`, `docs/audit/2026-04-14.md`, `docs/audit/2026-05-26-audit.md` |
| 📦 **Archive — frozen by design** | 0 | None yet |
| ❓ **Open strategic question** | 0 | None |

Numbers roll up from the per-tier tables below.

---

## Tier 1 — Structural docs (root)

| File | Last touched | Status | Action |
|---|---|---|---|
| `README.md` | 2026-05-26 | ✅ Fresh | De-Rusted in 2.2.1 (Cyrius pin, `cyrius deps`/`distlib`, `dist/abaco.cyr` consumer block). |
| `CHANGELOG.md` | 2026-05-26 | ✅ Fresh | **Source of truth.** Through 2.2.1; 2.2.2 entry lands at slot close. |
| `CLAUDE.md` | 2026-05-26 | ✅ Fresh | Cyrius-native; durable rules only, state delegated to `development/state.md`. |
| `CONTRIBUTING.md` | 2026-05-26 | ✅ Fresh | Prereqs reference the `cyrius.cyml` pin + `cyrius deps`; `[lib] modules` note. |
| `SECURITY.md` | 2026-05-26 | ✅ Fresh | Supported-versions table at 2.2.x / Cyrius 6.0.x. |
| `CODE_OF_CONDUCT.md` | 2026-03-22 | 🔵 Evergreen | Standard CoC. |
| `LICENSE` | — | 🔵 Evergreen | GPL-3.0-only. |
| `VERSION` | 2026-05-26 | ✅ Fresh | Single source of truth. |
| `ROADMAP.md` | 2026-05-26 | ✅ Fresh | Completed-work record + forward pointer to `development/roadmap.md`. |
| `bench-latest.md` | 2026-05-26 | ✅ Fresh | **Refreshed under 6.0.1 this slot** (early arc baseline; `is_prime_small` 2µs→1µs). Full 3-point trend at 2.2.3. |

---

## Tier 2 — Docs root (`docs/`)

| File | Last touched | Status | Action |
|---|---|---|---|
| `docs/README.md` | 2026-05-26 | ✅ Fresh | **Index refreshed this slot** — now lists architecture/, adr/, guides/, sources, doc-health, development/. |
| `docs/architecture.md` | 2026-05-26 | ✅ Fresh | Module layout + data flow; consumer-dep note uses `dist/abaco.cyr`. |
| `docs/development.md` | 2026-05-26 | ✅ Fresh | Build/test/bench/release loop on the `cyrius.cyml` pin + `cyrius deps`. |
| `docs/mcp-tools.md` | 2026-05-26 | ✅ Fresh | **Read-through done this slot** — 5 tools mapped to their backing functions in `ai`/`eval`/`units`; implementation-status note added. |
| `docs/sources.md` | 2026-05-26 | ✅ Fresh | **Completeness audit done this slot** — added PolyBLEP, constant-power pan, envelope time constant. |
| `docs/doc-health.md` | 2026-05-26 | ✅ Fresh | This ledger. |
| `docs/benchmarks.md` | 2026-05-26 | ✅ Fresh | **New 2.2.3** — 3-point trend (baseline → 4.8.5 → 6.0.1); pointer to bench-latest.md + CSV. |

---

## Tier 3 — Architecture invariants (`docs/architecture/`)

> **New this slot (2.2.2).** Non-obvious constraints you can't derive from one
> function. Append-only numbering.

| File | Last touched | Status |
|---|---|---|
| `README.md` | 2026-05-26 | ✅ Fresh — index |
| `001-token-storage-layout.md` | 2026-05-26 | ✅ Fresh |
| `002-expression-depth-bound.md` | 2026-05-26 | ✅ Fresh |
| `003-unit-registry-hashing.md` | 2026-05-26 | ✅ Fresh |
| `004-miller-rabin-witness-bound.md` | 2026-05-26 | ✅ Fresh |

---

## Tier 4 — ADRs (`docs/adr/`)

> **New this slot (2.2.2).** Decisions document *why*, not status — re-read, not
> rotated. Never renumber.

| File | Last touched | Status |
|---|---|---|
| `README.md` | 2026-05-26 | ✅ Fresh — index |
| `template.md` | 2026-05-26 | 🔵 Evergreen — ADR shape |
| `0001-dist-bundle-distribution.md` | 2026-05-26 | ✅ Accepted |
| `0002-eval-pow-precision-fast-path.md` | 2026-05-26 | ✅ Accepted |
| `0003-deterministic-miller-rabin.md` | 2026-05-26 | ✅ Accepted |
| `0004-error-handling-defer-sakshi.md` | 2026-05-26 | ✅ Accepted — **new 2.2.3** (user-ratified defer) |

---

## Tier 5 — Guides (`docs/guides/`)

| File | Last touched | Status |
|---|---|---|
| `consuming-abaco.md` | 2026-05-26 | ✅ Fresh — **new this slot**; consumer-side `dist/abaco.cyr` wiring + API surface map |

---

## Tier 6 — Development (`docs/development/`)

> `roadmap.md` (forward plan) and `state.md` (live snapshot) are the canonical
> operational surface. CLAUDE.md delegates volatile state to `state.md`.

| File | Last touched | Status | Action |
|---|---|---|---|
| `roadmap.md` | 2026-05-26 | ✅ Fresh | 2.2.x arc (2.2.2 in progress); 2.3.x ecosystem rollout. |
| `state.md` | 2026-05-26 | ✅ Fresh | **Rotates every release.** Bump assert count + sizes at 2.2.2 close. |

---

## Tier 7 — Audits (`docs/audit/`)

Periodic, timestamped — supersede with a new dated doc, don't edit in place.

| File | Last touched | Status |
|---|---|---|
| `2026-04-14.md` | 2026-04-14 | 🔵 Dated artifact |
| `2026-05-26-audit.md` | 2026-05-26 | 🔵 Dated artifact — **new 2.2.3** (P(-1) hardening) |

Open-item trail: LOW-9b regression tests landed in 2.2.2; **MED-4 fixed** and
**LOW-8 documented (upstream)** in 2.2.3. Remaining: LOW-8 awaits a seeded hash
in the cyrius stdlib. Next full audit: before the 2.3.0 cut.

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

| # | Commitment | Trigger | Status |
|---|---|---|---|
| 1 | `docs/sources.md` completeness audit | 2.2.2 | ✅ Done 2026-05-26 |
| 2 | Benchmark surface refresh under 6.0.1 | 2.2.2 baseline / 2.2.3 trend | ✅ Done — `docs/benchmarks.md` (3-point) |
| 3 | `docs/README.md` index refresh | 2.2.2 | ✅ Done 2026-05-26 |
| 4 | `docs/mcp-tools.md` read-through vs `src/ai.cyr` | 2.2.2 | ✅ Done 2026-05-26 |
| 5 | Full security re-audit → new `docs/audit/YYYY-MM-DD-audit.md` | 2.2.3 | ✅ Done — `2026-05-26-audit.md` |
| 6 | LOW-8 seeded hashmap | upstream (cyrius stdlib) | ⏳ Recommended; adopt when stdlib ships it |
| 7 | sakshi re-examine | consumer need or stdlib fold | ⏳ Deferred — [ADR 0004](adr/0004-error-handling-defer-sakshi.md) |

---

*Initial scaffold: 2026-05-26 (2.2.1) — caught CLAUDE.md Rust-era drift, 6×
`cyrius.toml` references, stale Cyrius `4.8.x` prereqs, a 2.0.x SECURITY.md
table. First depth pass: 2026-05-26 (2.2.2) — added architecture/ + adr/ +
guides/, completed the sources citation audit, cleared the lone read-through
(mcp-tools) and the two stale rows (bench-latest, docs/README). Hardening pass:
2026-05-26 (2.2.3) — added the security audit report, benchmarks 3-point trend,
and ADR 0004 (user-ratified sakshi defer); fuzz `.fcyr` rename refs swept.
Refresh in place when docs are touched.*
