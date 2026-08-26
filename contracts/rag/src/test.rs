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

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_document_versioning_flow() {
        let env = Env::default();
        env.mock_all_auths();
        let owner = Address::generate(&env);

        let doc_id = String::from_str(&env, "doc-v1");
        let hash_v1 = String::from_str(&env, "hash-v1");
        let hash_v2 = String::from_str(&env, "hash-v2");

        // Create document (Version 1)
        VersionedDocumentManager::create_document(&env, doc_id.clone(), hash_v1.clone(), owner.clone()).unwrap();

        let doc = VersionedDocumentManager::get_active_document(&env, doc_id.clone()).unwrap();
        assert_eq!(doc.active_version_id, 1);
        assert_eq!(doc.versions.len(), 1);

        // Append Version 2
        let new_ver = VersionedDocumentManager::append_version(&env, doc_id.clone(), hash_v2.clone(), owner.clone()).unwrap();
        assert_eq!(new_ver, 2);

        let updated_doc = VersionedDocumentManager::get_active_document(&env, doc_id.clone()).unwrap();
        assert_eq!(updated_doc.active_version_id, 2);
        assert_eq!(updated_doc.versions.len(), 2);

        // Verify historical version 1 is traceable
        let v1 = VersionedDocumentManager::get_version(&env, doc_id.clone(), 1).unwrap();
        assert_eq!(v1.content_hash, hash_v1);
        assert_eq!(v1.previous_version_id, None);

        // Verify historical version 2 references version 1
        let v2 = VersionedDocumentManager::get_version(&env, doc_id, 2).unwrap();
        assert_eq!(v2.content_hash, hash_v2);
        assert_eq!(v2.previous_version_id, Some(1));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_access_control_enforcement() {
        let env = Env::default();
        env.mock_all_auths();
        let owner = Address::generate(&env);
        let authorized_user = Address::generate(&env);
        let unauthorized_user = Address::generate(&env);

        let doc_id = String::from_str(&env, "doc-secured-1");

        let mut allowed = Vec::new(&env);
        allowed.push_back(authorized_user.clone());

        // Owner sets access policy
        let set_res = AccessControlManager::set_policy(&env, doc_id.clone(), allowed, owner.clone());
        assert!(set_res.is_ok());

        // Owner can access
        assert!(AccessControlManager::verify_access(&env, &doc_id, &owner).is_ok());

        // Authorized user can access
        assert!(AccessControlManager::verify_access(&env, &doc_id, &authorized_user).is_ok());

        // Unauthorized user fails
        let access_res = AccessControlManager::verify_access(&env, &doc_id, &unauthorized_user);
        assert!(access_res.is_err());
    }
}