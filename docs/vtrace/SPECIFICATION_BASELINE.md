# Specification Baseline

## Scope

Repo: DRAIN

Baseline type: target (provisional)

Baseline date: 2026-06-26

VTRACE adoption scope: define the controlled behavior DRAIN intends to build — the dimension pool, scoring scale, demand basis, corpus schema, evidence labels, tier model, and the **multi-scale model** — before architecture, interfaces, or implementation planning. Because DRAIN is greenfield, every item is `target`, not observed `current`. The dimension pool is **provisional**: dimensions and their basis are controlled here, but per-dimension anchors and rubric weights calibrate from the scored corpus (REQ-006) and are not asserted in this file. Future work packages must cite `SPEC-*` / `DIM-*` IDs instead of making unanchored changes.

## Specification Sources

| Source | Evidence | Status | Notes |
|---|---|---|---|
| `README.md` | DRAIN thesis, hypothesis, multi-scale, pipeline. | target | Public-facing repo intent. |
| `PRODUCT_PLAN.md` | Scope, non-goals, method, waves. | target | Product framing. |
| `CLAUDE.md` | House rules, multi-scale rule, pipeline, quality bar. | target | Operating constraints. |
| `docs/vtrace/MISSION.md` | `NEED-*`, `CON-*`. | current | VTRACE mission source. |
| `docs/vtrace/CONOPS.md` | `OPS-*` scenarios. | current | VTRACE scenario source. |
| `docs/vtrace/REQUIREMENTS.md` | `REQ-001..016`, `DEF-001..005`. | current | VTRACE requirement source. |
| `.roles/ROLE.md` | Parliament + editorial review lenses. | current | Review-lane source. |

## Scale Model (`SCALE-*`) (resolves NEED-008 / REQ-016)

DRAIN runs the same methodology at any scale. Every corpus element declares a scale; scores, tiers, and gaps are interpreted within scale.

| Scale | Meaning | Example governance |
|---|---|---|
| `international` | Cross-border basins, transboundary discharge, global benchmarking. | International basin bodies, treaty regimes, global benchmarking programs. |
| `national` | A single nation's wastewater systems and policy. | EPA or equivalent national regulators, national infrastructure programs. |
| `regional` | Metro/watershed utilities or multi-jurisdiction systems. | State agencies, watershed authorities, regional sewer districts. |
| `local` | A plant, interceptor, collection district, cluster, or onsite/decentralized system. | Utility operators, municipalities, sanitation districts, onsite regulators. |

| ID | Rule |
|---|---|
| SCALE-01 | Every corpus element carries a `scale` and a `market`/jurisdiction tag (REQ-016). |
| SCALE-02 | Scores, tiers, and gaps are interpreted within the element's scale; cross-scale comparison/aggregation requires an explicit labelled note (CON-007). |
| SCALE-03 | Scale may nest (a local plant within a utility within a national/international benchmark); nesting representation is provisional (DEF-005). |

## Dimension Pool (`DIM-*`)

The candidate pool DRAIN scores existing wastewater/sewer-network elements against. Each dimension is scored 0–10. Anchors and weights are **calibrated from the corpus** (REQ-006), not fixed here. "Primary basis" names where the input comes from; "Default label" is the evidence posture a fresh value carries until upgraded with a cited source (REQ-002, REQ-003).

