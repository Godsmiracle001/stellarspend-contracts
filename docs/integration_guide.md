# Integration Guide

The frontend should build Soroban contract invocations with the Stellar SDK, select the deployed contract ID, simulate, sign, and submit through the selected Stellar network RPC endpoint.

```ts
const operation = new Contract(contractId)
  .call(method, ...args)
const prepared = await rpc.simulateTransaction(operation)
const signed = await wallet.sign(prepared)
await rpc.sendTransaction(signed)
```

Use `initialize` once, then call the domain entry point for the deployed crate: spending policy and limits for authorization, budget and savings goals for planning, wallet and conversion for assets, transactions for history, batch contracts for settlement, and audit for observability. Never trust client-side totals; recompute fees, quantities, and authorization on-chain.
