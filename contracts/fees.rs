use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype, panic_with_error, symbol_short,
    Address, Env,
};

/// Storage keys used by the fees contract.
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    /// Fee percentage stored in basis points (bps).
    /// The value is expected to be between 0 and 10_000 (100%).
    FeePercentage,
    /// Cumulative fees that have been collected through `deduct_fee`.
    TotalFeesCollected,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FeeError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    InvalidPercentage = 4,
    InvalidAmount = 5,
    Overflow = 6,
}

/// Events emitted by the fees contract.
pub struct FeeEvents;

impl FeeEvents {
    pub fn fee_deducted(env: &Env, payer: &Address, amount: i128, fee: i128) {
        let topics = (symbol_short!("fee"), symbol_short!("deducted"));
        env.events().publish(
            topics,
            (payer.clone(), amount, fee, env.ledger().timestamp()),
        );
    }

    pub fn config_updated(env: &Env, admin: &Address, percentage_bps: u32) {
        let topics = (symbol_short!("fee"), symbol_short!("config_updated"));
        env.events().publish(
            topics,
            (admin.clone(), percentage_bps, env.ledger().timestamp()),
        );
    }
}

#[contract]
pub struct FeesContract;

#[contractimpl]
impl FeesContract {
    /// Initializes the fees contract with an admin and an initial percentage
    /// (in basis points). Only callable once.
    pub fn initialize(env: Env, admin: Address, percentage_bps: u32) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic_with_error!(&env, FeeError::AlreadyInitialized);
        }
        if percentage_bps > 10_000 {
            panic_with_error!(&env, FeeError::InvalidPercentage);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::FeePercentage, &percentage_bps);
        env.storage()
            .instance()
            .set(&DataKey::TotalFeesCollected, &0i128);
    }

    /// Sets a new percentage fee. Only the admin may call.
    pub fn set_percentage(env: Env, caller: Address, percentage_bps: u32) {
        caller.require_auth();
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic_with_error!(&env, FeeError::NotInitialized));
        if caller != admin {
            panic_with_error!(&env, FeeError::Unauthorized);
        }
        if percentage_bps > 10_000 {
            panic_with_error!(&env, FeeError::InvalidPercentage);
        }
        env.storage()
            .instance()
            .set(&DataKey::FeePercentage, &percentage_bps);
        FeeEvents::config_updated(&env, &caller, percentage_bps);
    }

    /// Returns the current fee percentage (in basis points).
    pub fn get_percentage(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::FeePercentage)
            .unwrap_or(0)
    }

    /// Calculates the fee for a given amount using the current percentage.
    /// Amount must be positive; fee is rounded down.
    pub fn calculate_fee(env: Env, amount: i128) -> i128 {
        if amount <= 0 {
            panic_with_error!(&env, FeeError::InvalidAmount);
        }
        let pct: u32 = Self::get_percentage(&env);
        let fee = amount
            .checked_mul(pct as i128)
            .unwrap_or_else(|| panic_with_error!(&env, FeeError::Overflow))
            .checked_div(10_000)
            .unwrap_or_else(|| panic_with_error!(&env, FeeError::Overflow));
        fee
    }

    /// Deducts the configured fee from `amount`. The caller must authorize
    /// as `payer`. Returns a tuple `(net_amount, fee)` and updates internal
    /// total-collected accounting. Emits a `fee_deducted` event.
    pub fn deduct_fee(env: Env, payer: Address, amount: i128) -> (i128, i128) {
        payer.require_auth();
        let fee = Self::calculate_fee(&env, amount);
        let net = amount
            .checked_sub(fee)
            .unwrap_or_else(|| panic_with_error!(&env, FeeError::Overflow));
        let mut total: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalFeesCollected)
            .unwrap_or(0);
        total = total
            .checked_add(fee)
            .unwrap_or_else(|| panic_with_error!(&env, FeeError::Overflow));
        env.storage()
            .instance()
            .set(&DataKey::TotalFeesCollected, &total);
        FeeEvents::fee_deducted(&env, &payer, amount, fee);
        (net, fee)
    }

    /// Returns cumulative fees collected so far.
    pub fn get_total_collected(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalFeesCollected)
            .unwrap_or(0)
    }
}
