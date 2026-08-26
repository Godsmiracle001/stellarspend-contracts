use soroban_sdk::{contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct AccessPolicy {
    pub document_id: String,
    pub owner: Address,
    pub allowed_users: Vec<Address>,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    AccessPolicy(String),
}

pub struct AccessControlManager;

impl AccessControlManager {
    /// Sets or updates the access control policy for a specific document.
    pub fn set_policy(
        env: &Env,
        document_id: String,
        allowed_users: Vec<Address>,
        caller: Address,
    ) -> Result<(), &'static str> {
        caller.require_auth();

        // Check if policy already exists to verify ownership
        if let Some(existing_policy) = env.storage().persistent().get::<_, AccessPolicy>(&DataKey::AccessPolicy(document_id.clone())) {
            if existing_policy.owner != caller {
                return Err("Unauthorized: only the document owner can modify access policy");
            }
        }

        let policy = AccessPolicy {
            document_id: document_id.clone(),
            owner: caller,
            allowed_users,
        };

        env.storage().persistent().set(&DataKey::AccessPolicy(document_id), &policy);
        Ok(())
    }

    /// Validates whether a caller is authorized to retrieve document metadata.
    pub fn verify_access(
        env: &Env,
        document_id: &String,
        caller: &Address,
    ) -> Result<(), &'static str> {
        let policy: AccessPolicy = env.storage()
            .persistent()
            .get(&DataKey::AccessPolicy(document_id.clone()))
            .ok_or("AccessPolicyNotFound")?;

        // Owners always have access
        if &policy.owner == caller {
            return Ok(());
        }

        // Check if caller is explicitly allowed
        for user in policy.allowed_users.iter() {
            if &user == caller {
                return Ok(());
            }
        }

        Err("AccessDenied: caller is not authorized to access this document")
    }
}