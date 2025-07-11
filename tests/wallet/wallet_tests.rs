#[cfg(test)]
mod tests {
    use rust_blockchain::wallet::Wallet;
    use rust_blockchain::util;
    use std::collections::HashSet;
    use ring::signature::KeyPair;

    // =============================================================================
    // WALLET CREATION TESTS
    // =============================================================================

    #[test]
    fn test_wallet_new() {
        let wallet = Wallet::new();
        
        // Verify wallet has valid keys
        assert!(!wallet.get_public_key().is_empty());
        assert!(!wallet.get_pkcs8().is_empty());
        
        // Verify key sizes are reasonable
        let public_key_len = wallet.get_public_key().len();
        let pkcs8_len = wallet.get_pkcs8().len();
        assert!(public_key_len > 0);
        assert!(pkcs8_len > 0);
        
        // Public key should be reasonable size for ECDSA P256
        // Uncompressed: 65 bytes, Compressed: 33 bytes, but ring may use different format
        assert!(wallet.get_public_key().len() >= 33);
        
        // PKCS8 key should be reasonable size
        assert!(wallet.get_pkcs8().len() >= 32);
    }

    #[test]
    fn test_wallet_new_generates_unique_keys() {
        let wallet1 = Wallet::new();
        let wallet2 = Wallet::new();
        
        // Each wallet should have unique keys
        assert_ne!(wallet1.get_public_key(), wallet2.get_public_key());
        assert_ne!(wallet1.get_pkcs8(), wallet2.get_pkcs8());
        
        // Each wallet should have unique addresses
        assert_ne!(wallet1.get_address(), wallet2.get_address());
    }

    #[test]
    fn test_wallet_new_multiple_instances() {
        let mut public_keys = HashSet::new();
        let mut private_keys = HashSet::new();
        let mut addresses = HashSet::new();
        
        // Create 50 wallets and ensure all are unique
        for _ in 0..50 {
            let wallet = Wallet::new();
            
            let pub_key = wallet.get_public_key().to_vec();
            let priv_key = wallet.get_pkcs8().to_vec();
            let address = wallet.get_address();
            
            // All should be unique
            assert!(public_keys.insert(pub_key), "Duplicate public key generated");
            assert!(private_keys.insert(priv_key), "Duplicate private key generated");
            assert!(addresses.insert(address), "Duplicate address generated");
        }
        
        assert_eq!(public_keys.len(), 50);
        assert_eq!(private_keys.len(), 50);
        assert_eq!(addresses.len(), 50);
    }

    // =============================================================================
    // WALLET DEFAULT TRAIT TESTS
    // =============================================================================

    #[test]
    fn test_wallet_default() {
        let wallet = Wallet::default();
        
        // Default should work like new()
        assert!(!wallet.get_public_key().is_empty());
        assert!(!wallet.get_pkcs8().is_empty());
        assert!(!wallet.get_address().is_empty());
    }

    #[test]
    fn test_wallet_default_vs_new() {
        let wallet_new = Wallet::new();
        let wallet_default = Wallet::default();
        
        // Both should have valid but different keys
        assert!(!wallet_new.get_public_key().is_empty());
        assert!(!wallet_default.get_public_key().is_empty());
        assert_ne!(wallet_new.get_public_key(), wallet_default.get_public_key());
        
        // Both should have valid but different addresses
        assert!(!wallet_new.get_address().is_empty());
        assert!(!wallet_default.get_address().is_empty());
        assert_ne!(wallet_new.get_address(), wallet_default.get_address());
    }

    // =============================================================================
    // ADDRESS GENERATION TESTS
    // =============================================================================

    #[test]
    fn test_get_address_basic() {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        
        // Address should not be empty
        assert!(!address.is_empty());
        
        // Address should be base58 encoded string with reasonable length
        assert!(address.len() > 20); // Base58 addresses are typically longer
        assert!(address.len() < 100); // But not excessively long
        
        // Address should contain only valid base58 characters
        let valid_chars = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
        for ch in address.chars() {
            assert!(valid_chars.contains(ch), "Invalid base58 character: {ch}");
        }
    }

    #[test]
    fn test_get_address_deterministic() {
        let wallet = Wallet::new();
        
        // Multiple calls should return the same address
        let address1 = wallet.get_address();
        let address2 = wallet.get_address();
        let address3 = wallet.get_address();
        
        assert_eq!(address1, address2);
        assert_eq!(address2, address3);
        assert_eq!(address1, address3);
    }

    #[test]
    fn test_get_address_unique_per_wallet() {
        let mut addresses = HashSet::new();
        
        // Generate 100 wallets and verify all addresses are unique
        for _ in 0..100 {
            let wallet = Wallet::new();
            let address = wallet.get_address();
            
            assert!(addresses.insert(address.clone()), 
                "Duplicate address generated: {address}");
        }
        
        assert_eq!(addresses.len(), 100);
    }

