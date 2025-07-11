use rust_blockchain::util::{current_timestamp, sha256_digest, base58_encode, base58_decode, current_dir, ecdsa_p256_sha256_sign_digest, ecdsa_p256_sha256_sign_verify, new_key_pair, ripemd160_digest};

#[test]
fn test_current_timestamp() {
    // Test that current_timestamp returns a reasonable value
    let timestamp = current_timestamp();

    // Should be positive (after Unix epoch)
    assert!(timestamp > 0);

    // Should be within a reasonable range
    // As of 2024, timestamp should be greater than 1704067200 (2024-01-01)
    // and less than 2000000000 (2033-05-18)
    assert!(timestamp > 1704067200);
    assert!(timestamp < 2000000000);

    // Test consistency - two calls should be very close
    let timestamp1 = current_timestamp();
    let timestamp2 = current_timestamp();

    // Should be the same or differ by at most 1 second
    assert!((timestamp1 - timestamp2).abs() <= 1);
}

#[test]
fn test_current_timestamp_monotonic() {
    // Test that timestamps are monotonic (non-decreasing)
    let mut timestamps = Vec::new();
    for _ in 0..5 {
        timestamps.push(current_timestamp());
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    // Each timestamp should be >= the previous one
    for i in 1..timestamps.len() {
        assert!(timestamps[i] >= timestamps[i - 1]);
    }
}

#[test]
fn test_sha256_digest_empty_input() {
    // Test with empty input
    let result = sha256_digest(&[]);

    // SHA256 of empty string is known
    let expected = vec![
        0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
        0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
        0x78, 0x52, 0xb8, 0x55,
    ];

    assert_eq!(result, expected);
    assert_eq!(result.len(), 32); // SHA256 produces 32 bytes
}

#[test]
fn test_sha256_digest_hello_world() {
    // Test with "hello world" - known test vector
    let input = b"hello world";
    let result = sha256_digest(input);

    // Known SHA256 hash of "hello world"
    let expected = vec![
        0xb9, 0x4d, 0x27, 0xb9, 0x93, 0x4d, 0x3e, 0x08, 0xa5, 0x2e, 0x52, 0xd7, 0xda, 0x7d,
        0xab, 0xfa, 0xc4, 0x84, 0xef, 0xe3, 0x7a, 0x53, 0x80, 0xee, 0x90, 0x88, 0xf7, 0xac,
        0xe2, 0xef, 0xcd, 0xe9,
    ];

    assert_eq!(result, expected);
    assert_eq!(result.len(), 32);
}

#[test]
fn test_sha256_digest_abc() {
    // Test with "abc" - another known test vector
    let input = b"abc";
    let result = sha256_digest(input);

    // Known SHA256 hash of "abc"
    let expected = vec![
        0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
        0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
        0xf2, 0x00, 0x15, 0xad,
    ];

    assert_eq!(result, expected);
    assert_eq!(result.len(), 32);
}

#[test]
fn test_sha256_digest_consistency() {
    // Test that same input produces same output
    let input = b"test data for consistency";
    let result1 = sha256_digest(input);
    let result2 = sha256_digest(input);

    assert_eq!(result1, result2);
    assert_eq!(result1.len(), 32);
    assert_eq!(result2.len(), 32);
}

#[test]
fn test_sha256_digest_different_inputs() {
    // Test that different inputs produce different outputs
    let input1 = b"test input 1";
    let input2 = b"test input 2";

    let result1 = sha256_digest(input1);
    let result2 = sha256_digest(input2);

    assert_ne!(result1, result2);
    assert_eq!(result1.len(), 32);
    assert_eq!(result2.len(), 32);
}

#[test]
fn test_sha256_digest_large_input() {
    // Test with large input
    let large_input = vec![0u8; 10000]; // 10KB of zeros
    let result = sha256_digest(&large_input);

    assert_eq!(result.len(), 32);
    // Verify it's different from empty input
    let empty_result = sha256_digest(&[]);
    assert_ne!(result, empty_result);
}

#[test]
fn test_sha256_digest_binary_data() {
    // Test with binary data (not just text)
    let binary_data = vec![0x00, 0xFF, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
    let result = sha256_digest(&binary_data);

    assert_eq!(result.len(), 32);

    // Test that it's different from similar but different data
    let similar_data = vec![0x00, 0xFF, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF1];
    let similar_result = sha256_digest(&similar_data);
    assert_ne!(result, similar_result);
}

#[test]
fn test_sha256_digest_output_format() {
    // Test that output is always 32 bytes regardless of input size
    let inputs = vec![
        vec![],
        vec![0x01],
        vec![0x01, 0x02],
        vec![0x01; 100],
        vec![0xFF; 1000],
    ];

    for input in inputs {
        let result = sha256_digest(&input);
        assert_eq!(result.len(), 32, "SHA256 should always produce 32 bytes");
    }
}

#[test]
fn test_base58_encode_empty_input() {
    // Test encoding empty input
    let input = vec![];
    let result = base58_encode(&input);
    
    // Empty input should produce empty string
    assert_eq!(result, "");
}

#[test]
fn test_base58_encode_single_byte() {
    // Test encoding single byte
    let input = vec![0x00];
    let result = base58_encode(&input);
    
    // Single zero byte should encode to "1"
    assert_eq!(result, "1");
    
    // Test another single byte
    let input = vec![0x01];
    let result = base58_encode(&input);
    assert_eq!(result, "2");
}

#[test]
fn test_base58_encode_known_vectors() {
    // Test with verified Base58 test vectors
    let test_cases = vec![
        (vec![0x00], "1"),
        (vec![0x00, 0x00], "11"),
        (vec![0x00, 0x00, 0x00], "111"),
        (vec![0x00, 0x01], "12"),
        (vec![0x00, 0x3c, 0x17, 0x6e], "1MBgH"),  // Fixed: was "16Ho"
        (vec![0x51, 0x6b, 0x6f, 0xcd, 0x0f], "ABnLTmg"),
        (b"hello world".to_vec(), "StV1DL6CwTryKyV"),
    ];

    for (input, expected) in test_cases {
        let result = base58_encode(&input);
        assert_eq!(result, expected, "Failed for input: {input:?}");
    }
}

#[test]
fn test_base58_decode_empty_input() {
    // Test decoding empty input
    let input = "";
    let result = base58_decode(input);
    
    // Empty input should produce empty vector
    assert_eq!(result, vec![]);
}

#[test]
fn test_base58_decode_single_character() {
    // Test decoding single character
    let input = "1";
    let result = base58_decode(input);
    
    // "1" should decode to single zero byte
    assert_eq!(result, vec![0x00]);
    
    // Test another single character
    let input = "2";
    let result = base58_decode(input);
    assert_eq!(result, vec![0x01]);
}

#[test]
fn test_base58_decode_known_vectors() {
    // Test with verified Base58 test vectors (reverse of encode tests)
    let test_cases = vec![
        ("1", vec![0x00]),
        ("11", vec![0x00, 0x00]),
        ("111", vec![0x00, 0x00, 0x00]),
        ("12", vec![0x00, 0x01]),
        ("1MBgH", vec![0x00, 0x3c, 0x17, 0x6e]),
        ("ABnLTmg", vec![0x51, 0x6b, 0x6f, 0xcd, 0x0f]),
        ("StV1DL6CwTryKyV", b"hello world".to_vec()),
    ];

    for (input, expected) in test_cases {
        let result = base58_decode(input);
        assert_eq!(result, expected, "Failed for input: {input}");
    }
}

#[test]
fn test_base58_encode_decode_round_trip() {
    // Test that encode->decode returns original data
    let test_data = vec![
        vec![],
        vec![0x00],
        vec![0xFF],
        vec![0x00, 0x01, 0x02, 0x03],
        vec![0xFF, 0xFE, 0xFD, 0xFC],
        b"Hello, World!".to_vec(),
        vec![0x00; 100], // Lots of leading zeros
        (0..255u8).collect(), // All possible byte values
    ];

    for original in test_data {
        let encoded = base58_encode(&original);
        let decoded = base58_decode(&encoded);
        assert_eq!(decoded, original, "Round trip failed for: {original:?}");
    }
}

#[test]
fn test_base58_encode_different_inputs() {
    // Test that different inputs produce different outputs
    let input1 = vec![0x01, 0x02, 0x03];
    let input2 = vec![0x01, 0x02, 0x04];

    let result1 = base58_encode(&input1);
    let result2 = base58_encode(&input2);

    assert_ne!(result1, result2);
}

#[test]
fn test_base58_encode_consistency() {
    // Test that same input produces same output
    let input = vec![0x12, 0x34, 0x56, 0x78];
    let result1 = base58_encode(&input);
    let result2 = base58_encode(&input);

    assert_eq!(result1, result2);
}

#[test]
fn test_base58_decode_consistency() {
    // Test that same input produces same output
    let input = "123456789";
    let result1 = base58_decode(input);
    let result2 = base58_decode(input);

    assert_eq!(result1, result2);
}

#[test]
fn test_base58_encode_large_input() {
    // Test with large input
    let large_input = vec![0x42u8; 1000]; // 1KB of data
    let result = base58_encode(&large_input);

    assert!(!result.is_empty());
    
    // Test round trip
    let decoded = base58_decode(&result);
    assert_eq!(decoded, large_input);
}

#[test]
fn test_base58_decode_invalid_character() {
    // Test decoding with invalid characters
    let input = "0OIl"; // Contains invalid characters
    let result = base58_decode(input);
    
    // Should return empty vector for invalid input
    assert_eq!(result, vec![]);
}

#[test]
fn test_base58_decode_invalid_character_o() {
    let result = base58_decode("O"); // uppercase 'O' is invalid
    assert_eq!(result, vec![]);
}

#[test]
fn test_base58_decode_invalid_character_i() {
    let result = base58_decode("I"); // uppercase 'I' is invalid
    assert_eq!(result, vec![]);
}

#[test]
fn test_base58_decode_invalid_character_l() {
    let result = base58_decode("l"); // lowercase 'l' is invalid
    assert_eq!(result, vec![]);
}

#[test]
fn test_current_dir_basic() {
    // Test that current_dir returns a valid path
    let dir = current_dir();
    
    // Should be an absolute path
    assert!(dir.is_absolute());
    
    // Should exist (assuming we're running in a valid directory)
    assert!(dir.exists());
    
    // Should be a directory
    assert!(dir.is_dir());
}

#[test]
fn test_current_dir_consistency() {
    // Test that multiple calls return the same result
    let dir1 = current_dir();
    let dir2 = current_dir();
    
    assert_eq!(dir1, dir2);
}

#[test]
fn test_current_dir_workspace_detection() {
    // Test that current_dir can find the workspace directory
    let dir = current_dir();
    
    // Should contain Cargo.toml (since this is a Rust project)
    let cargo_toml = dir.join("Cargo.toml");
    assert!(cargo_toml.exists(), "Cargo.toml should exist in project root");
}

#[test]
fn test_current_dir_path_components() {
    // Test that the returned path has reasonable components
    let dir = current_dir();
    
    // Should have at least one component
    assert!(dir.components().count() > 0);
    
    // Should be convertible to string
    let _dir_str = dir.to_string_lossy();
}

#[test]
fn test_sha256_digest_unicode_text() {
    // Test with Unicode text
    let unicode_text = "Hello, 世界! 🌍";
    let result = sha256_digest(unicode_text.as_bytes());
    
    assert_eq!(result.len(), 32);
    
    // Test consistency
    let result2 = sha256_digest(unicode_text.as_bytes());
    assert_eq!(result, result2);
}

#[test]
fn test_sha256_digest_edge_case_sizes() {
    // Test with various edge case sizes
    let sizes = vec![1, 55, 56, 64, 127, 128, 255, 256, 511, 512, 1023, 1024];
    
    for size in sizes {
        let input = vec![0x42u8; size];
        let result = sha256_digest(&input);
        assert_eq!(result.len(), 32, "Failed for input size: {size}");
    }
}

#[test]
fn test_sha256_digest_incremental_changes() {
    // Test that small changes produce very different hashes
    let base_input = b"test string for hash sensitivity";
    let base_hash = sha256_digest(base_input);
    
    for i in 0..base_input.len() {
        let mut modified_input = base_input.to_vec();
        modified_input[i] = modified_input[i].wrapping_add(1);
        
        let modified_hash = sha256_digest(&modified_input);
        assert_ne!(base_hash, modified_hash, "Hash should change for position {i}");
    }
}

#[test]
fn test_base58_encode_leading_zeros() {
    // Test encoding with leading zeros
    let input = vec![0x00, 0x00, 0x00, 0x01, 0x02, 0x03];
    let result = base58_encode(&input);
    
    // Should start with multiple '1's for leading zeros
    assert!(result.starts_with("111"));
    
    // Test round trip
    let decoded = base58_decode(&result);
    assert_eq!(decoded, input);
}

#[test]
fn test_base58_encode_boundary_values() {
    // Test encoding boundary values
    let test_cases = vec![
        vec![0x00],
        vec![0xFF],
        vec![0x00, 0xFF],
        vec![0xFF, 0x00],
        vec![0x7F, 0xFF, 0xFF, 0xFF],
        vec![0x80, 0x00, 0x00, 0x00],
    ];
    
    for input in test_cases {
        let encoded = base58_encode(&input);
        let decoded = base58_decode(&encoded);
        assert_eq!(decoded, input, "Boundary test failed for: {input:?}");
    }
}

#[test]
fn test_base58_character_set() {
    // Test that Base58 uses the correct character set
    // Base58 alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
    // (excludes 0, O, I, l)
    
    let input = vec![0x01; 20]; // Some arbitrary data
    let encoded = base58_encode(&input);
    
    // Should only contain valid Base58 characters
    for ch in encoded.chars() {
        assert!(
            ch.is_ascii_alphanumeric() && ch != '0' && ch != 'O' && ch != 'I' && ch != 'l',
            "Invalid Base58 character: {ch}"
        );
    }
}

#[test]
fn test_current_timestamp_precision() {
    // Test timestamp precision (should be in seconds)
    let timestamp = current_timestamp();
    
    // Convert to milliseconds and back - should be different
    let timestamp_ms = timestamp * 1000;
    assert!(timestamp_ms > timestamp);
    
    // But the timestamp itself should be reasonable for seconds since epoch
    let current_year_timestamp = 1640995200; // Roughly 2022-01-01 in seconds
    assert!(timestamp > current_year_timestamp);
}

#[test]
fn test_utility_functions_integration() {
    // Test integration of utility functions
    let data = "integration test data";
    
    // Hash the data
    let hash = sha256_digest(data.as_bytes());
    
    // Encode the hash
    let encoded = base58_encode(&hash);
    
    // Decode back
    let decoded = base58_decode(&encoded);
    
    // Should get back original hash
    assert_eq!(decoded, hash);
    
    // Timestamp should be consistent throughout
    let timestamp1 = current_timestamp();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let timestamp2 = current_timestamp();
    
    assert!(timestamp2 >= timestamp1);
    assert!(timestamp2 - timestamp1 <= 1); // Should be within 1 second
}

#[test]
fn test_ecdsa_p256_sha256_sign_digest_basic() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
    use ring::rand::SystemRandom;

    // Generate a test key pair
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let pkcs8 = pkcs8_bytes.as_ref();
    
    // Test message
    let message = b"test message for signing";
    
    // Sign the message
    let signature = ecdsa_p256_sha256_sign_digest(pkcs8, message);
    
    // Signature should not be empty
    assert!(!signature.is_empty());
    
    // Signature should be approximately 64 bytes for P-256 (may vary slightly)
    assert!(signature.len() >= 60 && signature.len() <= 72);
}

#[test]
fn test_ecdsa_p256_sha256_sign_digest_consistency() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
    use ring::rand::SystemRandom;

    // Generate a test key pair
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let pkcs8 = pkcs8_bytes.as_ref();
    
    // Test message
    let message = b"consistency test message";
    
    // Sign the same message multiple times - signatures may differ due to randomness
    let signature1 = ecdsa_p256_sha256_sign_digest(pkcs8, message);
    let signature2 = ecdsa_p256_sha256_sign_digest(pkcs8, message);
    
    // Both signatures should be valid (not empty)
    assert!(!signature1.is_empty());
    assert!(!signature2.is_empty());
    
    // Signatures may be different due to randomness in ECDSA
    // But both should be valid length
    assert!(signature1.len() >= 60 && signature1.len() <= 72);
    assert!(signature2.len() >= 60 && signature2.len() <= 72);
}

