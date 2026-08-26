#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_store_and_retrieve_document_source() {
        let env = Env::default();
        env.mock_all_auths();
        let creator = Address::generate(&env);

        let doc_id = String::from_str(&env, "doc-1");
        let hash = String::from_str(&env, "sha256-hash");
        let ref_id = String::from_str(&env, "bafybeicg64vxt...");

        let res = DocumentManager::store_document(
            &env,
            doc_id.clone(),
            hash,
            SourceType::IpfsCid,
            ref_id,
            creator,
        );
        assert!(res.is_ok());

        let retrieved = DocumentManager::get_document(&env, doc_id).unwrap();
        assert_eq!(retrieved.source.source_type, SourceType::IpfsCid);
    }

    #[test]
    #[should_panic(expected = "DocumentAlreadyExists")]
    fn test_prevents_silent_reference_overwrite() {
        let env = Env::default();
        env.mock_all_auths();
        let creator = Address::generate(&env);

        let doc_id = String::from_str(&env, "doc-2");
        let hash = String::from_str(&env, "hash");
        let ref_id = String::from_str(&env, "commit-abc");

        // First store succeeds
        DocumentManager::store_document(
            &env,
            doc_id.clone(),
            hash.clone(),
            SourceType::GitCommit,
            ref_id.clone(),
            creator.clone(),
        ).unwrap();

        // Second store with same ID should panic to prevent silent modification
        DocumentManager::store_document(
            &env,
            doc_id,
            hash,
            SourceType::GitCommit,
            ref_id,
            creator,
        ).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_register_and_retrieve_document_metadata() {
        let env = Env::default();
        env.mock_all_auths();
        let owner = Address::generate(&env);

        let doc_id = String::from_str(&env, "doc-meta-1");
        let hash = String::from_str(&env, "sha256-abc");

        let metadata = DocumentMetadata {
            title: String::from_str(&env, "Technical Specification"),
            mime_type: String::from_str(&env, "application/pdf"),
            version: String::from_str(&env, "1.0.0"),
            language: String::from_str(&env, "en"),
            source: SourceReference {
                source_type: SourceType::IpfsCid,
                reference: String::from_str(&env, "bafybeigdyrzt..."),
            },
            collection: String::from_str(&env, "architecture-docs"),
            creation_ledger: 1042,
        };

        let res = DocumentManager::register_document(
            &env,
            doc_id.clone(),
            hash,
            metadata.clone(),
            owner.clone(),
        );
        assert!(res.is_ok());

        let retrieved = DocumentManager::get_document(&env, doc_id).unwrap();
        assert_eq!(retrieved.metadata.title, metadata.title);
        assert_eq!(retrieved.metadata.creation_ledger, 1042);
    }

    #[test]
    fn test_authorized_metadata_update() {
        let env = Env::default();
        env.mock_all_auths();
        let owner = Address::generate(&env);

        let doc_id = String::from_str(&env, "doc-meta-2");
        let hash = String::from_str(&env, "sha256-def");

        let initial_metadata = DocumentMetadata {
            title: String::from_str(&env, "Draft Spec"),
            mime_type: String::from_str(&env, "text/plain"),
            version: String::from_str(&env, "0.1.0"),
            language: String::from_str(&env, "en"),
            source: SourceReference {
                source_type: SourceType::GitCommit,
                reference: String::from_str(&env, "abc1234"),
            },
            collection: String::from_str(&env, "drafts"),
            creation_ledger: 500,
        };

        DocumentManager::register_document(&env, doc_id.clone(), hash, initial_metadata, owner.clone()).unwrap();

        let updated_metadata = DocumentMetadata {
            title: String::from_str(&env, "Final Spec"),
            mime_type: String::from_str(&env, "application/pdf"),
            version: String::from_str(&env, "1.0.0"),
            language: String::from_str(&env, "en"),
            source: SourceReference {
                source_type: SourceType::GitCommit,
                reference: String::from_str(&env, "abc1234"),
            },
            collection: String::from_str(&env, "releases"),
            creation_ledger: 500,
        };

        let update_res = DocumentManager::update_metadata(&env, doc_id.clone(), updated_metadata, owner);
        assert!(update_res.is_ok());

        let retrieved = DocumentManager::get_document(&env, doc_id).unwrap();
        assert_eq!(retrieved.metadata.title, String::from_str(&env, "Final Spec"));
    }
}