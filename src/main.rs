mod block;
mod models;
mod proof_of_work;

use sha2::{Sha256, Digest};

pub fn current_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
            assert!(timestamps[i] >= timestamps[i-1]);
        }
    }
    
    #[test]
    fn test_sha256_digest_empty_input() {
        // Test with empty input
        let result = sha256_digest(&[]);
        
        // SHA256 of empty string is known
        let expected = vec![
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14,
            0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24,
            0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c,
            0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55
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
            0xb9, 0x4d, 0x27, 0xb9, 0x93, 0x4d, 0x3e, 0x08,
            0xa5, 0x2e, 0x52, 0xd7, 0xda, 0x7d, 0xab, 0xfa,
            0xc4, 0x84, 0xef, 0xe3, 0x7a, 0x53, 0x80, 0xee,
            0x90, 0x88, 0xf7, 0xac, 0xe2, 0xef, 0xcd, 0xe9
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
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea,
            0x41, 0x41, 0x40, 0xde, 0x5d, 0xae, 0x22, 0x23,
            0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c,
            0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00, 0x15, 0xad
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
}
