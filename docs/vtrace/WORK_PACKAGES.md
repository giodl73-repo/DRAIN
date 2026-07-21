# Work Packages

## Scope

Repo: DRAIN. Six implementation work packages that build the pipeline bottom-up. Each is sized for an implementing agent (implementation automation) to run end-to-end: pick the lowest unblocked WP, satisfy entry criteria, implement only the named surfaces, run the verification commands, meet exit criteria, record the pulse, commit.

Product boundary rule: VTRACE/work-package/proof/readiness/evidence concepts are **not** product features. Do **not** add `work-package`, `prove`, `readiness`, or `evidence` subcommands. The CLI's product surface is corpus/score/tier/gap only.

## Work Package Table

| ID | Objective | Parent IDs | Affected Surfaces | L0 / L1 / L2 | Status |
|---|---|---|---|---|---|
| WP-001 | Plant/sewer graph kernel: identity, connectivity, diverse paths, MGD capacity, typed demand basis | REQ-004/005/007, SPEC-001/005, IF-005, PKG-001, CR-001..008 | `Cargo.toml`, `crates/drain-network/**` | L0: `cargo test -p drain-network` / L1: workspace fmt+clippy+test / L2: n/a | ready |
| WP-002 | Corpus model + scale/jurisdiction tags + `corpus/SCHEMA.md` + `data/sources.md` + labels + hold/reject | REQ-001/002/003/005/016, SPEC-002/003/009/013, IF-001/002, PKG-002 | `crates/drain-corpus/**`, `corpus/SCHEMA.md`, `data/sources.md` | L0: `cargo test -p drain-corpus` + `proof check .` / L1: workspace / L2: n/a | blocked by WP-001 |
| WP-003 | Dimension scoring DIM-01..13 (0–10) + rubric version record | REQ-006, SPEC-004, IF-003, PKG-003 | `crates/drain-score/**` | L0: `cargo test -p drain-score` / L1: workspace / L2: n/a | blocked by WP-002 |
| WP-004 | Tier T1–T4 classification + SLA conformance (DIM-13) + tier-SLA gap | REQ-014/015, SPEC-011/012, IF-004, PKG-004 | `crates/drain-tier/**`, `tiers.toml` | L0: `cargo test -p drain-tier` / L1: workspace / L2: n/a | blocked by WP-003 |
| WP-005 | Gap analysis: under-served-region finder + scale filter + null result | REQ-008/016, SPEC-006/013, PKG-005 | `crates/drain-gap/**` | L0: `cargo test -p drain-gap` / L1: workspace / L2: n/a | blocked by WP-004 |
| WP-006 | `drain` CLI: corpus/score/tier-sla/gap commands (incl. `--scale`) + reproducible artifacts | REQ-001, IF-006, PKG-006 | `crates/drain-cli/**` | L0: `cargo run -p drain-cli -- --help` / L1: workspace / L2: end-to-end demo | blocked by WP-005 |

## Work Package Details

### WP-001: Plant/sewer graph kernel

Objective: a `drain-network` crate that models the network as a graph of treatment plants and sewers/interceptors/force mains and exposes the load-bearing metrics, including diverse-path/resilience and typed demand basis.

Parent requirements: REQ-004, REQ-005, REQ-007.
Parent specs: SPEC-001 (identity), SPEC-005 / SPEC-WW-01 / SPEC-DW-01 (demand basis typed).
Boundary/interface: PKG-001, IF-005. Code rigor: CR-001..008.

Product files to create:

- `Cargo.toml` (workspace, members include `crates/drain-network`).
- `crates/drain-network/Cargo.toml` (deps: `petgraph`, `serde`, `thiserror`).
- `crates/drain-network/src/lib.rs` with:
  - `Plant { id, name, role }`, `Sewer { id, capacity_mgd, basis }` where `basis` is a typed enum (`WetWeather` | `DryWeather`) — REQ-007.
  - `Network` over a `petgraph` graph + `id -> NodeIndex` index.
  - `add_plant` (reject duplicate id), `add_sewer` (reject unknown plant / non-positive capacity), with a `NetworkError` enum.
  - `plant_count`, `sewer_count`, `degree(id)`, `is_connected(a, b)`, `has_diverse_path(a, b)` (second node-disjoint path — resilience), `incident_capacity_mgd(id)`.

