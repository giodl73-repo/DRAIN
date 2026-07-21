# Requirements

## Scope

Repo: DRAIN

VTRACE adoption scope: derive initial repo-level requirements from `docs/vtrace/MISSION.md` and
`docs/vtrace/CONOPS.md`. These requirements describe what DRAIN must satisfy as analysis and
implementation proceed; they do not by themselves authorize implementation work — that comes
from accepted work packages. Requirements stay at contract level and assert no scores or designs.

## Requirement Table

| ID | Requirement | Parent Need / Constraint / Scenario | Rationale | Priority | Owner | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| REQ-001 | DRAIN shall maintain a documented regeneration path for the active corpus, score, and gap artifacts from public source data. | NEED-001 / CON-003 / OPS-001 | Reproducibility is the minimum condition for trusting generated claims. | must | DRAIN maintainer | inspection / command review | accepted |
| REQ-002 | DRAIN shall label every material quantity with an evidence posture (implemented, heuristic, simulated, proxy, planned, held, source-needed, confidence-limited). | NEED-002 / NEED-003 / CON-001 / CON-004 / OPS-001 / OPS-004 | Evidence labels prevent proxy or planned work from reading as proof. | must | DRAIN maintainer | artifact inspection / review | accepted |
| REQ-003 | DRAIN shall cite a declared source in `data/sources.md` for every quantity in a corpus entry, or mark it as a labelled proxy/heuristic. | NEED-001 / CON-003 / CON-004 / OPS-001 | Uncited numbers cannot be audited or regenerated; wet-weather and dry-weather bases differ. | must | data steward | citation audit / inspection | accepted |
| REQ-004 | DRAIN shall identify each element by a stable plant/interceptor/district/outfall/network identifier, not by a transient label, operator, permit nickname, or map id. | NEED-004 / CON-002 / OPS-001 | Stable physical and jurisdictional identity is required before scores, gaps, and proposals can be compared. | must | DRAIN maintainer | schema check / inspection | accepted |
| REQ-005 | DRAIN shall hold or reject any corpus or gap artifact that lacks a stable element identifier, a declared source label, or a declared scale. | NEED-004 / NEED-008 / CON-002 / CON-004 / CON-007 / OPS-001 | Mutable labels, uncited rows, and untagged scale cannot safely join across analysis stages. | must | DRAIN maintainer | gate / data inspection | accepted |
| REQ-006 | DRAIN shall calibrate its scoring rubric from observed corpus variance and correlation, and record the rubric version and rationale for each change. | NEED-002 / NEED-005 / OPS-002 | Calibration must be evidence-driven and auditable, not asserted. | must | DRAIN maintainer | calibration record / version diff | accepted |
| REQ-007 | DRAIN shall ground every capacity or adequacy claim in an explicit demand basis — peak **WetWeather** flow vs average **DryWeather** flow — and name the basis on the claim. | NEED-002 / CON-001 / OPS-003 / OPS-006 | A treatment-capacity, overflow, or adequacy claim is meaningless without stating whether it is wet-weather peak or dry-weather average. | must | operations reviewer | inspection / review | accepted |
| REQ-008 | DRAIN shall record a wastewater system that is already adequate, compliant, resilient, and resource-efficient as a valid null result rather than manufacturing a gap. | NEED-006 / CON-001 / OPS-003 | Silent scope expansion to rescue a hypothesis is forbidden. | must | DRAIN maintainer | gap-artifact inspection / review | accepted |
| REQ-009 | DRAIN shall route every promotable network or project claim through the 7-voice parliament and the 3-role editorial gate before downstream use. | NEED-005 / CON-001 / OPS-004 | DRAIN's review system is part of the evidence model, not decoration. | must | review steward | review inspection | accepted |
| REQ-010 | DRAIN shall represent treatment capacity, collection/service coverage, effluent quality, conveyance/connectivity, wet-weather resilience, CSO/SSO overflow control, pollutant removal, resource recovery, asset condition, receiving-water impact, equity/affordability, benefit-cost, and regulatory/consent-decree constraints in reviews or claim labels before a design option is promoted. | NEED-003 / NEED-005 / OPS-004 | These stakeholder lenses must remain first-class, per the mission users. | should | review steward | role review / inspection | accepted |
| REQ-011 | DRAIN shall keep its outputs framed as research, tooling, review, and conceptual design — not construction readiness, capacity/effluent validity of record, discharge-permit application, rate-case determination, consent-decree determination, or agency/utility/operator endorsement. | NEED-003 / CON-006 / OPS-004 | Scope control protects DRAIN from overclaiming public authority. | must | DRAIN maintainer | editorial review | accepted |
| REQ-012 | DRAIN shall keep implementation and VTRACE changes scoped to the DRAIN child repo until an intentional TRACKER submodule pointer update after intake. | CON-005 / OPS-005 | TRACKER is the snapshot repo; DRAIN owns implementation history. | must | DRAIN / portfolio maintainer | status / submodule review | accepted |
| REQ-013 | DRAIN shall advance VTRACE deliverables one at a time to a `.roles` review fixed point, recording dispositions and deferrals. | NEED-005 / OPS-005 | The one-at-a-time discipline keeps each artifact reviewable and traceable. | must | DRAIN maintainer | wave ledger / review notes | accepted |
| REQ-014 | DRAIN shall classify every analyzed element into exactly one tier (T1 Major Regional Wastewater System, T2 Municipal System, T3 Small Community System / Sanitation District, T4 Local/Decentralized collection district, cluster, or onsite/septic) and attach the tier's declared SLA (treatment capacity, effluent quality, overflow frequency, service coverage). | NEED-007 / CON-002 / OPS-006 | A tiered SLA system requires every element to carry a tier and a promise it is judged against. | must | DRAIN maintainer | schema check / inspection | accepted |
| REQ-015 | DRAIN shall assess each element against its tier SLA and report any tier-SLA shortfall as a gap before a market is described as adequate. | NEED-007 / NEED-002 / NEED-006 / OPS-003 / OPS-006 | Adequacy must be measured against an explicit tier promise; SLA gaps are first-class findings. | must | DRAIN maintainer | gate / gap-artifact inspection | accepted |
| REQ-016 | DRAIN shall tag every element with a scale (international/national/regional/local) and jurisdiction, interpret scores/tiers/gaps within scale, and require an explicit labelled note for any cross-scale comparison or aggregation. | NEED-008 / CON-007 / OPS-007 | The multi-scale methodology is only sound if scale is explicit and not silently mixed. | must | DRAIN maintainer | schema check / gate / review | accepted |

