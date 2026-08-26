#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_document_hash_commitment_flow() {
        let env = Env::default();
        env.mock_all_auths();
        let owner = Address::generate(&env);

        let doc_id = String::from_str(&env, "doc-hash-1");
        let hash_v1 = String::from_str(&env, "sha256-hash-v1");
        let hash_v2 = String::from_str(&env, "sha256-hash-v2");

        // Register document successfully
        let res = DocumentCommitmentManager::register_document(&env, doc_id.clone(), hash_v1.clone(), owner.clone());
        assert!(res.is_ok());

        // Retrieve and verify hash integrity for version 1
        let retrieved_hash = DocumentCommitmentManager::get_hash_for_version(&env, doc_id.clone(), 1).unwrap();
        assert_eq!(retrieved_hash, hash_v1);

        // Commit new version with a new hash
        let new_ver = DocumentCommitmentManager::commit_new_version(&env, doc_id.clone(), hash_v2.clone(), owner.clone());
        assert!(new_ver.is_ok());
        assert_eq!(new_ver.unwrap(), 2);

        // Verify version 1 hash is immutable and version 2 hash is correctly stored
        assert_eq!(DocumentCommitmentManager::get_hash_for_version(&env, doc_id.clone(), 1).unwrap(), hash_v1);
        assert_eq!(DocumentCommitmentManager::get_hash_for_version(&env, doc_id, 2).unwrap(), hash_v2);
    }

    #[test]
    #[should_panic(expected = "InvalidHash")]
    fn test_rejects_empty_hash() {
        let env = Env::default();
        env.mock_all_auths();
        let owner = Address::generate(&env);

        let doc_id = String::from_str(&env, "doc-empty-hash");
        let empty_hash = String::from_str(&env, "");

        DocumentCommitmentManager::register_document(&env, doc_id, empty_hash, owner).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_collection_version_tracking_flow() {
        let env = Env::default();
        env.mock_all_auths();
        let owner = Address::generate(&env);
        let user = Address::generate(&env);

        let col_id = String::from_str(&env, "ai-docs-v1");
        let doc_id = String::from_str(&env, "doc-ref-101");
        let record_id = String::from_str(&env, "retrieval-rec-1");

        // 1. Create collection (initial version 1)
        CollectionVersionManager::create_collection(&env, col_id.clone(), owner.clone()).unwrap();

        // 2. Add document and check deterministic version increment to 2
        let new_ver = CollectionVersionManager::add_document_to_collection(
            &env,
            col_id.clone(),
            doc_id,
            owner.clone(),
        ).unwrap();
        assert_eq!(new_ver, 2);

        // 3. Record retrieval referencing the current version (2)
        let retrieval_record = CollectionVersionManager::record_retrieval(
            &env,
            record_id,
            col_id,
            user,
        ).unwrap();

        assert_eq!(retrieval_record.collection_version, 2);
    }
}