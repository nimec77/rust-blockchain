use crate::transaction::tx_output::TXOutput;

impl TXOutput {
   
    // pub fn new(value: i32, address: &str) -> TXOutput {
    //     let mut output = TXOutput {
    //         value,
    //         pub_key_hash: vec![],
    //     };
    //     output.lock(address);
    //     return output;
    // }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_pub_key_hash(&self) -> &[u8] {
        self.pub_key_hash.as_slice()
    }

    // fn lock(&mut self, address: &str) {
    //     let payload = util::base58_decode(address);
    //     let pub_key_hash = payload[1..payload.len() - wallet::ADDRESS_CHECK_SUM_LEN].to_vec();
    //     self.pub_key_hash = pub_key_hash;
    // }

    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash.eq(pub_key_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_output() -> TXOutput {
        TXOutput {
            value: 100,
            pub_key_hash: vec![1, 2, 3, 4, 5],
        }
    }

    fn create_output_with_value(value: i32) -> TXOutput {
        TXOutput {
            value,
            pub_key_hash: vec![10, 20, 30],
        }
    }

    fn create_output_with_key_hash(key_hash: Vec<u8>) -> TXOutput {
        TXOutput {
            value: 50,
            pub_key_hash: key_hash,
        }
    }

    #[test]
    fn test_get_value_positive() {
        let output = create_output_with_value(150);
        assert_eq!(output.get_value(), 150);
    }

    #[test]
    fn test_get_value_zero() {
        let output = create_output_with_value(0);
        assert_eq!(output.get_value(), 0);
    }

    #[test]
    fn test_get_value_negative() {
        let output = create_output_with_value(-25);
        assert_eq!(output.get_value(), -25);
    }

    #[test]
    fn test_get_pub_key_hash_normal() {
        let key_hash = vec![10, 20, 30, 40, 50];
        let output = create_output_with_key_hash(key_hash.clone());
        assert_eq!(output.get_pub_key_hash(), key_hash.as_slice());
    }

    #[test]
    fn test_get_pub_key_hash_empty() {
        let output = create_output_with_key_hash(vec![]);
        assert_eq!(output.get_pub_key_hash(), &[]);
    }

    #[test]
    fn test_get_pub_key_hash_single_byte() {
        let key_hash = vec![255];
        let output = create_output_with_key_hash(key_hash.clone());
        assert_eq!(output.get_pub_key_hash(), key_hash.as_slice());
    }

    #[test]
    fn test_is_locked_with_key_matching() {
        let key_hash = vec![1, 2, 3, 4, 5];
        let output = create_output_with_key_hash(key_hash.clone());
        assert!(output.is_locked_with_key(&key_hash));
    }

    #[test]
    fn test_is_locked_with_key_not_matching() {
        let key_hash = vec![1, 2, 3, 4, 5];
        let different_key = vec![5, 4, 3, 2, 1];
        let output = create_output_with_key_hash(key_hash);
        assert!(!output.is_locked_with_key(&different_key));
    }

    #[test]
    fn test_is_locked_with_key_different_length() {
        let key_hash = vec![1, 2, 3];
        let longer_key = vec![1, 2, 3, 4, 5];
        let output = create_output_with_key_hash(key_hash);
        assert!(!output.is_locked_with_key(&longer_key));
    }

    #[test]
    fn test_is_locked_with_key_empty_keys() {
        let output = create_output_with_key_hash(vec![]);
        assert!(output.is_locked_with_key(&[]));
    }

    #[test]
    fn test_is_locked_with_key_empty_stored_key() {
        let output = create_output_with_key_hash(vec![]);
        let test_key = vec![1, 2, 3];
        assert!(!output.is_locked_with_key(&test_key));
    }

    #[test]
    fn test_struct_creation_and_fields() {
        let output = TXOutput {
            value: 42,
            pub_key_hash: vec![0xAB, 0xCD, 0xEF],
        };
        
        assert_eq!(output.value, 42);
        assert_eq!(output.pub_key_hash, vec![0xAB, 0xCD, 0xEF]);
    }

    #[test]
    fn test_clone_functionality() {
        let original = create_sample_output();
        let cloned = original.clone();
        
        assert_eq!(original.get_value(), cloned.get_value());
        assert_eq!(original.get_pub_key_hash(), cloned.get_pub_key_hash());
        assert!(cloned.is_locked_with_key(original.get_pub_key_hash()));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let original = create_sample_output();
        
        // Serialize
        let encoded = bincode::encode_to_vec(&original, bincode::config::standard())
            .expect("Failed to serialize TXOutput");
        
        // Deserialize
        let decoded: TXOutput = bincode::decode_from_slice(&encoded, bincode::config::standard())
            .expect("Failed to deserialize TXOutput").0;
        
        // Verify roundtrip integrity
        assert_eq!(original.get_value(), decoded.get_value());
        assert_eq!(original.get_pub_key_hash(), decoded.get_pub_key_hash());
        assert!(decoded.is_locked_with_key(original.get_pub_key_hash()));
    }

    #[test]
    fn test_large_values() {
        let output = create_output_with_value(i32::MAX);
        assert_eq!(output.get_value(), i32::MAX);
        
        let output = create_output_with_value(i32::MIN);
        assert_eq!(output.get_value(), i32::MIN);
    }

    #[test]
    fn test_large_key_hash() {
        let large_key = vec![42u8; 1000]; // 1000 bytes of the same value
        let output = create_output_with_key_hash(large_key.clone());
        assert_eq!(output.get_pub_key_hash(), large_key.as_slice());
        assert!(output.is_locked_with_key(&large_key));
    }
}