## Requirement Quality Checklist

- [x] Each requirement is clear.
- [x] Each requirement is feasible.
- [x] Each requirement is verifiable.
- [x] Each requirement has an owner.
- [x] Each requirement links to a mission need, constraint, or CONOPS scenario.
- [x] Each requirement avoids implementation detail unless the detail is itself required.

## Role Review Notes

| Role Lens | Requirement Impact | Disposition |
|---|---|---|
| Scope Keeper | Requirements stay at contract level; REQ-016 makes scale a hard requirement. | pass |
| Citation Auditor | Requirements introduce no new numeric claims; REQ-003 hardens citation discipline. | pass |
| Numeracy Checker | No calculations, units, scores, effluent, overflow-frequency, or cost claims. | pass |
| Wastewater-System Planner | Connectivity, tiering, and multi-scale intent preserved via REQ-014/016/010. | pass |
| Operations Officer | Initial draft left the demand basis implicit; resolved by adding REQ-007 (WetWeather vs DryWeather named on the claim). | resolved |
| Regulatory & Consent-Decree Realist | Permit, consent-decree, funding, and affordability constraints represented before promotion (REQ-010/011). | pass |
| Equity, Affordability & Receiving-Water advocates | Service coverage, receiving-water impact, and affordability required before promotion (REQ-010). | pass |

Fixed-point note: one actionable finding (demand basis implicit) was raised and applied as
REQ-007. No unresolved critical or major finding remains.

## CONOPS Trace Review

| Scenario ID | Requirements Derived |
|---|---|
| OPS-001 | REQ-001, REQ-002, REQ-003, REQ-004, REQ-005 |
| OPS-002 | REQ-006 |
| OPS-003 | REQ-007, REQ-008 |
| OPS-004 | REQ-002, REQ-009, REQ-010, REQ-011 |
| OPS-005 | REQ-012, REQ-013 |
| OPS-006 | REQ-014, REQ-015 |
| OPS-007 | REQ-005, REQ-016 |

## Deferred Requirements

| ID | Reason Deferred | Revisit Trigger |
|---|---|---|
| DEF-001 | Exact per-dimension anchors and weights. | `SPECIFICATION_BASELINE.md` and first corpus-calibration wave. |
| DEF-002 | Whether capacity/overflow scoring uses observed utility records vs permit/DMR/CWNS proxies. | `SPECIFICATION_BASELINE.md` and first corpus wave. |
| DEF-003 | Specific data-source acquisition commands and refresh cadence. | `data/sources.md` and `VERIFICATION.md`. |
| DEF-004 | Implementation interfaces (CLI, schemas, crates). | `ARCHITECTURE.md` / `INTERFACES.md` after the minimum slice. |
| DEF-005 | Whether scale is a flat tag or a nested hierarchy (a local plant within a utility within a national/international network). | `SPECIFICATION_BASELINE.md` / `INTERFACES.md`. |
