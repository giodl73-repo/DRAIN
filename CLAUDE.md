# DRAIN — House Rules

## 1. Project Identity

DRAIN is a **research and conceptual-design project for Wastewater & Sewer 2.0** — a data-driven upgrade plan for the wastewater collection, conveyance, and treatment network, applicable at international, national, regional, and local scales. The mission: score an existing network against a calibrated dimension pool (DIM-01 Treatment Capacity; DIM-02 Collection/Service Coverage; DIM-03 Treatment Level / Effluent Quality; DIM-04 Conveyance / Connectivity; DIM-05 Resilience (wet-weather + redundancy); DIM-06 CSO/SSO Overflow Control; DIM-07 Nutrient & Pollutant Removal; DIM-08 Resource Recovery (water reuse, biosolids, energy); DIM-09 Asset Condition / Age; DIM-10 Environmental / Receiving-Water Impact; DIM-11 Equity & Affordability; DIM-12 Benefit-Cost; DIM-13 Tier-SLA Conformance), find the gaps, and design into them.

**The architectural bet** — DRAIN applies the ROUTE/PYLON/GAUGE/BASIN/PACKET/TARMAC/HARBOR architectural bet to wastewater & sewer systems. Score enough of an existing network on enough dimensions and the design space tells you its own structure. The gaps aren't invented; they're found. A project designed into a real gap is better evidence than one invented from first principles.

**The testable hypothesis**: there is a set of ≤20 interventions — at a stated scale — that, if built to Wastewater 2.0 standards, would relieve sewer overflows, raise effluent quality, recover water/energy/nutrients, and harden systems against wet-weather and aging. **A rigorous null result is as valid as a positive one.** Silent scope expansion to rescue a failing hypothesis is not acceptable.

Sibling projects: **ROUTE** (highways), **PYLON** (grid), **GAUGE** (rail), **BASIN** (water), **PACKET** (internet), **TARMAC** (air), **HARBOR** (ports). DRAIN borrows their structural patterns and is the sanitation complement to BASIN's water supply; DRAIN's own rules apply here.

## 2. Multi-Scale Rule

Every corpus element carries a **scale** (`international` / `national` / `regional` / `local`) and a market/jurisdiction. Scores, tiers, gaps, and design proposals are interpreted **within their stated scale**. A claim must not compare or aggregate across scales without saying so. The same dimension pool and tier model apply at every scale; only the scope of the run changes.

## 3. The Pipeline

```
CORPUS (score existing networks) → RUBRIC CALIBRATES → GAP MAP
  → CONCEPT → SCORE → PARLIAMENT → DESIGN → HANDOFF
```

**Anchor rule**: one existing element must go through the full pipeline (corpus entry → calibration pass → gap-map entry) before any proposed project is analyzed. One proposed project must survive parliament manually before any skill is built. YAGNI is the law.

## 4. Quality Bar

- Research-paper-level estimates. Order-of-magnitude treatment capacity, flow, effluent quality, overflow frequency, and cost figures with citations.
- Every number cited. An uncited number blocks promotion to `validated`.
- No capacity, overflow, or adequacy claim dressed as solved engineering — conceptual analysis only, with evidence labels and the demand basis named (peak wet-weather flow vs average dry-weather flow; later enum: `WetWeather` / `DryWeather`).
- No hand-waving on economics. Marginal or negative benefit-cost projects are reported as such.
- Data sources declared. Every corpus entry names its source (`data/sources.md`).

## 5. Forbidden Vocabulary

In corpus entries and design proposals: no "obviously needed," "critical gap," "long overdue," or any pre-judged framing before the score supports it. Claims must cite (a) dimension, (b) score, (c) corpus comparison, (d) scale. "This interceptor scores 8.4 on Overflow Control vs. a corpus mean of 5.1 at regional scale" beats "this is a critical bottleneck."

## 6. VTRACE Governance

DRAIN's planning baseline lives in `docs/vtrace/` and is authored one deliverable at a time to a `.roles` review fixed point. Do not write implementation code in this repo during the foundation stage. implementation automation builds any later implementation only from accepted work packages.

## 7. Review Panel

Seven adversarial parliament voices and a three-role editorial gate review every promotable artifact. See `.roles/ROLE.md`. No voice is skipped. The regulatory-and-consent-decree realist exists because NPDES permits, EPA/state consent decrees, CSO/SSO mandates, rate-base limits, and affordability constraints decide what can be built and how fast — that regulatory/financial tension is a feature, not an accident.

## 8. Portfolio Discipline

DRAIN implementation changes belong in this repo. TRACKER receives only intentional submodule pointer updates after intake. Do not make build or validation correctness depend on TRACKER-relative paths.
