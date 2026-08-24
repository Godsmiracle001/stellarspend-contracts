use soroban_sdk::{Address, Env, Vec};

use crate::types::{DataKey, PolicyRule};

const ADMIN: &str = "ADMIN";

pub fn read_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&ADMIN)
}

pub fn write_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&ADMIN, admin);
}

/// Returns `user`'s current rule set, or an empty Vec if none has been set.
pub fn read_policy(env: &Env, user: &Address) -> Vec<PolicyRule> {
    env.storage()
        .persistent()
        .get(&DataKey::Policy(user.clone()))
        .unwrap_or_else(|| Vec::new(env))
}

/// Overwrites `user`'s entire rule set in a single storage write — this IS
/// the atomic replacement: Soroban has no partial-write concept within one
/// invocation, so a reader either sees the previous complete Vec or the new
/// complete Vec, never a mix.
pub fn write_policy(env: &Env, user: &Address, rules: &Vec<PolicyRule>) {
    env.storage()
        .persistent()
        .set(&DataKey::Policy(user.clone()), rules);
}
