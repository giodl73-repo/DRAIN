# Pulse 01: WP-001 `drain-network` wastewater/sewer kernel

Status: pending. Executes WP-001 (see `docs/vtrace/WORK_PACKAGES.md`).

## Scope

The wastewater/sewer graph kernel — the pipeline primitive every other crate depends on. Implements
the load-bearing identity, connectivity, and typed demand basis (WetWeather/DryWeather) invariants
required by REQ-007.

## Planned changes

- `Cargo.toml` workspace (member `crates/drain-network`).
- `crates/drain-network/Cargo.toml` (deps: `petgraph`, `serde`, `thiserror`).
- `crates/drain-network/src/lib.rs`: `Plant`, `Sewer` (with typed `DemandBasis` enum), `Network`,
  `NetworkError`; `add_plant`/`add_sewer` (identity + validation); `plant_count`, `sewer_count`,
  `degree`, `is_connected`, `has_diverse_path`, `incident_capacity_mgd`.

## Parent IDs

REQ-004/005/007 · SPEC-001/005 · IF-005 · PKG-001 · CR-001..008.

## Exit criteria

- Workspace compiles; `cargo test -p drain-network` green.
- Tests cover: build network; degree; connectivity vs gap; incident capacity; demand basis
  preserved (WetWeather/DryWeather); `has_diverse_path` true on a ring/mesh and false on a
  single-path chain; duplicate-plant, non-positive capacity, unknown-plant typed errors.
- No `unwrap`/`panic!` in lib paths except tests; `clippy -D warnings` clean.

## Validation

```powershell
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test -p drain-network
```

## VTRACE closeout (on completion)

VER-004/005/007 + EVID-CR-001..003 → passed; TRACE REQ-004/005/007 → implemented; WORK_PACKAGES
WP-001 → done; unblock WP-002.

## Status

Pending — ready for implementation automation to execute.