Entry criteria:

- [ ] `cargo` toolchain available.
- [ ] No existing `crates/` (greenfield).

Exit criteria:

- [ ] Workspace compiles; `cargo test -p drain-network` green.
- [ ] Tests cover: build network; degree; connectivity vs gap; incident capacity; a `WetWeather` vs `DryWeather` sewer basis is preserved (REQ-007); `has_diverse_path` true on a ring/mesh and false on a single-path chain; duplicate-plant, non-positive capacity, and unknown-plant typed errors (CR-003/004).
- [ ] No `unwrap`/`panic!` in lib paths except tests (CR-006); `clippy -D warnings` clean (CR-005).

Verification commands:

```powershell
cargo fmt --check
cargo clippy -p drain-network -- -D warnings
cargo test -p drain-network
```

Validation levels:

| Level | Required | Commands / Evidence |
|---|---|---|
| L0 | yes | `cargo test -p drain-network` |
| L1 | yes | `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace` |
| L2 | no | n/a (no pipeline yet) |

Boundary control:

| Boundary ID | Allowed Changes | Forbidden Changes | Integration |
|---|---|---|---|
| PKG-001 | `crates/drain-network/**`, workspace `Cargo.toml` | scoring, tier, CLI, corpus, scale logic | no |

Git execution: branch `wp-001-network`; one commit `DRAIN: WP-001 plant/sewer kernel`; push when L1 green; stop when exit criteria met (do not start WP-002).

VTRACE-only closeout: set EVID-004/005/007 to passed; mark TRACE rows REQ-004/005/007 `implemented`; record pulse.

Status: ready.

### WP-002: Corpus model, scale tags, schema, sources, labels

Objective: `drain-corpus` crate + `corpus/SCHEMA.md` + `data/sources.md` that represent one element as a labelled, sourced, **scale-tagged**, tiered corpus entry, and hold/reject unidentified, uncited, or untagged-scale rows.

Parent: REQ-001/002/003/005/016, SPEC-002/003/009/013, IF-001/002, PKG-002.

Product surfaces: `crates/drain-corpus/src/lib.rs` (`Scale` enum {International,National,Regional,Local}; `EvidenceLabel` enum; `DemandBasis` enum {WetWeather,DryWeather}; `Quantity { value, unit, label, source_id }`; `CorpusEntry { id, type, scale, jurisdiction, tier, sla, quantities, scores }`; load/validate from markdown+frontmatter; `validate()` → held/rejected reasons incl. missing scale); `corpus/SCHEMA.md` (IF-001 incl. scale enum); `data/sources.md` (IF-002) with seed entries for EPA CWNS, EPA ECHO/NPDES DMR, EPA CSO/SSO reports, Census of Governments utility finance, USGS receiving-water, and state discharge permits.

Exit criteria: `cargo test -p drain-corpus` green (missing-id reject, uncited quantity held, **missing-scale held**, label preservation — CR-007); `proof check .` clean. Boundary PKG-002 (depends on PKG-001 types). Git: `wp-002-corpus`. Status: blocked by WP-001.

### WP-003: Dimension scoring

Objective: `drain-score` crate scoring DIM-01..13 on a 0–10 scale with a versioned rubric; values provisional and labelled (no calibration yet).

Parent: REQ-006, SPEC-004, IF-003, PKG-003.

Product surfaces: `crates/drain-score/src/lib.rs` (`Dimension` enum DIM-01..13, `Score(f64)` bounded `[0,10]`, `Rubric { version, weights }`, scoring trait over a `CorpusEntry`); default rubric v0 with recorded rationale (IF-003).

Exit criteria: `cargo test -p drain-score` green; score-bounds invariant tested (CR-004); rubric version present. Boundary PKG-003 (depends on PKG-001/002). Git: `wp-003-score`. Status: blocked by WP-002.

### WP-004: Tier classification + SLA conformance

Objective: `drain-tier` crate classifying each element into T1–T4, attaching tier SLA terms, computing DIM-13 conformance, and emitting tier-SLA gaps.

