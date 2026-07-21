# Mission

## Scope

Repo: DRAIN

VTRACE adoption scope: establish the mission baseline for DRAIN before creating requirements, specification baselines, trace rows, or work packages. This file is the leftmost VTRACE artifact for the repo and anchors later `REQ-*`, `SPEC-*`, `WP-*`, verification, and validation records. DRAIN is greenfield and code-free: this mission defines intent ahead of any implementation, and any implementation must trace back to the needs and constraints below.

## Mission Need

| ID | Need | Success Criteria | Status |
|---|---|---|---|
| NEED-001 | DRAIN shall turn public wastewater/sewer data (e.g. EPA Clean Watersheds Needs Survey (CWNS), EPA ECHO/NPDES discharge monitoring (DMR), EPA CSO/SSO reports, Census of Governments / utility finance, USGS receiving-water data, state environmental agency discharge permits) into a reproducible scored corpus of existing wastewater/sewer-network elements. | A maintainer can regenerate the active corpus, score, and gap artifacts from documented commands, with source/proxy/heuristic labels preserved. | accepted |
| NEED-002 | DRAIN shall identify and explain wastewater/sewer gaps — treatment capacity, collection/service coverage, effluent quality, conveyance/connectivity, wet-weather resilience, CSO/SSO overflow control, nutrient/pollutant removal, resource recovery, asset condition, receiving-water impact, equity/affordability, tier-SLA shortfalls — without overstating the evidence or hiding the demand basis. | Every material claim is tied to a data artifact, command, source label, confidence label, review record, scale, and declared demand basis where capacity or adequacy is asserted. | accepted |
| NEED-003 | DRAIN shall convert analysis into defensible conceptual Wastewater 2.0 upgrade options, not engineering studies, treatment-plant designs, discharge-permit applications, rate cases, consent-decree determinations, or advocacy briefs. | Proposed projects and feature packages are labelled implemented, heuristic, simulated, planned, held, or deprecated, with the demand basis (peak wet-weather flow vs average dry-weather flow) and economic basis labelled before publication. | accepted |
| NEED-004 | DRAIN shall keep network identity stable as analysis moves from raw plants/interceptors/districts/overflows/outfalls to scored networks, gap regions, and design proposals. | Element-bearing artifacts join through a stable plant/interceptor/district/outfall/network identifier rather than a transient label, operator, permit nickname, or map id. | accepted |
| NEED-005 | DRAIN shall expose wastewater/sewer tradeoffs through adversarial review roles instead of hiding them behind a single score. | Parliament and editorial reviews can change claims, labels, next evidence steps, or promotion status. | accepted |
| NEED-006 | DRAIN shall report a rigorous null result as a valid finding. | When the scored corpus shows a wastewater system is already adequate, compliant, resilient, and resource-efficient, the artifacts say so rather than manufacturing a gap. | accepted |
| NEED-007 | DRAIN shall classify each element into a four-tier hierarchy (T1 Major Regional Wastewater System, T2 Municipal System, T3 Small Community System / Sanitation District, T4 Local / Decentralized (collection district, cluster, or onsite/septic)) and define treatment capacity, effluent quality, overflow frequency, and service coverage SLAs per tier, so that "is wastewater service adequate here?" is answered against an explicit tier promise. | Every analyzed element carries a tier and a declared SLA, and adequacy claims are made against the tier SLA rather than an unstated baseline. | accepted |
| NEED-008 | DRAIN shall apply the same methodology at multiple scales — international (cross-border basins, transboundary discharge, global benchmarking), national, regional (metro/watershed utilities), and local (a plant, an interceptor, a collection district, an onsite/decentralized cluster) — with every element tagged by scale and jurisdiction, and analysis runnable at a chosen scale. | Every corpus element declares a scale; scores, tiers, and gaps are interpreted within scale; a gap run can target a single scale without cross-scale leakage. | accepted |

## Users

| User | Need | Success Signal |
|---|---|---|
| DRAIN maintainer | Know which commands, artifacts, and review gates define the current truthful repo state at a given scale. | A clean validation bundle runs and the resulting artifacts match the documented claims and declared scale. |
| Wastewater / utility analyst | Inspect scored networks, gaps, and evidence labels without reverse-engineering the implementation. | Scores, gap maps, and reports cite their source surfaces, confidence posture, and scale. |
| Utility / watershed planner | Understand why a network, tier, or project is supported, held, or downgraded. | Each claim names the data, scenario, role review, scale, and next evidence step that governs it. |
| Operations reviewer | See how DRAIN handles wet-weather flow, dry-weather average, overflow, and recovery conceptually. | Capacity and adequacy claims expose their demand basis (WetWeather vs DryWeather) and evidence level, not just an aggregate score. |
| Regulatory / rate-base stakeholder | See whether NPDES permits, consent decrees, CSO/SSO mandates, financing, and affordability constraints are represented honestly. | Permit, schedule, ratepayer, and funding assumptions are explicit and priced, not assumed free. |
| Ratepayer / receiving-water reviewer | See service coverage, household affordability, public health, and downstream ecology before a project is promoted. | Affordability, backup, service-coverage, pollutant-load, and overflow claims point to data or held evidence, not narrative alone. |
| Coding agent | Make scoped changes without drifting claims, artifacts, scale, demand basis, or review obligations. | Work packages name parent IDs, affected modules/data/docs, validation commands, and evidence rows before closure. |

## Operating Context