| DIM ID | Dimension | What it measures | Primary basis | Default label |
|---|---|---|---|---|
| DIM-01 | Treatment Capacity | Treatment capacity against declared demand basis, usually MGD. | EPA CWNS, permits, utility reports | source-needed |
| DIM-02 | Collection/Service Coverage | Population/area/service connections covered by sewer or decentralized service. | CWNS, Census, utility service areas | heuristic |
| DIM-03 | Treatment Level/Effluent Quality | Effluent quality and treatment level, including mg·L⁻¹ pollutants where available. | ECHO/NPDES DMR, state permits | source-needed |
| DIM-04 | Conveyance/Connectivity | Sewer/interceptor/force-main connectivity, redundancy, and graph reach. | Utility GIS, CSO/SSO reports, network inventory | implemented |
| DIM-05 | Resilience (wet-weather + redundancy) | Wet-weather shock exposure, redundancy, storage, bypass, and recovery posture. | CWNS, permits, incident reports | heuristic |
| DIM-06 | CSO/SSO Overflow Control | Combined/sanitary overflow frequency, volume proxies, and control posture. | EPA CSO/SSO reports, consent decrees | source-needed |
| DIM-07 | Nutrient & Pollutant Removal | Nutrient, pathogen, industrial, and priority-pollutant removal posture. | DMRs, permits, TMDL/receiving-water data | source-needed |
| DIM-08 | Resource Recovery (water reuse/biosolids/energy) | Reuse, biosolids, energy recovery, heat recovery, nutrient recovery. | Utility plans, state reuse/biosolids data | heuristic |
| DIM-09 | Asset Condition/Age | Age, condition, infiltration/inflow exposure, renewal backlog. | CWNS, utility asset reports | proxy |
| DIM-10 | Environmental/Receiving-Water Impact | Receiving-water impairment, habitat, public-health, and downstream burden. | USGS, EPA/state water-quality data | heuristic |
| DIM-11 | Equity & Affordability | Service inequity, unsewered/underserved areas, ratepayer burden. | Census, utility finance, affordability data | implemented |
| DIM-12 | Benefit-Cost | Benefit per unit cost, public-health value, environmental value, affordability limits. | utility finance, public studies | heuristic |
| DIM-13 | Tier-SLA Conformance | Degree the element meets its tier's treatment-capacity, effluent-quality, overflow-frequency, and service-coverage SLA (derived; shortfall = tier-SLA gap). | Tier model + DIM-01/02/03/06 | heuristic |

Calibration note (per REQ-006, OPS-002): after the first corpus pass, low-variance or redundant dimensions are retired and informative ones promoted; the rubric version records each change. The pool above is the v0 candidate set, not a final rubric.

## Demand Basis (resolves DEF-002 minimum)

| ID | Rule |
|---|---|
| SPEC-WW-01 | Wet-weather capacity/adequacy claims use `DemandBasis::WetWeather` and identify the wet-weather basis (peak flow, storm/I&I condition, overflow condition, or permit/consent-decree scenario) when known. |
| SPEC-WW-02 | Wet-weather proxies may be used only with explicit proxy/source-needed labels; they must not be presented as observed design capacity. |
| SPEC-DW-01 | Dry-weather capacity/adequacy claims use `DemandBasis::DryWeather` and identify the average dry-weather basis when known. |
| SPEC-DW-02 | Dry-weather averages must not be used to imply wet-weather adequacy; any conversion or comparison requires an explicit labelled note. |

## System Tier Model (`T1–T4`) (resolves NEED-007 / REQ-014 / REQ-015)

DRAIN classifies each element into a four-tier hierarchy — from major regional wastewater system to local/decentralized service — with treatment capacity, effluent quality, overflow frequency, and service coverage SLA terms per tier. This is the Wastewater 2.0 analog of the portfolio tiering. Roles are typical, not strict.

| Tier | Name | Typical role | SLA promise (target) |
|---|---|---|---|
| T1 | Major Regional Wastewater System | Multi-jurisdiction regional utility, major treatment and interceptor system. | High treatment capacity; robust wet-weather management; stringent effluent quality; broad service coverage; redundancy. |
| T2 | Municipal System | City or town wastewater utility with treatment and collection service. | Adequate capacity; permit-aligned effluent; managed overflows; defined service coverage. |
| T3 | Small Community System / Sanitation District | Small utility, special district, village system, lagoon/package plant. | Basic compliant treatment; bounded overflow risk; affordable service; maintained assets. |
| T4 | Local/Decentralized (collection district, cluster, or onsite/septic) | Local collection district, cluster system, onsite/septic or decentralized treatment. | Safe sanitation, service reliability, public-health protection, maintained local discharge/soil constraints. |

Each tier's SLA is expressed over four contract terms, assessed by DIM-13:

| SLA term | Meaning | Backing dimensions |
|---|---|---|
| Treatment capacity | Capacity the tier promises vs declared demand basis. | DIM-01, DIM-05 |
| Effluent quality | Treatment level / pollutant concentration the tier must hold. | DIM-03, DIM-07 |
| Overflow frequency | CSO/SSO/back-up frequency the tier must control. | DIM-05, DIM-06 |
| Service coverage | People/area/connection coverage the tier provides. | DIM-02, DIM-11 |

SLA values per tier are **target and provisional** — exact thresholds calibrate with the rubric (REQ-006) and are not asserted here. A tier-SLA shortfall is a first-class gap (REQ-015, OPS-006).

## Controlled Specification Items

| Spec ID | Parent REQ IDs | Type | C/T/D/U | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-004 / REQ-005 | architecture | target | Every element is keyed by a stable plant/interceptor/district/outfall/network identifier; operator, permit nickname, and map id are mutable presentation fields, not keys. | schema check / inspection | OPS-001 | DRAIN maintainer | high | accepted |
| SPEC-002 | REQ-001 / REQ-003 / REQ-014 / REQ-016 | product | target | A corpus entry is one markdown file with frontmatter (id, type, scale, jurisdiction, termini, tier, sla, source rows) and a scored dimension block, regenerable from documented commands. | inspection / command review | OPS-001 | DRAIN maintainer | medium | accepted |
| SPEC-003 | REQ-002 | product | target | Every quantity carries an evidence label from {implemented, heuristic, simulated, proxy, planned, held, source-needed, confidence-limited}. | artifact inspection | OPS-001 / OPS-004 | DRAIN maintainer | medium | accepted |
| SPEC-004 | REQ-006 | product | target | The dimension pool is `DIM-01..DIM-13` scored 0–10; anchors and weights are calibrated from corpus variance and versioned, not fixed in this baseline. | calibration record / version diff | OPS-002 | DRAIN maintainer | high | accepted |
| SPEC-005 | REQ-007 | software | target | Capacity/adequacy dimensions name the demand basis (`WetWeather` vs `DryWeather`) on each claim and follow SPEC-WW-01/02 and SPEC-DW-01/02. | analysis / inspection | OPS-003 | operations reviewer | high | accepted |
| SPEC-006 | REQ-008 | product | target | An already adequate, compliant, resilient, resource-efficient wastewater system is recorded as a labelled null result; scope is not expanded to manufacture a gap. | gap-artifact inspection / review | OPS-003 | DRAIN maintainer | medium | accepted |
| SPEC-007 | REQ-009 / REQ-010 | ops | target | Promotable claims pass the 7-voice parliament and 3-role editorial gate, with capacity, coverage, effluent, conveyance, resilience, overflow, pollutant removal, recovery, asset, receiving-water, equity, benefit-cost, and regulatory/affordability lenses represented. | review inspection | OPS-004 | review steward | medium | accepted |
| SPEC-008 | REQ-011 | product | target | Outputs carry a scope boundary: research/tooling/conceptual-design only; no engineering study, treatment-plant design, discharge-permit application, rate case, consent-decree determination, or endorsement. | editorial review | OPS-004 | DRAIN maintainer | medium | accepted |
| SPEC-009 | REQ-003 | data | target | `data/sources.md` is the citation registry; every cited quantity names a registry entry, and proxies/heuristics (including permit, CWNS, ECHO, or modelled values) are labelled rather than silently treated as proof. | citation audit | OPS-001 | data steward | high | accepted |
| SPEC-010 | REQ-012 / REQ-013 | ops | target | VTRACE deliverables advance one at a time to a `.roles` fixed point; DRAIN changes stay in the child repo until an intentional TRACKER pointer update after intake. | wave ledger / status review | OPS-005 | DRAIN maintainer | low | accepted |
| SPEC-011 | REQ-014 | product | target | Every analyzed element is classified into exactly one tier (T1–T4) per the System Tier Model and carries that tier's declared SLA terms. | schema check / inspection | OPS-006 | DRAIN maintainer | high | accepted |
| SPEC-012 | REQ-015 | software | target | Tier-SLA conformance (DIM-13) is assessed per element against its tier SLA; any shortfall is recorded as a tier-SLA gap and a market is not called adequate while an unaddressed shortfall stands. | analysis / gate / inspection | OPS-003 / OPS-006 | DRAIN maintainer | high | accepted |
| SPEC-013 | REQ-016 | product | target | Every element carries a `scale` and `market`/jurisdiction tag (SCALE-01); analysis runs within a scale and any cross-scale comparison carries an explicit labelled note (SCALE-02). | schema check / gate / review | OPS-007 | DRAIN maintainer | high | accepted |

