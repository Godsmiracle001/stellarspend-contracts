use soroban_sdk::{Address, Env, Symbol};

use crate::types::{CategoryAssignment, DataKey, Period};

const ADMIN: &str = "ADMIN";

pub fn read_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&ADMIN)
}

pub fn write_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&ADMIN, admin);
}

pub fn read_assignment(env: &Env, tx_id: u64) -> Option<CategoryAssignment> {
    env.storage().persistent().get(&DataKey::Assignment(tx_id))
}

pub fn write_assignment(env: &Env, tx_id: u64, assignment: &CategoryAssignment) {
    env.storage()
        .persistent()
        .set(&DataKey::Assignment(tx_id), assignment);
}

pub fn read_category_total(
    env: &Env,
    owner: &Address,
    category: &Symbol,
    period: Period,
    period_index: u64,
) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::CategoryTotal(
            owner.clone(),
            category.clone(),
            period,
            period_index,
        ))
        .unwrap_or(0)
}

pub fn write_category_total(
    env: &Env,
    owner: &Address,
    category: &Symbol,
    period: Period,
    period_index: u64,
    total: i128,
) {
    env.storage().persistent().set(
        &DataKey::CategoryTotal(owner.clone(), category.clone(), period, period_index),
        &total,
    );
}