#[test]
fn test_ecdsa_p256_sha256_sign_digest_different_messages() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
    use ring::rand::SystemRandom;

    // Generate a test key pair
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let pkcs8 = pkcs8_bytes.as_ref();
    
    // Different messages
    let message1 = b"first test message";
    let message2 = b"second test message";
    
    // Sign different messages
    let signature1 = ecdsa_p256_sha256_sign_digest(pkcs8, message1);
    let signature2 = ecdsa_p256_sha256_sign_digest(pkcs8, message2);
    
    // Signatures should be different for different messages
    assert_ne!(signature1, signature2);
    assert!(!signature1.is_empty());
    assert!(!signature2.is_empty());
}

#[test]
fn test_ecdsa_p256_sha256_sign_verify_valid_signature() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
    use ring::rand::SystemRandom;

    // Generate a test key pair
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes.as_ref(), &rng).unwrap();
    
    // Get public key
    let public_key = key_pair.public_key().as_ref();
    
    // Test message
    let message = b"message to sign and verify";
    
    // Sign the message
    let signature = ecdsa_p256_sha256_sign_digest(pkcs8_bytes.as_ref(), message);
    
    // Verify the signature
    let is_valid = ecdsa_p256_sha256_sign_verify(public_key, &signature, message);
    
    // Signature should be valid
    assert!(is_valid);
}

