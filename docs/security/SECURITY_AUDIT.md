# Security Audit Baseline

## Threat model

The contracts assume hostile callers, malformed asset identifiers, replayed calls, and arithmetic boundary values. Every state-changing entry point authenticates its caller before business logic, validates inputs, and uses checked arithmetic.

## Boundaries

Administrative configuration is restricted by `access-control` and governance. User balances and goals are scoped to authenticated addresses. `zk-verifier` is the trust boundary for private spending policies. `audit` provides an append-only record of security-relevant actions.

## Limitations

This document is a baseline, not an independent audit. Deployments must validate issuer addresses for XLM, USDC, and EURC, configure emergency pause authority, and test contract composition on the intended Stellar network.
