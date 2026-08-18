# Transactions Architecture

`transactions` records the normalized transaction history. Before a spend is recorded, `spending-rules` composes policy, category, and wallet limit checks. `spending-categories` supplies reporting metadata, while `spending-limits` enforces per-wallet and period constraints. The transaction record is written only after authorization and checked amount validation succeed.
