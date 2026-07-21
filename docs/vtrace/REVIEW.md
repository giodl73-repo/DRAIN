# Review Gate

## Scope

Repo: DRAIN

Gate type: readiness (VTRACE minimum-slice planning baseline)

Decision: pass_with_risk

Date: 2026-06-26

Reviewer / lenses: DRAIN `.roles` parliament + editorial panel (simulated against committed role
files), requirements-traceability and V&V lenses.

This gate decides whether DRAIN's **planning baseline** is coherent enough to proceed to
implementation planning. It does **not** claim any implementation, scored corpus, or validated
result.

## Role Review Matrix

| Lane | Required | Reviewer / Role | Decision | Evidence / Rationale |
|---|---|---|---|---|
| Systems engineering | yes | Wastewater-System Planner + Scope Keeper | pass | MISSION→CONOPS→REQUIREMENTS→SPEC→TRACE form a coherent chain; tier + scale models integrated. |
| Requirements traceability | yes | Traceability lens | pass | `TRACE.md` maps NEED-001..008 / OPS-001..007 → REQ-001..016 → SPEC-001..013; gaps labelled. |
| V&V | yes | V&V lens | pass_with_risk | `VERIFICATION.md` methods credible; most results `pending` (greenfield). |
| Software assurance | no | — | not_required | No code yet; revisit at implementation planning. |
| Security/privacy | no | — | not_required | No data ingestion/code yet; revisit when sources/CLI exist. |
| Safety/mission impact | yes | Operations Officer + Regulatory & Consent-Decree Realist | pass | Demand basis (SPEC-WW-01 / SPEC-DW-01) and tier-SLA gating (REQ-015) control overclaim of capacity; permit/consent-decree/ratepayer assumptions must be explicit. |
| Source custody | yes | Citation Auditor + data steward | pass_with_risk | Citation + scale discipline specified (SPEC-009/013); permit/ECHO/CWNS proxy flagged (SPEC-UNK-002); no corpus sources ingested yet. |
| Configuration/change control | yes | Scope Keeper | pass | Public contracts IF-001..004 have change-control triggers; VTRACE one-at-a-time enforced. |

## Evidence Inspected

- `docs/vtrace/MISSION.md` (NEED-001..008, CON-001..007)
- `docs/vtrace/CONOPS.md` (OPS-001..007)
- `docs/vtrace/REQUIREMENTS.md` (REQ-001..016, DEF-001..005)
- `docs/vtrace/SPECIFICATION_BASELINE.md` (DIM-01..13, SCALE model, SPEC-001..013, T1–T4 tiers, IF-001..004)
- `docs/vtrace/TRACE.md` (requirement trace + honest gaps)
- `docs/vtrace/VERIFICATION.md` (VER matrix, EVID ledger)
- `.roles/` panel (7 parliament incl. regulatory/consent-decree realist, 3 editorial, 5 stakeholder, peer panel)
- `proof check .` → 0 errors; `doc whitespace inspection` clean

## Findings

| ID | Severity | Finding | Required Action | Disposition |
|---|---|---|---|---|
| FIND-001 | minor | Mission underplayed permit/consent-decree/ratepayer constraints. | Add Regulatory/consent-decree user + CON-006 boundary. | closed (MISSION stage) |
| FIND-002 | minor | Demand/constraint basis implicit in requirements. | Add REQ-007 (WetWeather vs DryWeather named). | closed (REQUIREMENTS stage) |
| FIND-003 | minor | Capacity (DIM-01) risked reading as control-free. | Regulatory/affordability review lens + SLA constraints + SPEC-007. | closed (SPEC stage) |
| FIND-004 | note | Review gate not yet exercised on a real corpus claim. | Exercise on the first corpus entry. | accepted risk |

No open critical or major findings.

## Accepted Risks

| Risk | Rationale | Owner | Revisit Trigger |
|---|---|---|---|
| Dimension weights, per-tier SLA thresholds, and scale nesting are provisional. | Calibrate from the corpus (REQ-006) and resolve DEF-005; asserting now would be unfounded. | DRAIN maintainer | First corpus-calibration wave |
| Most verification results are `pending`. | No implementation exists yet by design. | DRAIN maintainer | First implementation work package |
| permit/ECHO/CWNS proxy vs observed utility data ambiguity. | Recorded as SPEC-UNK-002; proxy/source-needed labels mitigate. | data steward | `data/sources.md` build |

## Required Follow-Up

- Add ARCHITECTURE and INTERFACES before non-trivial implementation (DEF-004).
- Author IMPLEMENTATION_PLAN + WORK_PACKAGES before writing code.
- Build `data/sources.md` and the corpus SCHEMA (incl. scale enum) before the first corpus entry.
- Resolve scale nesting (DEF-005) and observed-vs-proxy capacity/overflow (DEF-002) at the corpus wave.

## Validation Commands

```powershell
proof check .
doc whitespace inspection
```

## Result

The DRAIN planning baseline (minimum VTRACE slice: MISSION, CONOPS, REQUIREMENTS,
SPECIFICATION_BASELINE, TRACE, VERIFICATION, REVIEW) is internally coherent, fully traced, and
reviewed against the real `.roles` panel — and it carries the multi-scale model as a first-class,
traced concern. Three minor findings were closed during earlier stages; remaining risk is the
expected greenfield risk (provisional values, pending implementation evidence, permit/ECHO/CWNS proxy
ambiguity), all explicitly accepted or deferred.

**Decision: pass_with_risk.** DRAIN may proceed to implementation planning (ARCHITECTURE →
INTERFACES → IMPLEMENTATION_PLAN → WORK_PACKAGES). No public result, scored corpus, or
construction claim is authorized by this gate.
