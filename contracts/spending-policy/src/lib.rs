#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, Address, Env, Vec};

mod storage;
#[cfg(test)]
mod test;
pub mod types;
pub mod validation;

use types::PolicyRule;

/// Typed errors for the spending_policy contract.
#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Contract has already been initialized.
    AlreadyInitialized = 1,
    /// Caller is not authorized to perform this action.
    Unauthorized = 2,
    /// A rule's `limit` or `zk_required_above` is negative.
    InvalidAmount = 3,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Initializes the contract with an administrator. As with the other
    /// spending-* contracts, the admin has no power over per-user policy
    /// data — kept for deployment-tooling consistency and future use.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if storage::read_admin(&env).is_some() {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        storage::write_admin(&env, &admin);
        Ok(())
    }

    /// Atomically replaces `user`'s entire rule set with `rules`. Only
    /// `user` may set their own policy. This is a full replacement, not a
    /// merge — any rules not present in the new set are gone.
    pub fn set_policy(env: Env, user: Address, rules: Vec<PolicyRule>) -> Result<(), Error> {
        user.require_auth();
        validation::validate_rules(&rules)?;
        storage::write_policy(&env, &user, &rules);

        env.events().publish(
            (symbol_short!("policy"), symbol_short!("set"), user.clone()),
            rules.len(),
        );
        Ok(())
    }

    /// Returns `user`'s current rule set (empty if never set).
    pub fn get_policy(env: Env, user: Address) -> Vec<PolicyRule> {
        storage::read_policy(&env, &user)
    }
}
