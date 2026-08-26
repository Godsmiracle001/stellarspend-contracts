use soroban_sdk::{contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceType {
    IpfsCid,
    GitCommit,
    HttpsResource,
    AppGeneratedId,
    ExternalContentId,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct SourceReference {
    pub source_type: SourceType,
    pub reference: String, // Bounded reference identifier
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Document {
    pub id: String,
    pub content_hash: String,
    pub source: SourceReference,
    pub creator: Address,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Document(String),
}

pub struct DocumentManager;

impl DocumentManager {
    /// Stores a document with an explicitly recorded and immutable source reference.
    pub fn store_document(
        env: &Env,
        id: String,
        content_hash: String,
        source_type: SourceType,
        reference: String,
        creator: Address,
    ) -> Result<(), &'static str> {
        creator.require_auth();

        // Ensure source reference cannot silently overwrite an existing record
        if env.storage().persistent().has(&DataKey::Document(id.clone())) {
            return Err("DocumentAlreadyExists: source reference cannot be silently modified");
        }

        let source = SourceReference {
            source_type,
            reference,
        };

        let document = Document {
            id: id.clone(),
            content_hash,
            source,
            creator,
        };

        env.storage().persistent().set(&DataKey::Document(id), &document);
        Ok(())
    }

    /// Retrieves the stored document including its source reference.
    pub fn get_document(env: &Env, id: String) -> Result<Document, &'static str> {
        env.storage()
            .persistent()
            .get(&DataKey::Document(id))
            .ok_or("DocumentNotFound")
    }
}

use soroban_sdk::{contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceType {
    IpfsCid,
    GitCommit,
    HttpsResource,
    AppGeneratedId,
    ExternalContentId,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct SourceReference {
    pub source_type: SourceType,
    pub reference: String,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct DocumentMetadata {
    pub title: String,
    pub mime_type: String,
    pub version: String,
    pub language: String,
    pub source: SourceReference,
    pub collection: String,
    pub creation_ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Document {
    pub id: String,
    pub content_hash: String,
    pub metadata: DocumentMetadata,
    pub owner: Address,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Document(String),
}

pub struct DocumentManager;

impl DocumentManager {
    /// Registers a new document with bounded classification metadata and creation ledger tracking.
    pub fn register_document(
        env: &Env,
        id: String,
        content_hash: String,
        metadata: DocumentMetadata,
        owner: Address,
    ) -> Result<(), &'static str> {
        owner.require_auth();

        if env.storage().persistent().has(&DataKey::Document(id.clone())) {
            return Err("DocumentAlreadyExists");
        }

        let document = Document {
            id: id.clone(),
            content_hash,
            metadata,
            owner,
        };

        env.storage().persistent().set(&DataKey::Document(id), &document);
        Ok(())
    }

    /// Updates document metadata authorized strictly by the document owner.
    pub fn update_metadata(
        env: &Env,
        id: String,
        new_metadata: DocumentMetadata,
        caller: Address,
    ) -> Result<(), &'static str> {
        caller.require_auth();

        let mut document: Document = env.storage()
            .persistent()
            .get(&DataKey::Document(id.clone()))
            .ok_or("DocumentNotFound")?;

        if document.owner != caller {
            return Err("Unauthorized: only the document owner can update metadata");
        }

        document.metadata = new_metadata;
        env.storage().persistent().set(&DataKey::Document(id), &document);
        Ok(())
    }

    /// Retrieves the stored document record.
    pub fn get_document(env: &Env, id: String) -> Result<Document, &'static str> {
        env.storage()
            .persistent()
            .get(&DataKey::Document(id))
            .ok_or("DocumentNotFound")
    }
}