DRAIN will be a data corpus, review system, and research/design process for Wastewater & Sewer 2.0, with any implementation built later by implementation automation from accepted VTRACE work packages. It is **multi-scale by design**: the same corpus, dimension pool, and tier model apply to a local collection district, one treatment plant, a municipal system, a metro/watershed utility, a national wastewater program, or an international transboundary basin benchmark, and a run targets a stated scale. Work happens inside a dirty portfolio checkout, so repo-local changes must stay scoped and must not depend on TRACKER-relative paths for build correctness. DRAIN is not yet a TRACKER submodule until intake completes.

This mission file does not yet assert any scored result. It creates the VTRACE anchor that later requirements, specifications, and work packages trace back to.

The tiering frame (NEED-007) and the scale frame (NEED-008) extend the portfolio pattern shared with ROUTE, PYLON, GAUGE, BASIN, PACKET, TARMAC, and HARBOR: wastewater is a tiered SLA system (major regional system to local/decentralized service) that, like water supply, ports, internet, and air, must be analyzed at whatever scale (local to international) the question demands. DRAIN is the sanitation complement to BASIN's water-supply frame.

## Constraints

| ID | Constraint | Rationale | Status |
|---|---|---|---|
| CON-001 | DRAIN public claims must stay bounded by implemented commands, generated artifacts, source labels, confidence labels, and review records. | Prevents planned, heuristic, or simulated work from reading as proof-grade evidence. | accepted |
| CON-002 | Element-bearing artifacts must preserve stable plant/interceptor/district/outfall/network identity; operators, permit nicknames, and map ids are not primary keys. | Keeps scores, gaps, and proposals tied to stable physical and jurisdictional identity. | accepted |
| CON-003 | Generated artifacts must name the source-of-truth data and commands that regenerate them. | Keeps the repo reproducible and prevents hand-edited generated outputs from becoming hidden state. | accepted |
| CON-004 | Source gaps, heuristic rows, simulated evidence, and human/owner review holds must remain visible status, not missing prose. | Keeps evidence debt actionable and traceable. | accepted |
| CON-005 | DRAIN implementation changes belong in this repo; TRACKER receives only intentional submodule pointer updates after intake. | Preserves portfolio snapshot discipline. | accepted |
| CON-006 | DRAIN must not claim construction readiness, capacity/effluent validity of record, permit compliance determination, consent-decree determination, rate-case determination, or official agency/utility/operator endorsement. | Keeps the project framed as research, tooling, review, and conceptual design. | accepted |
| CON-007 | Every claim must declare its scale, and scores/tiers/gaps must not be compared or aggregated across scales without an explicit, labelled cross-scale note. | Prevents misleading mixing of local and national/international evidence (NEED-008). | accepted |

## Non-Goals

- DRAIN is not an engineering study, treatment-plant design, or discharge-permit application.
- DRAIN is not a rate case, consent-decree determination, or regulatory compliance determination.
- DRAIN is not an advocacy brief for a specific utility, community, technology, or policy.
- DRAIN does not predict what EPA, state agencies, utilities, or operators will build or call.
- DRAIN does not treat illustrative maps or heuristic forecasts as proof-grade evidence unless their evidence level says so.

## Success Criteria

| Criterion | Validation Method | Evidence Pointer |
|---|---|---|
| VTRACE mission needs are explicit enough to derive requirements. | Inspect this file before writing `REQUIREMENTS.md`. | future `EVID-*` |
| Mission needs cover corpus reproducibility, evidence posture, design boundaries, identity, review roles, null-result discipline, tiered SLAs, multi-scale applicability, and named demand basis. | Cross-check against `README.md`, `PRODUCT_PLAN.md`, and `CLAUDE.md`. | future `EVID-*` |
| Later VTRACE artifacts can reference stable parent IDs. | `REQ-*` rows should cite `NEED-*` and `CON-*` IDs from this file. | future `TRACE.md` |

## Role Review Notes

| Role Lens | Mission Impact | Disposition |
|---|---|---|
| Scope Keeper | Mission stays at repo/system intent; it asserts no scores, gap findings, or design proposals, and names the multi-scale rule. | pass |
| Citation Auditor | Mission makes no quantitative claims beyond ID/tier labels; public source families are named as future corpus inputs. | pass |
| Numeracy Checker | Mission contains no arithmetic, capacity, concentration, overflow-frequency, or cost claims. | pass |
| Wastewater-System Planner | Mission names treatment capacity, conveyance, tiering, multi-scale, and public-interest intent. | pass |
| Operations Officer | Mission requires demand-basis framing for capacity/adequacy (NEED-002/003). | pass |
| Regulatory & Consent-Decree Realist | Initial draft underplayed NPDES permits, consent-decree schedules, CSO/SSO mandates, and ratepayer limits; resolved by adding the regulatory/rate-base user lens and CON-006 determination boundary. | resolved |
| Equity, Affordability & Receiving-Water advocates | Mission names service coverage, affordability, public health, overflow, pollutant load, and downstream ecology as first-class via users and NEED-002. | pass |

Fixed-point note: one actionable finding (permit/consent-decree/ratepayer constraints under-represented) was raised and applied. No unresolved critical or major finding remains. Deferred: dimension pool, scoring rubric, tier SLA thresholds, demand methodology (`WetWeather` / `DryWeather`), and the scale-tagging schema to REQUIREMENTS and SPECIFICATION_BASELINE.

## Source Links

- `README.md`
- `PRODUCT_PLAN.md`
- `CLAUDE.md`
- `.roles/ROLE.md`