#[test]
fn test_ecdsa_p256_sha256_sign_verify_invalid_signature() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
    use ring::rand::SystemRandom;

    // Generate a test key pair
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes.as_ref(), &rng).unwrap();
    
    // Get public key
    let public_key = key_pair.public_key().as_ref();
    
    // Test message
    let message = b"original message";
    
    // Sign the message
    let mut signature = ecdsa_p256_sha256_sign_digest(pkcs8_bytes.as_ref(), message);
    
    // Corrupt the signature by changing one byte
    if !signature.is_empty() {
        signature[0] = signature[0].wrapping_add(1);
    }
    
    // Verify the corrupted signature
    let is_valid = ecdsa_p256_sha256_sign_verify(public_key, &signature, message);
    
    // Corrupted signature should be invalid
    assert!(!is_valid);
}

#[test]
fn test_ecdsa_p256_sha256_sign_verify_wrong_message() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
    use ring::rand::SystemRandom;

    // Generate a test key pair
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes.as_ref(), &rng).unwrap();
    
    // Get public key
    let public_key = key_pair.public_key().as_ref();
    
    // Original message and different message
    let original_message = b"original message";
    let different_message = b"different message";
    
    // Sign the original message
    let signature = ecdsa_p256_sha256_sign_digest(pkcs8_bytes.as_ref(), original_message);
    
    // Try to verify with different message
    let is_valid = ecdsa_p256_sha256_sign_verify(public_key, &signature, different_message);
    
    // Should be invalid with wrong message
    assert!(!is_valid);
}

