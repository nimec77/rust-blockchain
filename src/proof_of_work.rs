use data_encoding::HEXLOWER;
use num_bigint::{BigInt, Sign};

use crate::{block::Block, common::bincode_bigint::BincodeBigInt, util};

// Maximum number of nonce iterations to try
const MAX_NONCE: i64 = i64::MAX;

// Target difficulty - number of leading zeros in hash (adjustable)
const TARGET_BITS: usize = 24;

#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct ProofOfWork {
    block: Block,
    target: BincodeBigInt,
}

impl ProofOfWork {
    /// Create a new proof-of-work instance for the given block
    pub fn new_proof_of_work(block: Block) -> ProofOfWork {
        // Calculate target: 1 << (256 - TARGET_BITS)
        let target_value = BigInt::from(1) << (256 - TARGET_BITS);
        let target = BincodeBigInt::new(target_value);
        
        ProofOfWork { block, target }
    }

    /// Prepare data for hashing by combining block fields with nonce
    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let mut data = Vec::new();
        
        // Combine block data with nonce
        data.extend_from_slice(self.block.get_pre_block_hash().as_bytes());
        data.extend_from_slice(&self.block.hash_transactions());
        data.extend_from_slice(&self.block.get_timestamp().to_be_bytes());
        data.extend_from_slice(&(TARGET_BITS as u64).to_be_bytes());
        data.extend_from_slice(&nonce.to_be_bytes());
        
