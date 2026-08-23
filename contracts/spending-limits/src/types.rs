use soroban_sdk::{contracttype, Address, Env, Symbol};

/// Spending-limit reset period. Mirrors the canonical Symbol values learners
/// pass in ("daily" | "weekly" | "monthly") — see `Period::from_symbol`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum Period {
    Daily,
    Weekly,
    Monthly,
}

impl Period {
    /// Period length in seconds.
    pub fn seconds(&self) -> u64 {
        match self {
            Period::Daily => 86_400,
            Period::Weekly => 604_800,
            Period::Monthly => 2_592_000, // 30 days
        }
    }

    /// Parses a period from its canonical Symbol representation.
    /// Returns `None` for anything other than "daily" | "weekly" | "monthly".
    pub fn from_symbol(env: &Env, symbol: &Symbol) -> Option<Period> {
        if *symbol == Symbol::new(env, "daily") {
            Some(Period::Daily)
        } else if *symbol == Symbol::new(env, "weekly") {
            Some(Period::Weekly)
        } else if *symbol == Symbol::new(env, "monthly") {
            Some(Period::Monthly)
        } else {
            None
        }
    }

    /// The index of the period bucket the current ledger timestamp falls
    /// into. Spend is accumulated per `(user, asset, index)` — a new index
    /// (i.e. a new period) always starts with an empty accumulator, which is
    /// the entire period-reset mechanism: there is no explicit "reset" step,
    /// old buckets are simply never read again once time moves past them.
    pub fn index(&self, env: &Env) -> u64 {
        env.ledger().timestamp() / self.seconds()
    }
}

/// A per-user, per-asset spending cap.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Limit {
    pub amount: i128,
    pub period: Period,
}

/// Storage keys for this contract. User records use persistent storage;
/// `Admin` uses instance storage — matches docs/ARCHITECTURE.md's storage
/// tiering convention ("User records use persistent storage; instance
/// storage holds contract configuration").
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// (user, asset) -> Limit
    Limit(Address, Symbol),
    /// (user, asset, period_index) -> i128 accumulated spend
    Spent(Address, Symbol, u64),
}
