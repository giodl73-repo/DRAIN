# DRAIN — Role Index

Four tiers of review roles. Read this before opening any role file. Reviews of corpus entries, gap findings, design proposals, tier/SLA definitions, and VTRACE deliverables run against these files and record dispositions (`pass` / `finding` / `defer`).

---

## Parliament roles (7 voices)

Adversarial expert voices. They plant incompatible stakes; the argument record is the output, not consensus. No voice is skipped. A good project survives all seven; a weak one collapses under one or two, and the collapse is the finding.

| File | Voice | Primary tension |
|---|---|---|
| `parliament/wastewater-system-planner.md` | Wastewater-System Planner | System capacity + resilience vs. single-plant framing |
| `parliament/sanitary-civil-engineer.md` | Sanitary / Civil Engineer | Buildable plant/interceptor/pump capacity vs. brochure-fantasy throughput |
| `parliament/operations-officer.md` | Operations Officer | Wet-weather surge / overflow vs. dry-weather-average optimism |
| `parliament/public-health-economist.md` | Public-Health & Environmental Economist | Benefit-cost + public-health/receiving-water value vs. discretionary-project inflation |
| `parliament/equity-affordability-advocate.md` | Equity & Affordability Advocate | Unsewered/underserved coverage + ratepayer burden vs. capital-concentrated benefit |
| `parliament/ecology-receiving-water-advocate.md` | Receiving-Water & Ecology Advocate | Nutrient/CSO/SSO loads + habitat vs. capacity expansion |
| `parliament/regulatory-consent-decree-realist.md` | Regulatory & Consent-Decree Realist | NPDES/consent-decree/ratepayer constraints vs. assumed-cooperation/assumed-funding |

---

## Editorial roles (3 voices)

Form gate before `validated` status. Run after parliament, not instead of it.

| File | Role | Checks |
|---|---|---|
| `editorial/citation-auditor.md` | Citation Auditor | Every quantity sourced in `data/sources.md` or labelled |
| `editorial/scope-keeper.md` | Scope Keeper | Artifact stays within its declared type, **scale**, schema, pool, and tier model |
| `editorial/numeracy-checker.md` | Numeracy Checker | Units consistent (MGD / mg·L⁻¹ / overflow events·yr⁻¹ / $ / days); magnitudes sane; 0–10 scale clean |

---

## Stakeholder roles (cross-cutting lenses)

Not reviewers — lenses for who the network serves, used during corpus scoring, gap analysis, and tier/SLA assignment.

| File | Stakeholder | Primary concern |
|---|---|---|
| `stakeholders/ratepayer-household.md` | Ratepayer / Household | Affordable, reliable sewer service |
| `stakeholders/public-health-officer.md` | Public-Health Officer | Disease prevention, safe sanitation, backups |
| `stakeholders/downstream-community.md` | Downstream / Receiving-Water Community | Clean water, no overflows on their shore |
| `stakeholders/utility-operator.md` | Utility Operator | Operability, asset condition, compliance |
| `stakeholders/industrial-discharger.md` | Industrial Discharger | Pretreatment, capacity, permit limits |

---

## Panel reviewer roles (illustrative peer panel)

Archetype academic/practitioner peer reviewers for DRAIN research outputs. See `panel-reviewer/panel.md`. Used for paper-grade methodology review, distinct from parliament and editorial.

---

## How reviews are recorded

When a `docs/vtrace/` deliverable, corpus entry, gap finding, design proposal, or tier/SLA definition is being settled, the relevant subset of this panel is applied and dispositions are recorded in:

- the deliverable's **Role Review Notes** section, and
- the active wave pulse ledger.

A stage reaches its **fixed point** when no unresolved critical or major actionable finding remains and every deferred item names a later stage or work package.