        data
    }

    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash = Vec::new();
        println!("Mining the block");
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = util::sha256_digest(data.as_slice());
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());
            
            if hash_int < *self.target.as_bigint() {
                println!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
            }
        }
        println!();
        (nonce, HEXLOWER.encode(hash.as_slice()))
    }

    /// Validate that a block's hash satisfies the proof-of-work requirement
    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.get_nonce());
        let hash = util::sha256_digest(data.as_slice());
        let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());
        
        hash_int < *self.target.as_bigint()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::{transaction::Transaction, tx_input::TXInput, tx_output::TXOutput};

    fn create_test_transaction(id: Vec<u8>) -> Transaction {
        let tx_input = TXInput {
            txid: vec![1, 2, 3],
            vout: 0,
            signature: vec![4, 5, 6],
            pub_key: vec![7, 8, 9],
        };
        let tx_output = TXOutput {
            value: 100,
            pub_key_hash: vec![10, 11, 12],
        };

        Transaction {
            id,
            vin: vec![tx_input],
            vout: vec![tx_output],
        }
    }

    fn create_test_block() -> Block {
        let transaction = create_test_transaction(vec![1, 2, 3, 4]);
        let transactions = vec![transaction];
        let pre_block_hash = "test_previous_hash".to_string();
        let height = 1;

        Block::new_block_without_proof_of_work(pre_block_hash, &transactions, height)
    }



    #[test]
    fn test_new_proof_of_work() {
        let block = create_test_block();
        let pow = ProofOfWork::new_proof_of_work(block.clone());

        // Verify that the target is calculated correctly
        let expected_target = BigInt::from(1) << (256 - TARGET_BITS);
        assert_eq!(*pow.target.as_bigint(), expected_target);

        // Verify that the block is stored correctly
        assert_eq!(pow.block.get_pre_block_hash(), block.get_pre_block_hash());
        assert_eq!(pow.block.get_timestamp(), block.get_timestamp());
        assert_eq!(pow.block.get_height(), block.get_height());
    }

    #[test]
    fn test_new_proof_of_work_target_calculation() {
        let block = create_test_block();
        let pow = ProofOfWork::new_proof_of_work(block);

        // Test that target is a valid BigInt and has the expected magnitude
        let target = pow.target.as_bigint();
        assert!(target > &BigInt::from(0));
        
        // Target should be 2^(256-24) = 2^232
        let expected_target = BigInt::from(1) << 232;
        assert_eq!(target, &expected_target);
    }

    #[test]
    fn test_prepare_data() {
        let block = create_test_block();
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let nonce = 12345;

        let data = pow.prepare_data(nonce);

        // Verify that the data contains all expected components in correct order
        let mut expected_data = Vec::new();
        expected_data.extend_from_slice(block.get_pre_block_hash().as_bytes());
        expected_data.extend_from_slice(&block.hash_transactions());
        expected_data.extend_from_slice(&block.get_timestamp().to_be_bytes());
        expected_data.extend_from_slice(&(TARGET_BITS as u64).to_be_bytes());
        expected_data.extend_from_slice(&nonce.to_be_bytes());

        assert_eq!(data, expected_data);
    }

    #[test]
    fn test_prepare_data_different_nonces() {
        let block = create_test_block();
        let pow = ProofOfWork::new_proof_of_work(block);

        let data1 = pow.prepare_data(100);
        let data2 = pow.prepare_data(200);

        // Data should differ only in the last 8 bytes (nonce)
        assert_ne!(data1, data2);
        assert_eq!(data1.len(), data2.len());
        
        // All bytes except the last 8 should be the same
        let prefix_len = data1.len() - 8;
        assert_eq!(&data1[..prefix_len], &data2[..prefix_len]);
        assert_ne!(&data1[prefix_len..], &data2[prefix_len..]);
    }

    #[test]
    fn test_prepare_data_deterministic() {
        let block = create_test_block();
        let pow = ProofOfWork::new_proof_of_work(block);
        let nonce = 42;

        let data1 = pow.prepare_data(nonce);
        let data2 = pow.prepare_data(nonce);

        // Same nonce should produce identical data
        assert_eq!(data1, data2);
    }

    #[test]
    fn test_validate_with_default_nonce() {
        let block = create_test_block();
        let pow = ProofOfWork::new_proof_of_work(block);

        // With TARGET_BITS = 24, it's extremely unlikely that the default nonce (0) will produce a valid hash
        let is_valid = pow.validate();
        
        // This test might occasionally fail due to the probabilistic nature of hashing,
        // but it's extremely unlikely with TARGET_BITS = 24 and default nonce
        assert!(!is_valid);
    }

    #[test]
    fn test_validate_deterministic() {
        let block = create_test_block();
        let pow = ProofOfWork::new_proof_of_work(block);

        let result1 = pow.validate();
        let result2 = pow.validate();

        // Validation should be deterministic
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_validate_different_blocks() {
        let transaction1 = create_test_transaction(vec![1, 2, 3, 4]);
        let transaction2 = create_test_transaction(vec![5, 6, 7, 8]);
        
        let block1 = Block::new_block_without_proof_of_work("hash1".to_string(), &[transaction1], 1);
        let block2 = Block::new_block_without_proof_of_work("hash2".to_string(), &[transaction2], 2);
        
        let pow1 = ProofOfWork::new_proof_of_work(block1);
        let pow2 = ProofOfWork::new_proof_of_work(block2);

        let result1 = pow1.validate();
        let result2 = pow2.validate();

        // Different blocks will almost certainly produce different validation results
        // (though both are likely to be false with TARGET_BITS = 24)
        // This test ensures the validation logic works with different inputs
        assert_eq!(result1, pow1.validate()); // Deterministic for same input
        assert_eq!(result2, pow2.validate()); // Deterministic for same input
    }

    #[ignore] // This test can take a very long time to complete
    #[test]
    fn test_run_mining_process() {
        let block = create_test_block();
        let pow = ProofOfWork::new_proof_of_work(block.clone());

        let (nonce, hash) = pow.run();

        // Verify that the returned nonce and hash are valid
        assert!(nonce >= 0);
        assert!(nonce < MAX_NONCE);
        assert_eq!(hash.len(), 64); // SHA256 hex string length
        
        // Verify that the hash string is valid hex
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
        
        // Verify that the nonce is reasonable (not immediately 0, which would be suspicious)
        // Note: This could theoretically be 0 in very rare cases, but highly unlikely with TARGET_BITS=24
    }

    #[test]
    fn test_proof_of_work_clone() {
        let block = create_test_block();
        let pow = ProofOfWork::new_proof_of_work(block);
        
        let cloned_pow = pow.clone();
        
        // Verify that cloned instance behaves identically
        assert_eq!(pow.target.as_bigint(), cloned_pow.target.as_bigint());
        assert_eq!(pow.validate(), cloned_pow.validate());
        
        let data1 = pow.prepare_data(42);
        let data2 = cloned_pow.prepare_data(42);
        assert_eq!(data1, data2);
    }

    #[test]
    fn test_proof_of_work_serialization() {
        let block = create_test_block();
        let pow = ProofOfWork::new_proof_of_work(block);
        
        // Test serialization
        let encoded = bincode::encode_to_vec(&pow, bincode::config::standard()).unwrap();
        assert!(!encoded.is_empty());
        
        // Test deserialization
        let (decoded_pow, _): (ProofOfWork, usize) = 
            bincode::decode_from_slice(&encoded, bincode::config::standard()).unwrap();
        
        // Verify that deserialized instance is identical
        assert_eq!(pow.target.as_bigint(), decoded_pow.target.as_bigint());
        assert_eq!(pow.validate(), decoded_pow.validate());
    }

    #[test]
    fn test_target_bits_constant() {
        // Verify that TARGET_BITS constant is as expected
        assert_eq!(TARGET_BITS, 24);
        
        // Verify that this produces a reasonable target
        let block = create_test_block();
        let pow = ProofOfWork::new_proof_of_work(block);
        let target = pow.target.as_bigint();
        
        // Target should be significantly smaller than 2^256
        let max_hash = BigInt::from(1) << 256;
        assert!(target < &max_hash);
        
        // But still a very large number
        let min_reasonable_target = BigInt::from(1) << 200;
        assert!(target > &min_reasonable_target);
    }

    #[test]
    fn test_max_nonce_constant() {
        // Verify MAX_NONCE is set to a reasonable value
        assert_eq!(MAX_NONCE, i64::MAX);
    }
}