#[test]
fn test_ecdsa_p256_sha256_sign_verify_wrong_public_key() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
    use ring::rand::SystemRandom;

    let rng = SystemRandom::new();
    
    // Generate first key pair
    let pkcs8_bytes1 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    
    // Generate second key pair
    let pkcs8_bytes2 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let key_pair2 = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes2.as_ref(), &rng).unwrap();
    
    // Get public key from second pair
    let public_key2 = key_pair2.public_key().as_ref();
    
    // Test message
    let message = b"test message";
    
    // Sign with first key pair
    let signature = ecdsa_p256_sha256_sign_digest(pkcs8_bytes1.as_ref(), message);
    
    // Try to verify with second key pair's public key
    let is_valid = ecdsa_p256_sha256_sign_verify(public_key2, &signature, message);
    
    // Should be invalid with wrong public key
    assert!(!is_valid);
}

#[test]
fn test_ecdsa_p256_sha256_sign_verify_empty_message() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
    use ring::rand::SystemRandom;

    // Generate a test key pair
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes.as_ref(), &rng).unwrap();
    
    // Get public key
    let public_key = key_pair.public_key().as_ref();
    
    // Empty message
    let message = b"";
    
    // Sign the empty message
    let signature = ecdsa_p256_sha256_sign_digest(pkcs8_bytes.as_ref(), message);
    
    // Verify the signature
    let is_valid = ecdsa_p256_sha256_sign_verify(public_key, &signature, message);
    
    // Should be valid even for empty message
    assert!(is_valid);
}