## Public Contracts

| Contract ID | Spec IDs | Surface | Compatibility Rule | Change-Control Trigger | Verification Evidence |
|---|---|---|---|---|---|
| IF-001 | SPEC-001 / SPEC-002 / SPEC-013 | corpus file (markdown + frontmatter, incl. scale/jurisdiction) | Frontmatter keys are additive; `id` immutable; `scale` from a fixed enum. | Any key rename/removal, id-semantics, or scale-enum change. | schema check (target) |
| IF-002 | SPEC-009 | `data/sources.md` (registry) | Source entries are append/annotate; ids stable. | Removing or re-pointing a source id. | citation audit (target) |
| IF-003 | SPEC-004 | rubric version record | Dimension set + weights versioned; changes recorded. | Retiring/adding a `DIM-*` or changing weights. | calibration record (target) |
| IF-004 | SPEC-011 / SPEC-012 | `tiers.toml` tier/SLA record | Tier set (T1–T4) and per-tier SLA terms are versioned; tier reassignment is recorded. | Changing a tier definition, SLA term, or an element's tier. | tier/SLA record (target) |

## Package / Language Allocation

| Spec IDs | Package / Module | Responsibility | Forbidden Responsibility | Validation Profile |
|---|---|---|---|---|
| SPEC-001 / SPEC-005 | wastewater/sewer graph kernel (future `drain-network`) | Graph model, identity, connectivity, incident capacity, diverse paths, typed demand basis. | Scoring policy, evidence labels, review logic. | L1 |
| SPEC-002 / SPEC-003 / SPEC-009 / SPEC-013 | corpus + data layer | File schema, scale/jurisdiction tags, source registry, evidence labels. | Graph math, design proposals. | L0/L1 |
| SPEC-004 | scoring layer | DIM-01..13 scoring and versioned rubric. | Tier SLA decisions without tier layer. | L1 |
| SPEC-007 / SPEC-008 | review layer (`.roles`) | Parliament/editorial gate, scope boundary. | Computing scores. | L0 |
| SPEC-011 / SPEC-012 | tier/SLA layer | Tier classification, SLA terms, tier-SLA conformance (DIM-13). | Setting calibrated SLA thresholds without rubric. | L1 |

## Nonfunctional Constraints

| Constraint ID | Parent Spec IDs | Constraint | Threshold / Rule | Verification Method | Status |
|---|---|---|---|---|---|
| SPEC-NF-001 | SPEC-002 / SPEC-004 | Reproducibility | Active corpus/score/gap artifacts regenerate from documented commands with labels and scale preserved. | command review | proposed |
| SPEC-NF-002 | SPEC-009 | No raw datasets committed | Raw/cache data is gitignored; only derived, cited artifacts are committed. | inspection | proposed |
| SPEC-NF-003 | SPEC-001 / SPEC-013 | Deterministic identity + scale | Element ids and scale tags are deterministic given source inputs. | inspection / test | proposed |

## Assumptions And Unknowns

