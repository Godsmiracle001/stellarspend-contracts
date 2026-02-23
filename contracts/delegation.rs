#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, panic_with_error, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DelegationError {
    InvalidAddress = 1,
    InvalidAmount = 2,
    Unauthorized = 3,
    AmountTooLarge = 4,
}

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct Delegation {
    pub limit: i128,
    pub spent: i128,
}

#[derive(Clone)]
#[contracttype]
pub enum DelegationDataKey {
    Allowance(Address, Address), // Owner, Delegate
}

#[contract]
pub struct DelegationContract;

#[contractimpl]
impl DelegationContract {
    /// Authorize a delegate to spend up to a specific limit
    pub fn set_delegation(env: Env, owner: Address, delegate: Address, limit: i128) {
        owner.require_auth();
        
        if owner == delegate {
            panic_with_error!(&env, DelegationError::InvalidAddress);
        }
        if limit <= 0 {
            panic_with_error!(&env, DelegationError::InvalidAmount);
        }

        let key = DelegationDataKey::Allowance(owner.clone(), delegate.clone());
        let mut delegation: Delegation = env.storage().persistent().get(&key).unwrap_or(Delegation { limit: 0, spent: 0 });
        
        delegation.limit = limit;
        env.storage().persistent().set(&key, &delegation);

        // Emit delegated event
        env.events().publish((soroban_sdk::symbol_short!("delegate"), soroban_sdk::symbol_short!("set"), owner.clone(), delegate.clone()), limit);
    }

    /// Revoke a delegate's spending rights
    pub fn revoke_delegation(env: Env, owner: Address, delegate: Address) {
        owner.require_auth();

        let key = DelegationDataKey::Allowance(owner.clone(), delegate.clone());
        if env.storage().persistent().has(&key) {
            env.storage().persistent().remove(&key);
            
            // Emit revoked event
            env.events().publish((soroban_sdk::symbol_short!("delegate"), soroban_sdk::symbol_short!("revoked"), owner.clone(), delegate.clone()), ());
        }
    }

    /// Consume a portion of the delegate's allowance
    pub fn consume_allowance(env: Env, owner: Address, delegate: Address, amount: i128) -> Result<(), DelegationError> {
        delegate.require_auth();
        
        if amount <= 0 {
            return Err(DelegationError::InvalidAmount);
        }

        let key = DelegationDataKey::Allowance(owner.clone(), delegate.clone());
        
        if let Some(mut delegation) = env.storage().persistent().get::<_, Delegation>(&key) {
            let new_spent = delegation.spent.checked_add(amount).unwrap_or(i128::MAX);
            if new_spent > delegation.limit {
                return Err(DelegationError::AmountTooLarge);
            }

            delegation.spent = new_spent;
            env.storage().persistent().set(&key, &delegation);

            // Emit consumed event
            env.events().publish(
                (soroban_sdk::symbol_short!("delegate"), soroban_sdk::symbol_short!("consumed"), owner.clone(), delegate.clone()),
                amount,
            );

            Ok(())
        } else {
            Err(DelegationError::Unauthorized)
        }
    }

    /// Get the current delegation state
    pub fn get_delegation(env: Env, owner: Address, delegate: Address) -> Option<Delegation> {
        let key = DelegationDataKey::Allowance(owner, delegate);
        env.storage().persistent().get(&key)
    }
}

