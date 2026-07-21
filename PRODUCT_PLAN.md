# DRAIN Product Plan

## Thesis

Score sanitation networks at a declared scale, identify measurable collection,
overflow, treatment, resilience, and recovery gaps, and design interventions
only where evidence supports them.

## Implemented product shape

- Six-crate Rust workspace for network, corpus, score, tier, gap, and CLI.
- DIM-01..13 scale-aware evidence contracts.
- Tier-SLA shortfall and explicit null-result artifacts.
- Deterministic tests and machine-readable CLI output.

## Next public work

1. Select a bounded source-backed utility or watershed corpus.
2. Publish reproducible flow, overflow, treatment, and asset inputs.
3. Run wet-weather and dry-weather sensitivity analysis.
4. Review the first gap-targeted intervention through the full panel.

## Non-goals

No engineering design, permit, rate case, consent-decree determination, or
uncited adequacy or compliance claim.

## Validation

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p drain-cli -- --help
```
