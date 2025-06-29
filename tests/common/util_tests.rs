use rust_blockchain::util::{current_timestamp, sha256_digest, base58_encode, base58_decode, current_dir};

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