#[test]
fn test_ecdsa_p256_sha256_sign_verify_large_message() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
    use ring::rand::SystemRandom;

    // Generate a test key pair
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes.as_ref(), &rng).unwrap();
    
    // Get public key
    let public_key = key_pair.public_key().as_ref();
    
    // Large message (10KB)
    let large_message = vec![0x42u8; 10000];
    
    // Sign the large message
    let signature = ecdsa_p256_sha256_sign_digest(pkcs8_bytes.as_ref(), &large_message);
    
    // Verify the signature
    let is_valid = ecdsa_p256_sha256_sign_verify(public_key, &signature, &large_message);
    
    // Should be valid for large message
    assert!(is_valid);
}

#[test]
fn test_ecdsa_p256_sha256_sign_verify_empty_signature() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
    use ring::rand::SystemRandom;

    // Generate a test key pair
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes.as_ref(), &rng).unwrap();
    
    // Get public key
    let public_key = key_pair.public_key().as_ref();
    
    // Test message
    let message = b"test message";
    
    // Empty signature
    let empty_signature = vec![];
    
    // Try to verify empty signature
    let is_valid = ecdsa_p256_sha256_sign_verify(public_key, &empty_signature, message);
    
    // Empty signature should be invalid
    assert!(!is_valid);
}

