# StellarSpend Architecture

StellarSpend is a Soroban workspace for financial management on Stellar, designed around stable asset payments, transparent policy enforcement, and accessible savings primitives.

```text
wallet -> spending-policy -> spending-rules -> spending-limits
                      \-> spending-categories -> transactions
payment -> fee -> treasury
savings-goals -> savings -> audit
private policy proof -> zk-verifier
```

Each contract is independently deployable. Shared types and authentication helpers are intentionally small so contracts remain composable. User records use persistent storage; instance storage holds contract configuration; temporary storage is reserved for ephemeral workflow state.

## Payment flow

A caller authenticates, the rules composition checks policy and category limits, the payment contract computes a checked fee, and the treasury receives the fee before the transfer is finalized. Audit events record state changes.

## ZK-gated policy flow

A user submits a proof to `zk-verifier`; the policy layer accepts only a valid proof, then limits and categories authorize the requested spend. The verifier crate is preserved as supplied and is not modified.
