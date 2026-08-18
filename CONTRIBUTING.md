# Contributing to StellarSpend Contracts

## Ownership map

| Crate | Responsibility |
|---|---|
| spending-policy | Programmable policy configuration |
| spending-limits | Per-wallet caps |
| spending-categories | Transaction category tags |
| spending-rules | Policy composition |
| budget / budget-allocation | Budget periods and allocations |
| savings / savings-goals | Vaults and savings milestones |
| escrow / escrow-v2 | Conditional and disputed funds |
| multi-currency-wallet / currency-conversion | Stellar asset balances and conversion |
| transactions | Transaction history |
| recurring-payment / subscription | Scheduled obligations |
| batch-payment / batch-transfer | Multi-recipient settlement |
| rewards / batch-rewards | Rewards issuance |
| token / treasury / fee | Protocol economics |
| governance / delegation / multisig / timelock | Authorization and governance |
| access-control / audit / pausable / compliance | Security controls |
| merchant-tagging / notification / activity-feed | User experience records |
| cross-contract / shared | Reusable contract interfaces and utilities |
| zk-verifier | ZK proof verification; frozen submission artifact |

## Adding a contract

Create `contracts/<name>/Cargo.toml` and `contracts/<name>/src/lib.rs`, `types.rs`, `storage.rs`, `validation.rs`, and `test.rs`. Add the crate to the workspace, use the pinned Soroban SDK, document every public function, authenticate state changes first, and use checked arithmetic.

## Pull request checklist

Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and the WASM release build. Include test output and explain storage, authorization, and security decisions.