#[test]
fn test_ecdsa_p256_sha256_sign_verify_binary_data() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
    use ring::rand::SystemRandom;

    // Generate a test key pair
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes.as_ref(), &rng).unwrap();
    
    // Get public key
    let public_key = key_pair.public_key().as_ref();
    
    // Binary data message
    let binary_message = vec![0x00, 0xFF, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
    
    // Sign the binary message
    let signature = ecdsa_p256_sha256_sign_digest(pkcs8_bytes.as_ref(), &binary_message);
    
    // Verify the signature
    let is_valid = ecdsa_p256_sha256_sign_verify(public_key, &signature, &binary_message);
    
    // Should be valid for binary data
    assert!(is_valid);
}

#[test]
fn test_ecdsa_p256_sha256_integration_multiple_signatures() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
    use ring::rand::SystemRandom;

    // Generate a test key pair
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes.as_ref(), &rng).unwrap();
    
    // Get public key
    let public_key = key_pair.public_key().as_ref();
    
    // Test multiple messages
    let messages = vec![
        b"first message".to_vec(),
        b"second message".to_vec(),
        b"third message".to_vec(),
        vec![0x01, 0x02, 0x03, 0x04], // binary data
        vec![], // empty message
    ];
    
    // Sign and verify each message
    for message in &messages {
        let signature = ecdsa_p256_sha256_sign_digest(pkcs8_bytes.as_ref(), message);
        let is_valid = ecdsa_p256_sha256_sign_verify(public_key, &signature, message);
        
        assert!(is_valid, "Failed for message: {message:?}");
        assert!(!signature.is_empty(), "Signature should not be empty for message: {message:?}");
    }
}

#[test]
fn test_ecdsa_p256_sha256_cross_verification() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
    use ring::rand::SystemRandom;

    let rng = SystemRandom::new();
    
    // Generate multiple key pairs
    let key_pairs: Vec<_> = (0..3).map(|_| {
        let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
        let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes.as_ref(), &rng).unwrap();
        (pkcs8_bytes, key_pair)
    }).collect();
    
    let message = b"cross verification test";
    
    // Each key pair should verify its own signatures but not others'
    for i in 0..key_pairs.len() {
        let (ref pkcs8_i, ref key_pair_i) = key_pairs[i];
        let _public_key_i = key_pair_i.public_key().as_ref();
        let signature_i = ecdsa_p256_sha256_sign_digest(pkcs8_i.as_ref(), message);
        
        for (j, (_, key_pair_j)) in key_pairs.iter().enumerate() {
            let public_key_j = key_pair_j.public_key().as_ref();
            
            let is_valid = ecdsa_p256_sha256_sign_verify(public_key_j, &signature_i, message);
            
            if i == j {
                // Should verify its own signature
                assert!(is_valid, "Key pair {i} should verify its own signature");
            } else {
                // Should not verify other signatures
                assert!(!is_valid, "Key pair {j} should not verify signature from key pair {i}");
            }
        }
    }
}

// Tests for new_key_pair function

#[test]
fn test_new_key_pair_basic() {
    // Test that new_key_pair generates a valid key pair
    let key_pair_bytes = new_key_pair();
    
    // Should not be empty
    assert!(!key_pair_bytes.is_empty());
    
    // Should be a reasonable size (PKCS8 encoded ECDSA key pair)
    // P-256 PKCS8 private keys are typically around 138 bytes
    assert!(key_pair_bytes.len() > 100);
    assert!(key_pair_bytes.len() < 200);
}

