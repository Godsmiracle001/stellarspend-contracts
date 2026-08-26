use soroban_sdk::{contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct KnowledgeCollection {
    pub collection_id: String,
    pub owner: Address,
    pub current_version: u32,
    pub document_ids: Vec<String>,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct RetrievalRecord {
    pub record_id: String,
    pub collection_id: String,
    pub collection_version: u32,
    pub queried_by: Address,
    pub creation_ledger: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Collection(String),
    RetrievalRecord(String),
}

pub struct CollectionVersionManager;

impl CollectionVersionManager {
    /// Creates a new knowledge collection starting at version 1.
    pub fn create_collection(
        env: &Env,
        collection_id: String,
        owner: Address,
    ) -> Result<(), &'static str> {
        owner.require_auth();

        if env.storage().persistent().has(&DataKey::Collection(collection_id.clone())) {
            return Err("CollectionAlreadyExists");
        }

        let collection = KnowledgeCollection {
            collection_id: collection_id.clone(),
            owner,
            current_version: 1,
            document_ids: Vec::new(env),
        };

        env.storage().persistent().set(&DataKey::Collection(collection_id), &collection);
        Ok(())
    }

    /// Adds a document to the collection and deterministically increments the collection version.
    pub fn add_document_to_collection(
        env: &Env,
        collection_id: String,
        document_id: String,
        caller: Address,
    ) -> Result<u32, &'static str> {
        caller.require_auth();

        let mut collection: KnowledgeCollection = env.storage()
            .persistent()
            .get(&DataKey::Collection(collection_id.clone()))
            .ok_or("CollectionNotFound")?;

        if collection.owner != caller {
            return Err("Unauthorized: only collection owner can modify contents");
        }

        collection.document_ids.push_back(document_id);
        collection.current_version += 1; // Deterministic version increment

        env.storage().persistent().set(&DataKey::Collection(collection_id), &collection);
        Ok(collection.current_version)
    }

    /// Records a retrieval action referencing the exact knowledge collection version used.
    pub fn record_retrieval(
        env: &Env,
        record_id: String,
        collection_id: String,
        caller: Address,
    ) -> Result<RetrievalRecord, &'static str> {
        caller.require_auth();

        let collection: KnowledgeCollection = env.storage()
            .persistent()
            .get(&DataKey::Collection(collection_id.clone()))
            .ok_or("CollectionNotFound")?;

        let record = RetrievalRecord {
            record_id: record_id.clone(),
            collection_id,
            collection_version: collection.current_version,
            queried_by: caller,
            creation_ledger: env.ledger().sequence(),
        };

        env.storage().persistent().set(&DataKey::RetrievalRecord(record_id), &record);
        Ok(record)
    }
}