use soroban_sdk::{contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentVersion {
    pub version_id: u32,
    pub content_hash: String,
    pub creation_ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct VersionedDocument {
    pub id: String,
    pub owner: Address,
    pub active_version_id: u32,
    pub versions: Vec<DocumentVersion>,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    VersionedDoc(String),
}

pub struct DocumentCommitmentManager;

impl DocumentCommitmentManager {
    /// Registers a document with a mandatory, non-empty content hash commitment.
    pub fn register_document(
        env: &Env,
        id: String,
        content_hash: String,
        owner: Address,
    ) -> Result<(), &'static str> {
        owner.require_auth();

        if content_hash.len() == 0 {
            return Err("InvalidHash: document content hash cannot be empty");
        }

        if env.storage().persistent().has(&DataKey::VersionedDoc(id.clone())) {
            return Err("DocumentAlreadyExists");
        }

        let initial_version = DocumentVersion {
            version_id: 1,
            content_hash,
            creation_ledger: env.ledger().sequence(),
        };

        let mut versions = Vec::new(env);
        versions.push_back(initial_version);

        let document = VersionedDocument {
            id: id.clone(),
            owner,
            active_version_id: 1,
            versions,
        };

        env.storage().persistent().set(&DataKey::VersionedDoc(id), &document);
        Ok(())
    }

    /// Appends a new version with a new hash commitment, leaving existing versions immutable.
    pub fn commit_new_version(
        env: &Env,
        id: String,
        new_content_hash: String,
        caller: Address,
    ) -> Result<u32, &'static str> {
        caller.require_auth();

        if new_content_hash.len() == 0 {
            return Err("InvalidHash: new content hash cannot be empty");
        }

        let mut document: VersionedDocument = env.storage()
            .persistent()
            .get(&DataKey::VersionedDoc(id.clone()))
            .ok_or("DocumentNotFound")?;

        if document.owner != caller {
            return Err("Unauthorized: only the document owner can commit a new version");
        }

        let next_version_id = document.versions.len() + 1;
        let new_version = DocumentVersion {
            version_id: next_version_id,
            content_hash: new_content_hash,
            creation_ledger: env.ledger().sequence(),
        };

        document.versions.push_back(new_version);
        document.active_version_id = next_version_id;

        env.storage().persistent().set(&DataKey::VersionedDoc(id), &document);
        Ok(next_version_id)
    }

    /// Retrieves the content hash for a specific version to perform integrity verification.
    pub fn get_hash_for_version(
        env: &Env,
        id: String,
        version_id: u32,
    ) -> Result<String, &'static str> {
        let document: VersionedDocument = env.storage()
            .persistent()
            .get(&DataKey::VersionedDoc(id))
            .ok_or("DocumentNotFound")?;

        for v in document.versions.iter() {
            if v.version_id == version_id {
                return Ok(v.content_hash);
            }
        }

        Err("VersionNotFound")
    }
}