#[test]
fn test_new_key_pair_consistency() {
    // Test that each call generates a new, unique key pair
    let key_pair1 = new_key_pair();
    let key_pair2 = new_key_pair();
    
    // Should be different each time
    assert_ne!(key_pair1, key_pair2);
    
    // Both should be valid length
    assert!(!key_pair1.is_empty());
    assert!(!key_pair2.is_empty());
}

#[test]
fn test_new_key_pair_multiple_generation() {
    // Test generating multiple key pairs
    let mut key_pairs = Vec::new();
    
    for _ in 0..5 {
        let key_pair = new_key_pair();
        assert!(!key_pair.is_empty());
        
        // Ensure this key pair is unique
        for existing_key_pair in &key_pairs {
            assert_ne!(key_pair, *existing_key_pair);
        }
        
        key_pairs.push(key_pair);
    }
    
    // All should be different
    assert_eq!(key_pairs.len(), 5);
}

#[test]
fn test_new_key_pair_with_crypto_operations() {
    use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
    use ring::rand::SystemRandom;

    // Generate a key pair using new_key_pair
    let pkcs8_bytes = new_key_pair();
    
    // Test that it can be used with ring's ECDSA operations
    let rng = SystemRandom::new();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &pkcs8_bytes, &rng);
    
    // Should be able to create a valid key pair
    assert!(key_pair.is_ok());
    
    let key_pair = key_pair.unwrap();
    let public_key = key_pair.public_key().as_ref();
    
    // Public key should not be empty
    assert!(!public_key.is_empty());
    
    // Test that we can sign with the generated key
    let message = b"test message";
    let signature = ecdsa_p256_sha256_sign_digest(&pkcs8_bytes, message);
    
    // Should produce a valid signature
    assert!(!signature.is_empty());
    
    // Should be able to verify the signature
    let is_valid = ecdsa_p256_sha256_sign_verify(public_key, &signature, message);
    assert!(is_valid);
}

#[test]
fn test_new_key_pair_integration_with_existing_functions() {
    // Test integration with existing cryptographic functions
    let message = b"integration test message";
    
    // Generate multiple key pairs and test each one
    for i in 0..3 {
        let pkcs8_bytes = new_key_pair();
        
        // Test signing
        let signature = ecdsa_p256_sha256_sign_digest(&pkcs8_bytes, message);
        assert!(!signature.is_empty(), "Signature should not be empty for iteration {i}");
        
        // Extract public key and test verification
        use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
        use ring::rand::SystemRandom;
        
        let rng = SystemRandom::new();
        let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &pkcs8_bytes, &rng).unwrap();
        let public_key = key_pair.public_key().as_ref();
        
        let is_valid = ecdsa_p256_sha256_sign_verify(public_key, &signature, message);
        assert!(is_valid, "Signature should be valid for iteration {i}");
    }
}

// Tests for ripemd160_digest function

#[test]
fn test_ripemd160_digest_empty_input() {
    // Test with empty input
    let result = ripemd160_digest(&[]);
    
    // RIPEMD160 of empty string is known
    let expected = vec![
        0x9c, 0x11, 0x85, 0xa5, 0xc5, 0xe9, 0xfc, 0x54, 0x61, 0x28,
        0x08, 0x97, 0x7e, 0xe8, 0xf5, 0x48, 0xb2, 0x25, 0x8d, 0x31
    ];
    
    assert_eq!(result, expected);
    assert_eq!(result.len(), 20); // RIPEMD160 produces 20 bytes
}

#[test]
fn test_ripemd160_digest_abc() {
    // Test with "abc" - known test vector
    let input = b"abc";
    let result = ripemd160_digest(input);
    
    // Known RIPEMD160 hash of "abc"
    let expected = vec![
        0x8e, 0xb2, 0x08, 0xf7, 0xe0, 0x5d, 0x98, 0x7a, 0x9b, 0x04,
        0x4a, 0x8e, 0x98, 0xc6, 0xb0, 0x87, 0xf1, 0x5a, 0x0b, 0xfc
    ];
    
    assert_eq!(result, expected);
    assert_eq!(result.len(), 20);
}

