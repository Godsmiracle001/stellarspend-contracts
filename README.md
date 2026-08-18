# StellarSpend Contracts

Soroban smart contracts for StellarSpend, a financial management platform for unbanked and underbanked users on Stellar. The workspace provides spending policies, budgets, savings goals, multi-currency wallets, payments, rewards, treasury controls, and ZK-gated spending rules.

## Architecture

```text
Stellar wallet
    |
    +--> spending-policy --> spending-rules --> spending-limits
    |                              |                 |
    |                              +--> categories  +--> zk-verifier
    +--> transactions --> audit --> treasury <-- fee
    +--> savings-goals --> savings
```

Every contract is a standalone Cargo crate under `contracts/`. The `shared` crate contains common validation, authorization, asset, event, and rate-curve utilities. The `zk-verifier` crate is retained as the frozen verification artifact.

## Quick start

Install Rust stable and the Soroban target, then run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --target wasm32-unknown-unknown --release
```

## Contract families

Spending: `spending-policy`, `spending-limits`, `spending-categories`, `spending-rules`.

Planning: `budget`, `budget-allocation`, `savings`, `savings-goals`.

Assets and settlement: `multi-currency-wallet`, `currency-conversion`, `transactions`, `recurring-payment`, `batch-payment`, `batch-transfer`.

Protocol controls: `token`, `treasury`, `fee`, `governance`, `delegation`, `timelock`, `multisig`, `access-control`, `audit`, `pausable`, `compliance`.

Experience and composition: `merchant-tagging`, `notification`, `cross-contract`, `activity-feed`.

See [CONTRIBUTING.md](CONTRIBUTING.md), [ARCHITECTURE.md](docs/ARCHITECTURE.md), [security audit](docs/security/SECURITY_AUDIT.md), [integration guide](docs/integration_guide.md), and [gas reference](docs/gas-reference.md).

## Security

State-changing functions authenticate before business logic. Financial amounts use checked arithmetic, typed contract errors, and explicit Stellar asset validation. This repository is not a substitute for an independent production security audit.

## License

MIT
