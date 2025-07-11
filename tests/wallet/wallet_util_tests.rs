#[cfg(test)]
mod tests {
    use rust_blockchain::wallet::wallet_util::{
        checksum, convert_address, hash_pub_key, validate_address,
    };
    use rust_blockchain::wallet::{ADDRESS_CHECK_SUM_LEN, VERSION};
    use rust_blockchain::util;

    // =============================================================================
    // CHECKSUM FUNCTION TESTS
    // =============================================================================

    #[test]
    fn test_checksum_basic() {
        let payload = b"test_payload";
        let result = checksum(payload);
        
        assert_eq!(result.len(), ADDRESS_CHECK_SUM_LEN);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_checksum_empty_payload() {
        let payload = b"";
        let result = checksum(payload);
        
        assert_eq!(result.len(), ADDRESS_CHECK_SUM_LEN);
    }

    #[test]
    fn test_checksum_deterministic() {
        let payload = b"deterministic_test";
        let result1 = checksum(payload);
        let result2 = checksum(payload);
        
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_checksum_different_inputs() {
        let payload1 = b"test1";
        let payload2 = b"test2";
        let result1 = checksum(payload1);
        let result2 = checksum(payload2);
        
        assert_ne!(result1, result2);
    }

    #[test]
    fn test_checksum_large_payload() {
        let payload = vec![0u8; 1000];
        let result = checksum(&payload);
        
        assert_eq!(result.len(), ADDRESS_CHECK_SUM_LEN);
    }

    // =============================================================================
    // HASH_PUB_KEY FUNCTION TESTS
    // =============================================================================

    #[test]
    fn test_hash_pub_key_basic() {
        let pub_key = b"test_public_key";
        let result = hash_pub_key(pub_key);
        
        // RIPEMD160 produces 20-byte hash
        assert_eq!(result.len(), 20);
    }

    #[test]
    fn test_hash_pub_key_empty() {
        let pub_key = b"";
        let result = hash_pub_key(pub_key);
        
        assert_eq!(result.len(), 20);
    }

    #[test]
    fn test_hash_pub_key_deterministic() {
        let pub_key = b"deterministic_key";
        let result1 = hash_pub_key(pub_key);
        let result2 = hash_pub_key(pub_key);
        
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_hash_pub_key_different_keys() {
        let pub_key1 = b"key1";
        let pub_key2 = b"key2";
        let result1 = hash_pub_key(pub_key1);
        let result2 = hash_pub_key(pub_key2);
        
        assert_ne!(result1, result2);
    }

    #[test]
    fn test_hash_pub_key_large_key() {
        let pub_key = vec![1u8; 256];
        let result = hash_pub_key(&pub_key);
        
        assert_eq!(result.len(), 20);
    }

    // =============================================================================
    // CONVERT_ADDRESS FUNCTION TESTS
    // =============================================================================

    #[test]
    fn test_convert_address_basic() {
        let pub_hash_key = b"test_hash_key";
        let address = convert_address(pub_hash_key);
        
        assert!(!address.is_empty());
        // Base58 encoded addresses should be reasonable length
        assert!(address.len() > 10);
    }

    #[test]
    fn test_convert_address_empty_hash() {
        let pub_hash_key = b"";
        let address = convert_address(pub_hash_key);
        
        assert!(!address.is_empty());
    }

    #[test]
    fn test_convert_address_deterministic() {
        let pub_hash_key = b"deterministic_hash";
        let address1 = convert_address(pub_hash_key);
        let address2 = convert_address(pub_hash_key);
        
        assert_eq!(address1, address2);
    }

    #[test]
    fn test_convert_address_different_hashes() {
        let pub_hash_key1 = b"hash1";
        let pub_hash_key2 = b"hash2";
        let address1 = convert_address(pub_hash_key1);
        let address2 = convert_address(pub_hash_key2);
        
        assert_ne!(address1, address2);
    }

    #[test]
    fn test_convert_address_structure() {
        let pub_hash_key = b"test_key";
        let address = convert_address(pub_hash_key);
        
        // Decode the address to verify structure
        let decoded = util::base58_decode(&address);
        assert!(decoded.len() >= 1 + pub_hash_key.len() + ADDRESS_CHECK_SUM_LEN);
        
        // Check version byte
        assert_eq!(decoded[0], VERSION);
        
        // Check pub_key_hash portion
        let embedded_hash = &decoded[1..1 + pub_hash_key.len()];
        assert_eq!(embedded_hash, pub_hash_key);
        
        // Check checksum length
        let checksum_part = &decoded[decoded.len() - ADDRESS_CHECK_SUM_LEN..];
        assert_eq!(checksum_part.len(), ADDRESS_CHECK_SUM_LEN);
    }

    // =============================================================================
    // VALIDATE_ADDRESS FUNCTION TESTS
    // =============================================================================

    #[test]
    fn test_validate_address_valid() {
        let pub_hash_key = b"valid_test_key";
        let address = convert_address(pub_hash_key);
        
        assert!(validate_address(&address));
    }

    #[test]
    fn test_validate_address_invalid_empty() {
        assert!(!validate_address(""));
    }

    #[test]
    fn test_validate_address_invalid_short() {
        assert!(!validate_address("short"));
    }

    #[test]
    fn test_validate_address_invalid_base58() {
        // Invalid base58 characters
        assert!(!validate_address("0OIl"));
    }

    #[test]
    fn test_validate_address_corrupted_checksum() {
        let pub_hash_key = b"test_key_for_corruption";
        let address = convert_address(pub_hash_key);
        
        // Corrupt the last character (which affects checksum)
        let mut chars: Vec<char> = address.chars().collect();
        if let Some(last_char) = chars.last_mut() {
            *last_char = if *last_char == '1' { '2' } else { '1' };
        }
        let corrupted_address: String = chars.into_iter().collect();
        
        assert!(!validate_address(&corrupted_address));
    }

    #[test]
    fn test_validate_address_wrong_version() {
        // Create address with wrong version
        let pub_hash_key = b"test_key";
        let mut payload = vec![];
        payload.push(0x01); // Wrong version (should be 0x00)
        payload.extend(pub_hash_key);
        let checksum_val = checksum(&payload);
        payload.extend(checksum_val);
        let invalid_address = util::base58_encode(&payload);
        
        assert!(!validate_address(&invalid_address));
    }

    // =============================================================================
    // ROUND-TRIP AND INTEGRATION TESTS
    // =============================================================================

    #[test]
    fn test_round_trip_address_creation_validation() {
        let test_cases = vec![
            b"short".to_vec(),
            b"medium_length_key".to_vec(),
            b"very_long_public_key_hash_for_testing_purposes".to_vec(),
            vec![0u8; 32], // 32 zero bytes
            vec![255u8; 20], // 20 max bytes
            (0..33).collect::<Vec<u8>>(), // Sequential bytes
        ];

        for pub_hash_key in test_cases {
            let address = convert_address(&pub_hash_key);
            assert!(validate_address(&address), 
                "Round-trip failed for pub_hash_key: {pub_hash_key:?}");
        }
    }

    #[test]
    fn test_pub_key_to_address_workflow() {
        let pub_key = b"sample_public_key_data";
        let pub_hash = hash_pub_key(pub_key);
        let address = convert_address(&pub_hash);
        
        assert!(validate_address(&address));
        assert_eq!(pub_hash.len(), 20); // RIPEMD160 output length
    }

    #[test]
    fn test_multiple_keys_unique_addresses() {
        let mut addresses = std::collections::HashSet::new();
        
        for i in 0..100 {
            let pub_key = format!("test_key_{i}");
            let pub_hash = hash_pub_key(pub_key.as_bytes());
            let address = convert_address(&pub_hash);
            
            assert!(validate_address(&address));
            assert!(addresses.insert(address.clone()), 
                "Duplicate address generated for key {i}: {address}");
        }
        
        assert_eq!(addresses.len(), 100);
    }

    // =============================================================================
    // PERFORMANCE AND STRESS TESTS
    // =============================================================================

    #[test]
    fn test_address_validation_performance() {
        let pub_hash_key = b"performance_test_key";
        let address = convert_address(pub_hash_key);
        
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            assert!(validate_address(&address));
        }
        let duration = start.elapsed();
        
        // Should complete 1000 validations in reasonable time (< 1 second)
        assert!(duration.as_millis() < 1000, 
            "Address validation too slow: {}ms for 1000 operations", duration.as_millis());
    }

    #[test]
    fn test_address_creation_performance() {
        let pub_hash_key = b"performance_test_key";
        
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _address = convert_address(pub_hash_key);
        }
        let duration = start.elapsed();
        
        // Should complete 1000 creations in reasonable time (< 1 second)
        assert!(duration.as_millis() < 1000, 
            "Address creation too slow: {}ms for 1000 operations", duration.as_millis());
    }

    // =============================================================================
    // EDGE CASE TESTS
    // =============================================================================

    #[test]
    fn test_checksum_single_byte() {
        let payload = vec![42u8];
        let result = checksum(&payload);
        assert_eq!(result.len(), ADDRESS_CHECK_SUM_LEN);
    }

    #[test]
    fn test_address_with_maximum_size_key() {
        let large_key = vec![123u8; 10000]; // Very large key
        let address = convert_address(&large_key);
        assert!(validate_address(&address));
    }

    #[test] 
    fn test_constants_are_correct() {
        assert_eq!(VERSION, 0x00);
        assert_eq!(ADDRESS_CHECK_SUM_LEN, 4);
    }

    #[test]
    fn test_hash_pub_key_with_real_world_sizes() {
        // Test with typical public key sizes
        let key_33_bytes = vec![1u8; 33]; // Compressed public key
        let key_65_bytes = vec![2u8; 65]; // Uncompressed public key
        
        let hash_33 = hash_pub_key(&key_33_bytes);
        let hash_65 = hash_pub_key(&key_65_bytes);
        
        assert_eq!(hash_33.len(), 20);
        assert_eq!(hash_65.len(), 20);
        assert_ne!(hash_33, hash_65);
    }
} 
