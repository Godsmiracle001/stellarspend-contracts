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