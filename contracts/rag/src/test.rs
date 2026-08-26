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