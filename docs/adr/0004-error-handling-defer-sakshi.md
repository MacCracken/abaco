# 0004 — Error handling: enums-by-value; defer sakshi (heft vs need)

- **Status**: accepted — user-ratified 2026-05-26
- **Date**: 2026-05-26
- **Version**: 2.2.3

## Context

**sakshi** is the canonical AGNOS error/tracing/logging crate — first-party
standards name it the replacement for `thiserror`/`anyhow`/`tracing` and say
"library error types → sakshi `Error` with module-tagged variants" and "every
crate uses sakshi." By the letter of the standard, abaco *should* adopt it.

abaco today returns errors as plain enum values: `EvalErr`, `AI_ERR_*`, `UERR_*`.
No logging, no tracing — pure synchronous return values.

The 2.2.x arc asked us to *evaluate* sakshi. The evaluation is a genuine tension,
not a foregone conclusion — so it was put to the user rather than auto-decided.

## Decision

**Defer sakshi adoption** — keep enums-by-value for now. This is an explicit,
user-ratified exception to the standard, made on **heft vs need**, not on any
"leaf-library is exempt" principle (there is no such exemption in the standard):

- **Heft**: abaco ships as the self-contained `dist/abaco.cyr` bundle. Adopting
  sakshi makes it a transitive dependency that *every* consumer (hisab, dhvani,
  the Abacus app) must vendor — a real, paid-by-everyone cost.
- **Need**: there is no concrete need today. abaco emits no logs and runs no
  audit-worthy lifecycle; its errors are side-effect-free return values that
  already satisfy the hard "no panic — errors flow through return values" rule.

The cost is certain and borne now; the benefit is hypothetical. So we wait.

## Re-examine triggers (concrete, so this isn't just can-kicking)

Adopt sakshi when **either** fires:

1. **A consumer surfaces a specific need** — when a consumer of `dist/abaco.cyr`
   (hisab/dhvani/Abacus) needs structured, auditable errors or logging *from
   abaco* (e.g. the currency-fetch path must emit security-relevant events the
   consumer audits). Adopt it for that surface, not wholesale.
2. **The heft drops** — if sakshi is folded into the Cyrius stdlib distribution
   (as vani/sankoch/sandhi/niyama were), the transitive-dep cost largely
   vanishes and the standards-conformance argument wins by default.

Until one fires, the standards tension is recorded and accepted, not resolved.

## Consequences

- No new dependency now; the bundle stays self-contained and consumer-cheap.
- abaco is knowingly non-conforming on the sakshi-error point — tracked here, to
  be revisited per the triggers above (not on a timer).
- Companion convention call (same evaluation): the test layout stays flat at
  `tests/*.tcyr` rather than `tests/tcyr/` — the standards-sanctioned form for
  cross-module integration tests. Recorded here to keep the evaluation together.