#[test]
fn test_ripemd160_digest_hello_world() {
    // Test with "hello world"
    let input = b"hello world";
    let result = ripemd160_digest(input);
    
    // Known RIPEMD160 hash of "hello world"
    let expected = vec![
        0x98, 0xc6, 0x15, 0x78, 0x4c, 0xcb, 0x5f, 0xe5, 0x93, 0x6f,
        0xbc, 0x0c, 0xbe, 0x9d, 0xfd, 0xb4, 0x08, 0xd9, 0x2f, 0x0f
    ];
    
    assert_eq!(result, expected);
    assert_eq!(result.len(), 20);
}

#[test]
fn test_ripemd160_digest_consistency() {
    // Test that same input produces same output
    let input = b"test data for consistency";
    let result1 = ripemd160_digest(input);
    let result2 = ripemd160_digest(input);
    
    assert_eq!(result1, result2);
    assert_eq!(result1.len(), 20);
    assert_eq!(result2.len(), 20);
}

#[test]
fn test_ripemd160_digest_different_inputs() {
    // Test that different inputs produce different outputs
    let input1 = b"test input 1";
    let input2 = b"test input 2";
    
    let result1 = ripemd160_digest(input1);
    let result2 = ripemd160_digest(input2);
    
    assert_ne!(result1, result2);
    assert_eq!(result1.len(), 20);
    assert_eq!(result2.len(), 20);
}

#[test]
fn test_ripemd160_digest_large_input() {
    // Test with large input
    let large_input = vec![0u8; 10000]; // 10KB of zeros
    let result = ripemd160_digest(&large_input);
    
    assert_eq!(result.len(), 20);
    // Verify it's different from empty input
    let empty_result = ripemd160_digest(&[]);
    assert_ne!(result, empty_result);
}

#[test]
fn test_ripemd160_digest_binary_data() {
    // Test with binary data (not just text)
    let binary_data = vec![0x00, 0xFF, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
    let result = ripemd160_digest(&binary_data);
    
    assert_eq!(result.len(), 20);
    
    // Test that it's different from similar but different data
    let similar_data = vec![0x00, 0xFF, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF1];
    let similar_result = ripemd160_digest(&similar_data);
    assert_ne!(result, similar_result);
}

#[test]
fn test_ripemd160_digest_output_format() {
    // Test that output is always 20 bytes regardless of input size
    let inputs = vec![
        vec![],
        vec![0x01],
        vec![0x01, 0x02],
        vec![0x01; 100],
        vec![0xFF; 1000],
    ];
    
    for input in inputs {
        let result = ripemd160_digest(&input);
        assert_eq!(result.len(), 20, "RIPEMD160 should always produce 20 bytes");
    }
}

#[test]
fn test_ripemd160_digest_unicode_text() {
    // Test with Unicode text
    let unicode_text = "Hello, 世界! 🚀";
    let input = unicode_text.as_bytes();
    let result = ripemd160_digest(input);
    
    assert_eq!(result.len(), 20);
    
    // Should be different from ASCII version
    let ascii_result = ripemd160_digest(b"Hello, World!");
    assert_ne!(result, ascii_result);
}

#[test]
fn test_ripemd160_digest_vs_sha256() {
    // Test that RIPEMD160 produces different results than SHA256 for same input
    let input = b"compare hash functions";
    
    let ripemd160_result = ripemd160_digest(input);
    let sha256_result = sha256_digest(input);
    
    // Different hash functions should produce different results
    assert_ne!(ripemd160_result.len(), sha256_result.len());
    assert_eq!(ripemd160_result.len(), 20);
    assert_eq!(sha256_result.len(), 32);
    
    // Results should be different (they're different lengths anyway, but good to verify)
    assert_ne!(ripemd160_result, sha256_result[..20]);
}

#[test]
fn test_ripemd160_digest_incremental_changes() {
    // Test that small changes in input produce very different outputs
    let base_input = b"test message";
    let modified_input = b"test messagi"; // changed last character
    
    let base_result = ripemd160_digest(base_input);
    let modified_result = ripemd160_digest(modified_input);
    
    assert_ne!(base_result, modified_result);
    
    // Count how many bytes are different (should be many due to avalanche effect)
    let different_bytes = base_result.iter()
        .zip(modified_result.iter())
        .filter(|(a, b)| a != b)
        .count();
    
    // At least half the bytes should be different (avalanche effect)
    assert!(different_bytes >= 10, "Expected avalanche effect, only {different_bytes} bytes different");
}