Parent: REQ-014/015, SPEC-011/012, DIM-13, IF-004, PKG-004.

Product surfaces: `crates/drain-tier/src/lib.rs` (`Tier { T1..T4 }`, `Sla { treatment_capacity, effluent_quality, overflow_frequency, service_coverage }` per tier, `classify(entry) -> Tier`, `conformance(entry, network) -> Dim13` naming the demand basis (REQ-007), `tier_sla_gap(entry) -> Option<Gap>`); `tiers.toml` SLA record (IF-004), values labelled provisional.

Exit criteria: `cargo test -p drain-tier` green (classification, a conforming element, a shortfall producing a tier-SLA gap; SLA values labelled provisional). Boundary PKG-004 (depends on PKG-001/003). Git: `wp-004-tier`. Status: blocked by WP-003.

### WP-005: Gap analysis (scale-filtered)

Objective: `drain-gap` crate that plots scored elements in dimension space, finds under-served regions **at a chosen scale**, and records an already-adequate market as a labelled null result (REQ-008). Integrates tier-SLA gaps from `drain-tier`.

Parent: REQ-008/016, SPEC-006/013, PKG-005.

Product surfaces: `crates/drain-gap/src/lib.rs` (`GapRegion`, `find_gaps(corpus, rubric, scale) -> Vec<GapRegion>` filtering to a `Scale`, `null_result` path; cross-scale comparisons require an explicit marker; consume tier-SLA gaps from PKG-004).

Exit criteria: `cargo test -p drain-gap` green (one found gap at a scale, one null-result case, and a test that elements of another scale are excluded unless a cross-scale marker is set — REQ-016). Boundary PKG-005 (depends on PKG-003/004). Git: `wp-005-gap`. Status: blocked by WP-004.

### WP-006: CLI orchestration

Objective: `drain` CLI exposing `corpus`, `score`, `tier-sla`, and `gap` subcommands (with a `--scale` filter) that drive the pipeline and emit reproducible artifacts with labels and scale preserved (REQ-001, IF-006).

Parent: REQ-001, IF-006, PKG-006.

Product surfaces: `crates/drain-cli/src/main.rs` (clap subcommands + `--scale`; reads corpus, runs score/tier/gap, writes artifacts; `--help` documents the product surface; no VTRACE subcommands).

Exit criteria: `cargo run -p drain-cli -- --help` lists product subcommands and the `--scale` flag; end-to-end run on a seed corpus at a chosen scale regenerates artifacts deterministically (CR-008); `cargo test --workspace` green. Boundary PKG-006 (depends on all). Git: `wp-006-cli`. Status: blocked by WP-005.

## Orphan Check

- [x] Every accepted `REQ-*` is assigned to a work package or dispositioned (REQ-009..013 → already_satisfied process; REQ-001..008/014/015/016 → WP-001..006).
- [x] Every accepted `SPEC-*` is assigned to a work package or verification item.
- [x] Every interface-changing work package names `IF-*` IDs.
- [x] Every package/module-changing work package names `PKG-*` boundary IDs.
- [x] Every critical-code work package names `CR-*` IDs (WP-001 explicitly; all inherit CR-001..008 via CODE_RIGOR).
- [x] Every work package has exit criteria and verification commands.
- [x] Every work package lists L0/L1/L2 requirements.
- [x] No work package is only "cleanup" without parent IDs.

## Role Review Notes

| Role Lens | Work-Package Impact | Disposition |
|---|---|---|
| Systems engineering / V&V | Each WP is self-contained, ordered, with concrete verification commands and exit criteria. | pass |
| Scope Keeper | Product/process split enforced; scale lives in WP-002 (corpus) and WP-005 (gap), not the CLI surface as a VTRACE concept. | pass |
| Software-assurance lens | WPs inherit CR-001..008; WP-001 pins identity/connectivity/diverse-path/demand basis, WP-002 pins the scale-tag invariant. | pass |
| Operations Officer / Regulatory & Consent-Decree Realist | WP-001 makes the demand basis typed; WP-004 names it on DIM-13 conformance (REQ-007). | pass |

Fixed-point note: no actionable finding required a change. Work packages are runnable and orphan-free. WP-001 is `ready`; the rest unblock in sequence.