| ID | Item | Impact | Disposition | Owner |
|---|---|---|---|---|
| SPEC-UNK-001 | Cross-scale availability for DIM-01/03/06 varies by country, state, utility, and permit regime. | May force proxy/source-needed labels on early corpus rows. | discovery → `data/sources.md` | data steward |
| SPEC-UNK-002 | Wet-weather peak capacity and dry-weather average capacity are often reported differently. | Claims may be incomparable unless `DemandBasis` is explicit. | accept risk (labelled basis) | operations reviewer |
| SPEC-UNK-003 | Benefit-cost (DIM-12) depends on funding, rate, affordability, and public-health assumptions. | Heuristic until grounded. | defer to corpus calibration | public-health economist |
| SPEC-UNK-004 | Per-tier SLA thresholds (DIM-13). | Affects conformance scoring. | defer to calibration | DRAIN maintainer |
| SPEC-UNK-005 | Whether scale nests as a hierarchy or stays a flat tag. | Affects schema + cross-scale notes. | defer (DEF-005) | DRAIN maintainer |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001 | SPEC-002, SPEC-NF-001 | covered | Regeneration path. |
| REQ-002 | SPEC-003 | covered | Evidence labels. |
| REQ-003 | SPEC-009 | covered | Citation registry. |
| REQ-004 | SPEC-001 | covered | Stable identity. |
| REQ-005 | SPEC-001, SPEC-013 | covered | Hold/reject unidentified/untagged rows. |
| REQ-006 | SPEC-004, IF-003 | covered | Calibrated rubric. |
| REQ-007 | SPEC-005, SPEC-WW-01/02, SPEC-DW-01/02 | covered | Demand basis named. |
| REQ-008 | SPEC-006 | covered | Null result. |
| REQ-009 | SPEC-007 | covered | Review gate. |
| REQ-010 | SPEC-007 | covered | Stakeholder lenses. |
| REQ-011 | SPEC-008 | covered | Scope boundary. |
| REQ-012 | SPEC-010 | covered | Child-repo scoping. |
| REQ-013 | SPEC-010 | covered | One-at-a-time VTRACE. |
| REQ-014 | SPEC-011, IF-004 | covered | Tier classification + SLA. |
| REQ-015 | SPEC-012, DIM-13 | covered | Tier-SLA gap gating. |
| REQ-016 | SPEC-013, SCALE-01..03, IF-001 | covered | Multi-scale tagging + within-scale interpretation. |

## Spec-To-Verification Coverage

| Spec ID | Verification IDs / Commands | Expected Result | Evidence Pointer | Status |
|---|---|---|---|---|
| SPEC-001..013 | future `VER-*` in `VERIFICATION.md` | Each spec has a credible check (schema, command, inspection, or review). | future `EVID-*` | planned |

## Role Review Notes

| Role Lens | Spec Impact | Disposition |
|---|---|---|
| Scope Keeper | Baseline defines controlled behavior, a candidate pool, a tier model, and the scale model; it asserts no scored network or design. | pass |
| Citation Auditor | No quantities asserted; primary bases name where inputs come from; DIM default labels enforce citation discipline. | pass |
| Numeracy Checker | Units are listed but no computed values are asserted; the system `scale` enum is distinct from the score scale. | pass |
| Operations Officer | Demand basis is controlled (`WetWeather`/`DryWeather`); wet-weather vs dry-weather ambiguity is a named unknown. | pass |
| Regulatory & Consent-Decree Realist | Initial draft made upgrades feel unconstrained; resolved by adding permit/consent-decree/affordability constraints to SPEC-007/008 and SPEC-UNK-003. | resolved |
| Public-Health & Environmental Economist | Benefit-cost (DIM-12) default label set to `heuristic`; SPEC-UNK-003 records the gap. | pass |
| Equity, Affordability & Receiving-Water advocates | Equity/affordability (DIM-11) and receiving-water impact (DIM-10) are in the pool. | pass |

Fixed-point note: one actionable finding (regulatory/affordability constraints underplayed) was raised and applied. No unresolved critical or major finding remains. Pool, SLA, and scale-nesting details are explicitly provisional; calibration and DEF-005 deferred.

## Specification Gate

Decision: pass_with_risk

Required before implementation planning:

- [x] Every accepted `REQ-*` maps to one or more `SPEC-*` IDs or a recorded deferral.
- [x] Every implementation work package can name parent `SPEC-*` IDs or discovery status.
- [x] Public contracts have owners and change-control triggers.
- [~] Unknowns are resolved, blocked, deferred, or converted to discovery work (SPEC-UNK-001..005 are discovery/defer/accept-risk).
- [x] Verification and validation methods are credible for the controlled claim.

Rationale: the baseline is coherent enough to drive trace, verification, and the review gate. Residual risk is concentrated in cross-scale data openness, wet-weather/dry-weather basis ambiguity, provisional weights/SLA thresholds, and scale-nesting representation, all deferred to the corpus calibration wave rather than blocking the minimum slice.
