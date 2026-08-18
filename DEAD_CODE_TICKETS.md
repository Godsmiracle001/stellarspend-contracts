# Dead Code Tickets

No workspace crate is intentionally left outside the declared workspace. The following modules are scaffolded for domain expansion and should receive contract-specific storage and integration tests before production deployment:

- `spending-policy`, `spending-rules`: policy composition implementation.
- `budget`, `budget-allocation`: period and allocation persistence.
- `multi-currency-wallet`, `currency-conversion`: issuer-aware asset handling.
- `transactions`, `recurring-payment`, `batch-transfer`: settlement integration.
- `token`, `treasury`, `governance`, `delegation`, `timelock`, `multisig`: protocol governance flows.
- `compliance`, `merchant-tagging`, `notification`, `activity-feed`: application-facing records.

`zk-verifier` is intentionally frozen and must not be modified.
