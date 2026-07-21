# DRAIN

**Wastewater & Sewer 2.0 — multi-scale sanitation-network analysis.**

**A treatment plant cannot solve a network that fails before flow reaches it.**

DRAIN scores collection, conveyance, treatment, overflow control, effluent
quality, resilience, asset condition, resource recovery, affordability, and
receiving-water impact at a declared scale.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

> DRAIN is not an engineering study, treatment design, discharge-permit
> application, rate case, consent-decree determination, or advocacy brief, and
> it claims no EPA, state-agency, utility, or operator endorsement.

## Why this matters

Capacity on paper can disappear under wet weather, infiltration, failing lift
stations, overflow constraints, or weak collection coverage. DRAIN keeps the
whole sanitation service chain and its demand basis visible.

## What is implemented

| Crate | Responsibility |
|---|---|
| `drain-network` | Sanitation elements and scale-aware graph contracts. |
| `drain-corpus` | Evidence-labelled corpus validation. |
| `drain-score` | DIM-01..13 score artifacts. |
| `drain-tier` | Tier-SLA classification and shortfalls. |
| `drain-gap` | Gap analysis and explicit null results. |
| `drain-cli` | Corpus, score, tier-SLA, and gap commands. |

The implementation baseline is complete and fixture-backed. The first cited
end-to-end wastewater-system analysis is next.

## Quick start

```powershell
cargo run -p drain-cli -- --help
cargo test --workspace
```

## Method

```text
CORPUS -> SCORE -> TIER-SLA -> GAP -> CONCEPT -> REVIEW -> DESIGN
```

## Documentation

- [`PRODUCT_PLAN.md`](PRODUCT_PLAN.md)
- [`docs/vtrace/`](docs/vtrace)
- [`context/waves/`](context/waves)
- [`.roles/ROLE.md`](.roles/ROLE.md)

## License

MIT. See [`LICENSE`](LICENSE).