    #[test]
    fn test_address_structure_validity() {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        
        // Decode the address to verify its structure
        let decoded = util::base58_decode(&address);
        assert!(decoded.len() >= 5, "Address too short after decoding"); // version + hash + checksum
        
        // First byte should be version (0x00)
        assert_eq!(decoded[0], 0x00, "Invalid version byte");        
    }

    // =============================================================================
    // PUBLIC KEY TESTS
    // =============================================================================

    #[test]
    fn test_get_public_key_basic() {
        let wallet = Wallet::new();
        let public_key = wallet.get_public_key();
        
        // Public key should not be empty
        assert!(!public_key.is_empty());
        
        // Should be reasonable size for ECDSA keys
        assert!(public_key.len() >= 33); // At least compressed key size
        assert!(public_key.len() <= 65); // At most uncompressed key size
    }

    #[test]
    fn test_get_public_key_deterministic() {
        let wallet = Wallet::new();
        
        // Multiple calls should return the same key
        let key1 = wallet.get_public_key();
        let key2 = wallet.get_public_key();
        let key3 = wallet.get_public_key();
        
        assert_eq!(key1, key2);
        assert_eq!(key2, key3);
        assert_eq!(key1, key3);
    }

    #[test]
    fn test_get_public_key_unique_per_wallet() {
        let mut public_keys = HashSet::new();
        
        // Generate 50 wallets and verify all public keys are unique
        for _ in 0..50 {
            let wallet = Wallet::new();
            let pub_key = wallet.get_public_key().to_vec();
            
            assert!(public_keys.insert(pub_key.clone()), 
                "Duplicate public key generated");
        }
        
        assert_eq!(public_keys.len(), 50);
    }

    #[test]
    fn test_public_key_address_relationship() {
        let wallet = Wallet::new();
        let public_key = wallet.get_public_key();
        let address = wallet.get_address();
        
        // The address should be derivable from the public key
        // This tests the relationship between public key and address
        let pub_key_hash = rust_blockchain::wallet::wallet_util::hash_pub_key(public_key);
        let derived_address = rust_blockchain::wallet::wallet_util::convert_address(&pub_key_hash);
        
        assert_eq!(address, derived_address, 
            "Address should be derivable from public key");
    }

    // =============================================================================
    // PRIVATE KEY (PKCS8) TESTS
    // =============================================================================

    #[test]
    fn test_get_pkcs8_basic() {
        let wallet = Wallet::new();
        let pkcs8 = wallet.get_pkcs8();
        
        // PKCS8 key should not be empty
        assert!(!pkcs8.is_empty());
        
        // Should be reasonable size for PKCS8 encoded ECDSA key
        assert!(pkcs8.len() >= 100); // PKCS8 encoding adds overhead
        assert!(pkcs8.len() <= 1000); // But shouldn't be excessively large
    }

    #[test]
    fn test_get_pkcs8_deterministic() {
        let wallet = Wallet::new();
        
        // Multiple calls should return the same key
        let key1 = wallet.get_pkcs8();
        let key2 = wallet.get_pkcs8();
        let key3 = wallet.get_pkcs8();
        
        assert_eq!(key1, key2);
        assert_eq!(key2, key3);
        assert_eq!(key1, key3);
    }

    #[test]
    fn test_get_pkcs8_unique_per_wallet() {
        let mut private_keys = HashSet::new();
        
        // Generate 50 wallets and verify all private keys are unique
        for _ in 0..50 {
            let wallet = Wallet::new();
            let priv_key = wallet.get_pkcs8().to_vec();
            
            assert!(private_keys.insert(priv_key.clone()), 
                "Duplicate private key generated");
        }
        
        assert_eq!(private_keys.len(), 50);
    }

    #[test]
    fn test_pkcs8_can_recreate_public_key() {
        let wallet = Wallet::new();
        let pkcs8 = wallet.get_pkcs8();
        let original_public_key = wallet.get_public_key();
        
        // We should be able to recreate the public key from PKCS8
        use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair};
        use ring::rand::SystemRandom;
        
