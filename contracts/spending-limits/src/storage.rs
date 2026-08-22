use soroban_sdk::{Address, Env, Symbol};

use crate::types::{DataKey, Limit};

const ADMIN: &str = "ADMIN";

/// Reads the contract administrator from instance storage.
pub fn read_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&ADMIN)
}

/// Writes the contract administrator to instance storage.
pub fn write_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&ADMIN, admin);
}

/// Reads a user's configured limit for `asset`, if any.
pub fn read_limit(env: &Env, user: &Address, asset: &Symbol) -> Option<Limit> {
    env.storage()
        .persistent()
        .get(&DataKey::Limit(user.clone(), asset.clone()))
}

/// Writes a user's limit for `asset`. Overwriting an existing limit updates
/// only the cap — accumulated spend (stored under a separate key) is
/// untouched, so raising or lowering a limit never resets what's already
/// been spent this period.
pub fn write_limit(env: &Env, user: &Address, asset: &Symbol, limit: &Limit) {
    env.storage()
        .persistent()
        .set(&DataKey::Limit(user.clone(), asset.clone()), limit);
}

/// Reads accumulated spend for `(user, asset)` in period bucket `period_index`.
/// Defaults to 0 — an unwritten bucket means nothing has been spent yet in
/// that period, which is also how a new period "resets" the accumulator.
pub fn read_spent(env: &Env, user: &Address, asset: &Symbol, period_index: u64) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::Spent(user.clone(), asset.clone(), period_index))
        .unwrap_or(0)
}

/// Writes accumulated spend for `(user, asset)` in period bucket `period_index`.
pub fn write_spent(env: &Env, user: &Address, asset: &Symbol, period_index: u64, spent: i128) {
    env.storage().persistent().set(
        &DataKey::Spent(user.clone(), asset.clone(), period_index),
        &spent,
    );
}
