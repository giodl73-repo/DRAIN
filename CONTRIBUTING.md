# Contributing

Keep DRAIN scale-aware, evidence-labelled, and explicit about the difference between analysis and operational, regulatory, or individual decisions.

Useful public contributions include source inventories for collection,
conveyance, overflow, treatment, effluent, asset, affordability, and
receiving-water evidence. For first public run scoping, start with
[`docs/adoption/README.md`](docs/adoption/README.md).

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p drain-cli -- --help
```

Do not commit restricted datasets, credentials, local build state, personal records, or uncited public claims.