        let rng = SystemRandom::new();
        let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8, &rng)
            .expect("Should be able to recreate key pair from PKCS8");
        
        let recreated_public_key = key_pair.public_key().as_ref();
        
        assert_eq!(original_public_key, recreated_public_key, 
            "Public key should be recreatable from PKCS8");
    }

    // =============================================================================
    // SERIALIZATION TESTS
    // =============================================================================

    #[test]
    fn test_wallet_serialization() {
        let wallet = Wallet::new();
        
        // Test bincode serialization
        let serialized = bincode::encode_to_vec(&wallet, bincode::config::standard())
            .expect("Wallet should be serializable");
        
        assert!(!serialized.is_empty());
        
        // Test deserialization
        let (deserialized_wallet, _): (Wallet, usize) = 
            bincode::decode_from_slice(&serialized, bincode::config::standard())
                .expect("Wallet should be deserializable");
        
        // Verify the deserialized wallet is identical
        assert_eq!(wallet.get_public_key(), deserialized_wallet.get_public_key());
        assert_eq!(wallet.get_pkcs8(), deserialized_wallet.get_pkcs8());
        assert_eq!(wallet.get_address(), deserialized_wallet.get_address());
    }

    #[test]
    fn test_wallet_serialization_roundtrip() {
        let original_wallet = Wallet::new();
        
        // Multiple serialization/deserialization cycles
        let mut current_wallet = original_wallet.clone();
        for _ in 0..5 {
            let serialized = bincode::encode_to_vec(&current_wallet, bincode::config::standard())
                .expect("Wallet should be serializable");
            
            let (deserialized, _): (Wallet, usize) = 
                bincode::decode_from_slice(&serialized, bincode::config::standard())
                    .expect("Wallet should be deserializable");
            
            current_wallet = deserialized;
        }
        
        // Verify integrity after multiple cycles
        assert_eq!(original_wallet.get_public_key(), current_wallet.get_public_key());
        assert_eq!(original_wallet.get_pkcs8(), current_wallet.get_pkcs8());
        assert_eq!(original_wallet.get_address(), current_wallet.get_address());
    }

    // =============================================================================
    // CLONE TESTS
    // =============================================================================

    #[test]
    fn test_wallet_clone() {
        let original_wallet = Wallet::new();
        let cloned_wallet = original_wallet.clone();
        
        // Verify clone has same values
        assert_eq!(original_wallet.get_public_key(), cloned_wallet.get_public_key());
        assert_eq!(original_wallet.get_pkcs8(), cloned_wallet.get_pkcs8());
        assert_eq!(original_wallet.get_address(), cloned_wallet.get_address());
    }

    #[test]
    fn test_wallet_clone_independence() {
        let wallet1 = Wallet::new();
        let wallet2 = wallet1.clone();
        
        // They should have the same data
        assert_eq!(wallet1.get_address(), wallet2.get_address());
        
        // But accessing one shouldn't affect the other
        let address1_first = wallet1.get_address();
        let address2_first = wallet2.get_address();
        let address1_second = wallet1.get_address();
        let address2_second = wallet2.get_address();
        
        assert_eq!(address1_first, address1_second);
        assert_eq!(address2_first, address2_second);
        assert_eq!(address1_first, address2_first);
    }

    // =============================================================================
    // INTEGRATION TESTS
    // =============================================================================

    #[test]
    fn test_wallet_address_validation_integration() {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        
        // The generated address should be valid according to the validation function
        assert!(rust_blockchain::wallet::wallet_util::validate_address(&address),
            "Generated wallet address should be valid: {address}");
    }

    #[test]
    fn test_wallet_cryptographic_operations() {
        let wallet = Wallet::new();
        let message = b"test message for signing";
        
        // Test that we can sign with the private key
        let signature = util::ecdsa_p256_sha256_sign_digest(wallet.get_pkcs8(), message);
        assert!(!signature.is_empty(), "Signature should not be empty");
        
        // Test that we can verify with the public key
        let verification_result = util::ecdsa_p256_sha256_sign_verify(
            wallet.get_public_key(), 
            &signature, 
            message
        );
        assert!(verification_result, "Signature verification should succeed");
    }

    #[test]
    fn test_wallet_signature_uniqueness() {
        let wallet = Wallet::new();
        let message = b"test message";
        
        // Due to randomness in signing, multiple signatures should be different
        let sig1 = util::ecdsa_p256_sha256_sign_digest(wallet.get_pkcs8(), message);
        let sig2 = util::ecdsa_p256_sha256_sign_digest(wallet.get_pkcs8(), message);
        
        // Signatures should be different (due to random nonce in ECDSA)
        assert_ne!(sig1, sig2, "ECDSA signatures should have randomness");
        
        // But both should verify correctly
        assert!(util::ecdsa_p256_sha256_sign_verify(wallet.get_public_key(), &sig1, message));
        assert!(util::ecdsa_p256_sha256_sign_verify(wallet.get_public_key(), &sig2, message));
    }

    #[test]
    fn test_wallet_cross_verification_fails() {
        let wallet1 = Wallet::new();
        let wallet2 = Wallet::new();
        let message = b"test message";
        
        // Sign with wallet1
        let signature = util::ecdsa_p256_sha256_sign_digest(wallet1.get_pkcs8(), message);
        
        // Verification with wallet1's public key should succeed
        assert!(util::ecdsa_p256_sha256_sign_verify(
            wallet1.get_public_key(), 
            &signature, 
            message
        ));
        
        // Verification with wallet2's public key should fail
        assert!(!util::ecdsa_p256_sha256_sign_verify(
            wallet2.get_public_key(), 
            &signature, 
            message
        ));
    }

    // =============================================================================
    // PERFORMANCE TESTS
    // =============================================================================

    #[test]
    fn test_wallet_creation_performance() {
        let start = std::time::Instant::now();
        
        // Create 100 wallets
        for _ in 0..100 {
            let _wallet = Wallet::new();
        }
        
        let duration = start.elapsed();
        
        // Should complete in reasonable time (< 5 seconds for 100 wallets)
        assert!(duration.as_secs() < 5, 
            "Wallet creation too slow: {}ms for 100 wallets", duration.as_millis());
    }

    #[test]
    fn test_address_generation_performance() {
        let wallet = Wallet::new();
        let start = std::time::Instant::now();
        
        // Generate 1000 addresses from same wallet
        for _ in 0..1000 {
            let _address = wallet.get_address();
        }
        
        let duration = start.elapsed();
        
        // Should complete in reasonable time (< 1 second for 1000 addresses)
        assert!(duration.as_millis() < 1000, 
            "Address generation too slow: {}ms for 1000 operations", duration.as_millis());
    }

    // =============================================================================
    // EDGE CASE TESTS
    // =============================================================================

    #[test]
    fn test_wallet_methods_consistency() {
        let wallet = Wallet::new();
        
        // All methods should return consistent data across multiple calls
        let pub_key1 = wallet.get_public_key().to_vec();
        let pub_key2 = wallet.get_public_key().to_vec();
        let pkcs8_1 = wallet.get_pkcs8().to_vec();
        let pkcs8_2 = wallet.get_pkcs8().to_vec();
        let address1 = wallet.get_address();
        let address2 = wallet.get_address();
        
        assert_eq!(pub_key1, pub_key2);
        assert_eq!(pkcs8_1, pkcs8_2);
        assert_eq!(address1, address2);
    }

    #[test]
    fn test_wallet_data_relationships() {
        let wallet = Wallet::new();
        
        // Verify that the wallet data maintains proper cryptographic relationships
        let public_key = wallet.get_public_key();
        let pkcs8 = wallet.get_pkcs8();
        let address = wallet.get_address();
        
        // Public key should be derivable from PKCS8
        use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair};
        use ring::rand::SystemRandom;
        
        let rng = SystemRandom::new();
        let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8, &rng)
            .expect("Should be able to create key pair from PKCS8");
        
        let derived_public_key = key_pair.public_key().as_ref();
        assert_eq!(public_key, derived_public_key);
        
        // Address should be derivable from public key
        let pub_key_hash = rust_blockchain::wallet::wallet_util::hash_pub_key(public_key);
        let derived_address = rust_blockchain::wallet::wallet_util::convert_address(&pub_key_hash);
        assert_eq!(address, derived_address);
    }

    #[test]
    fn test_multiple_wallets_independent() {
        let wallet1 = Wallet::new();
        let wallet2 = Wallet::new();
        let wallet3 = Wallet::new();
        
        // All wallets should be completely independent
        assert_ne!(wallet1.get_public_key(), wallet2.get_public_key());
        assert_ne!(wallet1.get_public_key(), wallet3.get_public_key());
        assert_ne!(wallet2.get_public_key(), wallet3.get_public_key());
        
        assert_ne!(wallet1.get_pkcs8(), wallet2.get_pkcs8());
        assert_ne!(wallet1.get_pkcs8(), wallet3.get_pkcs8());
        assert_ne!(wallet2.get_pkcs8(), wallet3.get_pkcs8());
        
        assert_ne!(wallet1.get_address(), wallet2.get_address());
        assert_ne!(wallet1.get_address(), wallet3.get_address());
        assert_ne!(wallet2.get_address(), wallet3.get_address());
    }

    #[test]
    fn test_wallet_memory_safety() {
        // Test that wallet operations don't cause memory issues
        let mut wallets = Vec::new();
        
        // Create many wallets and store them
        for _ in 0..100 {
            wallets.push(Wallet::new());
        }
        
        // Access all wallets multiple times
        for wallet in &wallets {
            let _pub_key = wallet.get_public_key();
            let _pkcs8 = wallet.get_pkcs8();
            let _address = wallet.get_address();
        }
        
        // Drop all wallets (implicit when going out of scope)
        drop(wallets);
        
        // Create new wallet after dropping others
        let new_wallet = Wallet::new();
        assert!(!new_wallet.get_address().is_empty());
    }
} 
