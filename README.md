# DRAIN

**Wastewater & Sewer 2.0 — multi-scale sanitation-network analysis.**

**A treatment plant cannot solve a network that fails before flow reaches it.**

DRAIN scores collection, conveyance, treatment, overflow control, effluent
quality, resilience, asset condition, resource recovery, affordability, and
receiving-water impact at a declared scale.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

## Infrastructure 2.0 family

DRAIN is one domain implementation of a shared evidence-first method:

```text
PUBLIC SOURCES → CORPUS → SCORE → SERVICE PROMISE → GAP MAP
                                                     ↓
                                      CONCEPT → REVIEW → DESIGN
```

| Lane | Repositories |
|------|--------------|
| Movement | [ROUTE](https://github.com/giodl73-repo/ROUTE), [GAUGE](https://github.com/giodl73-repo/GAUGE), [TARMAC](https://github.com/giodl73-repo/TARMAC), [HARBOR](https://github.com/giodl73-repo/HARBOR) |
| Lifelines | [PYLON](https://github.com/giodl73-repo/PYLON), [PACKET](https://github.com/giodl73-repo/PACKET), [BASIN](https://github.com/giodl73-repo/BASIN), [DRAIN](https://github.com/giodl73-repo/DRAIN) |
| Public access | [SHIELD](https://github.com/giodl73-repo/SHIELD), [SLATE](https://github.com/giodl73-repo/SLATE) |
| Civic boundaries | [ZONES](https://github.com/giodl73-repo/ZONES) |

The family shares evidence labels, explicit scale and demand bases, T1–T4
service promises where meaningful, adversarial review, and acceptance of a
rigorous null result. Each repository owns its domain semantics and safety
boundary.

> DRAIN is not an engineering study, treatment design, discharge-permit
> application, rate case, consent-decree determination, or advocacy brief, and
> it claims no EPA, state-agency, utility, or operator endorsement.

## Use DRAIN

DRAIN is public and open to use as a reference model for sanitation-system
evidence discipline. To scope the first public run or contribute source
surfaces, start with [`docs/adoption/README.md`](docs/adoption/README.md).

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

## Next public run

The first public DRAIN finding should be deliberately narrow: one utility
service area, collection basin, overflow problem, treatment constraint,
receiving-water exposure, or affordability question with reproducible source
manifests.

| Need | Example Source Surface |
|---|---|
| Collection and conveyance | sewer maps, lift-station records, infiltration/inflow reports. |
| Overflow and wet-weather exposure | CSO/SSO reports, consent-decree documents, rain records. |
| Treatment and effluent | permit records, plant capacity, discharge monitoring reports. |
| Affordability and asset posture | rate schedules, capital plans, asset-management records. |

The first run should prove the evidence workflow and gap classification. It
should not claim engineering-study status, treatment design, permit readiness,
rate-case support, consent-decree determination, or EPA, state-agency, utility,
or operator endorsement.

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
- [`docs/adoption/`](docs/adoption)
- [`docs/vtrace/`](docs/vtrace)
- [`context/waves/`](context/waves)
- [`.roles/ROLE.md`](.roles/ROLE.md)

## License

MIT. See [`LICENSE`](LICENSE).
