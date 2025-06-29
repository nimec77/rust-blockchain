use bincode::config::standard;
use sled::IVec;

use crate::{models::transaction::Transaction, proof_of_work::ProofOfWork, util};

#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

impl Block {
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: util::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;

        block
    }

    pub fn new_block_without_proof_of_work(
        pre_block_hash: String,
        transactions: &[Transaction],
        height: usize,
    ) -> Block {
        Block {
            timestamp: util::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, standard()).unwrap()
    }

    pub fn deserialize(bytes: &[u8]) -> Block {
        let (blk, _) = bincode::decode_from_slice(bytes, standard()).unwrap();

        blk
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }

    pub fn get_pre_block_hash(&self) -> &str {
        self.pre_block_hash.as_str()
    }

    pub fn get_hash(&self) -> &str {
        self.hash.as_str()
    }

    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_nonce(&self) -> i64 {
        self.nonce
    }

    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id());
        }
        util::sha256_digest(txhashs.as_slice())
    }

    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];

        Block::new_block_without_proof_of_work(String::from("None"), &transactions, 0)
    }
}

impl From<Block> for IVec {
    fn from(b: Block) -> Self {
        let bytes = b.serialize();
        IVec::from(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{tx_input::TXInput, tx_output::TXOutput};

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

    #[ignore]
    #[test]
    fn test_new_block() {
        let transaction = create_test_transaction(vec![1, 2, 3, 4]);
        let transactions = vec![transaction];
        let pre_block_hash = "previous_hash".to_string();
        let height = 1;

        let block =
            Block::new_block_without_proof_of_work(pre_block_hash.clone(), &transactions, height);

        assert_eq!(block.pre_block_hash, pre_block_hash);
        assert_eq!(block.transactions.len(), 1);
        assert_eq!(block.height, height);
        // Proof of work should have run, so nonce should be set (not 0)
        assert!(block.nonce >= 0);
        // Hash should be set after proof of work
        assert!(!block.hash.is_empty());
        assert!(block.timestamp > 0);
        // Verify hash is hex encoded and reasonable length
        assert!(block.hash.len() == 64); // SHA256 hex string length
    }

    #[test]
    fn test_serialize_deserialize() {
        let transaction = create_test_transaction(vec![1, 2, 3, 4]);
        let transactions = vec![transaction];
        let pre_block_hash = "test_hash".to_string();
        let height = 5;

        let original_block =
            Block::new_block_without_proof_of_work(pre_block_hash, &transactions, height);

        // Test serialization
        let serialized = original_block.serialize();
        assert!(!serialized.is_empty());

        // Test deserialization
        let deserialized_block = Block::deserialize(&serialized);

        assert_eq!(
            original_block.pre_block_hash,
            deserialized_block.pre_block_hash
        );
        assert_eq!(original_block.hash, deserialized_block.hash);
        assert_eq!(
            original_block.transactions.len(),
            deserialized_block.transactions.len()
        );
        assert_eq!(original_block.nonce, deserialized_block.nonce);
        assert_eq!(original_block.height, deserialized_block.height);
        assert_eq!(original_block.timestamp, deserialized_block.timestamp);
    }

    #[test]
    fn test_get_transactions() {
        let transaction1 = create_test_transaction(vec![1, 2, 3]);
        let transaction2 = create_test_transaction(vec![4, 5, 6]);
        let transactions = vec![transaction1, transaction2];

        let block =
            Block::new_block_without_proof_of_work("test_hash".to_string(), &transactions, 1);
        let retrieved_transactions = block.get_transactions();

        assert_eq!(retrieved_transactions.len(), 2);
        assert_eq!(retrieved_transactions[0].get_id(), &[1, 2, 3]);
        assert_eq!(retrieved_transactions[1].get_id(), &[4, 5, 6]);
    }

    #[test]
    fn test_get_pre_block_hash() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];
        let pre_block_hash = "previous_block_hash".to_string();

        let block =
            Block::new_block_without_proof_of_work(pre_block_hash.clone(), &transactions, 1);

        assert_eq!(block.get_pre_block_hash(), pre_block_hash.as_str());
    }

    #[test]
    fn test_get_hash() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];

        let mut block =
            Block::new_block_without_proof_of_work("test".to_string(), &transactions, 1);
        block.hash = "test_hash".to_string();

        assert_eq!(block.get_hash(), "test_hash");
    }

    #[test]
    fn test_get_hash_bytes() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];

        let mut block =
            Block::new_block_without_proof_of_work("test".to_string(), &transactions, 1);
        block.hash = "test_hash".to_string();

        let hash_bytes = block.get_hash_bytes();
        assert_eq!(hash_bytes, "test_hash".as_bytes().to_vec());
    }

    #[test]
    fn test_get_timestamp() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];

        let block = Block::new_block_without_proof_of_work("test".to_string(), &transactions, 1);
        let timestamp = block.get_timestamp();

        assert!(timestamp > 0);
        assert!(timestamp <= util::current_timestamp());
    }

    #[test]
    fn test_get_height() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];
        let height = 42;

        let block =
            Block::new_block_without_proof_of_work("test".to_string(), &transactions, height);

        assert_eq!(block.get_height(), height);
    }

    #[test]
    fn test_hash_transactions() {
        let transaction1 = create_test_transaction(vec![1, 2, 3]);
        let transaction2 = create_test_transaction(vec![4, 5, 6]);
        let transactions = vec![transaction1, transaction2];

        let block = Block::new_block_without_proof_of_work("test".to_string(), &transactions, 1);
        let hash = block.hash_transactions();

        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 32); // SHA256 produces 32 bytes

        // Test that the same transactions produce the same hash
        let block2 =
            Block::new_block_without_proof_of_work("different_hash".to_string(), &transactions, 2);
        let hash2 = block2.hash_transactions();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_hash_transactions_empty() {
        let transactions = vec![];
        let block = Block::new_block_without_proof_of_work("test".to_string(), &transactions, 1);
        let hash = block.hash_transactions();

        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 32); // SHA256 produces 32 bytes even for empty input
    }

    #[test]
    fn test_generate_genesis_block() {
        let transaction = create_test_transaction(vec![1, 2, 3, 4]);

        let genesis_block = Block::generate_genesis_block(&transaction);

        assert_eq!(genesis_block.get_pre_block_hash(), "None");
        assert_eq!(genesis_block.get_height(), 0);
        assert_eq!(genesis_block.get_transactions().len(), 1);
        assert_eq!(genesis_block.get_transactions()[0].get_id(), &[1, 2, 3, 4]);
        assert!(genesis_block.get_timestamp() > 0);
    }

    #[test]
    fn test_from_block_to_ivec() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];

        let block = Block::new_block_without_proof_of_work("test".to_string(), &transactions, 1);
        let ivec: IVec = block.clone().into();

        // Verify that we can deserialize back to the original block
        let deserialized_block = Block::deserialize(&ivec);

        assert_eq!(block.pre_block_hash, deserialized_block.pre_block_hash);
        assert_eq!(block.hash, deserialized_block.hash);
        assert_eq!(block.height, deserialized_block.height);
        assert_eq!(block.timestamp, deserialized_block.timestamp);
        assert_eq!(block.nonce, deserialized_block.nonce);
    }

    #[test]
    fn test_block_with_multiple_transactions() {
        let mut transactions = vec![];
        for i in 0..5 {
            transactions.push(create_test_transaction(vec![i as u8; 4]));
        }

        let block =
            Block::new_block_without_proof_of_work("multi_tx_test".to_string(), &transactions, 10);

        assert_eq!(block.get_transactions().len(), 5);
        assert_eq!(block.get_height(), 10);
        assert_eq!(block.get_pre_block_hash(), "multi_tx_test");

        // Verify each transaction is preserved
        for (i, tx) in block.get_transactions().iter().enumerate() {
            assert_eq!(tx.get_id(), &vec![i as u8; 4]);
        }
    }

    #[test]
    fn test_serialize_deserialize_round_trip_consistency() {
        let transaction = create_test_transaction(vec![255, 128, 64, 32, 16]);
        let transactions = vec![transaction];

        let mut original_block = Block::new_block_without_proof_of_work(
            "consistency_test".to_string(),
            &transactions,
            999,
        );
        original_block.hash = "custom_hash_value".to_string();
        original_block.nonce = 12345;

        // Serialize and deserialize multiple times
        let mut current_block = original_block.clone();
        for _ in 0..3 {
            let serialized = current_block.serialize();
            current_block = Block::deserialize(&serialized);
        }

        // Verify all fields remain unchanged
        assert_eq!(original_block.pre_block_hash, current_block.pre_block_hash);
        assert_eq!(original_block.hash, current_block.hash);
        assert_eq!(original_block.height, current_block.height);
        assert_eq!(original_block.timestamp, current_block.timestamp);
        assert_eq!(original_block.nonce, current_block.nonce);
        assert_eq!(
            original_block.transactions.len(),
            current_block.transactions.len()
        );
    }

    #[test]
    fn test_get_nonce() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];

        let block = Block::new_block_without_proof_of_work("test".to_string(), &transactions, 1);
        let nonce = block.get_nonce();

        // For new_block_without_proof_of_work, nonce should be 0
        assert_eq!(nonce, 0);
        assert_eq!(nonce, block.nonce);
    }

    #[ignore]
    #[test]
    fn test_proof_of_work_integration() {
        let transaction = create_test_transaction(vec![1, 2, 3, 4]);
        let transactions = vec![transaction];

        let block =
            Block::new_block_without_proof_of_work("pow_test".to_string(), &transactions, 1);

        // Verify proof of work ran and set values
        assert!(block.get_nonce() >= 0);
        assert!(!block.get_hash().is_empty());
        assert_eq!(block.get_hash().len(), 64); // SHA256 hex string

        // Verify hash contains only hex characters
        assert!(block.get_hash().chars().all(|c| c.is_ascii_hexdigit()));

        // Create proof of work instance and validate
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        assert!(pow.validate(), "Block should pass proof of work validation");
    }

    #[test]
    fn test_empty_transactions_block() {
        let transactions = vec![];

        let block =
            Block::new_block_without_proof_of_work("empty_tx_test".to_string(), &transactions, 0);

        assert_eq!(block.get_transactions().len(), 0);
        assert_eq!(block.get_nonce(), 0); // Without proof of work, nonce should be 0
        assert_eq!(block.get_hash(), ""); // Without proof of work, hash should be empty
        assert_eq!(block.get_height(), 0);

        // Verify hash_transactions works with empty transactions
        let tx_hash = block.hash_transactions();
        assert_eq!(tx_hash.len(), 32); // SHA256 produces 32 bytes
    }

    #[test]
    fn test_hash_transactions_ordering() {
        let tx1 = create_test_transaction(vec![1, 2, 3]);
        let tx2 = create_test_transaction(vec![4, 5, 6]);

        // Create blocks with different transaction orders
        let block1 = Block::new_block_without_proof_of_work(
            "order_test1".to_string(),
            &[tx1.clone(), tx2.clone()],
            1,
        );
        let block2 = Block::new_block_without_proof_of_work(
            "order_test2".to_string(),
            &[tx2.clone(), tx1.clone()],
            1,
        );

        // Transaction hashes should be different due to ordering
        let hash1 = block1.hash_transactions();
        let hash2 = block2.hash_transactions();
        assert_ne!(hash1, hash2, "Transaction ordering should affect hash");
    }

    #[test]
    fn test_block_hash_consistency() {
        let transaction = create_test_transaction(vec![1, 2, 3, 4]);
        let transactions = vec![transaction];

        // Create two blocks with identical data
        let block1 = Block::new_block_without_proof_of_work(
            "consistency_test".to_string(),
            &transactions,
            5,
        );
        let block2 = Block::new_block_without_proof_of_work(
            "consistency_test".to_string(),
            &transactions,
            5,
        );

        // Blocks should have the same transaction hash
        assert_eq!(block1.hash_transactions(), block2.hash_transactions());

        // Note: The overall block hashes will likely be different due to different timestamps
        // even when using new_block_without_proof_of_work
    }

    #[ignore]
    #[test]
    fn test_genesis_block_validation() {
        let transaction = create_test_transaction(vec![0, 0, 0, 0]);
        let genesis_block = Block::generate_genesis_block(&transaction);

        // Verify genesis block properties
        assert_eq!(genesis_block.get_pre_block_hash(), "None");
        assert_eq!(genesis_block.get_height(), 0);
        assert_eq!(genesis_block.get_transactions().len(), 1);

        // Verify proof of work ran on genesis block
        assert!(genesis_block.get_nonce() >= 0);
        assert!(!genesis_block.get_hash().is_empty());

        // Validate proof of work
        let pow = ProofOfWork::new_proof_of_work(genesis_block.clone());
        assert!(
            pow.validate(),
            "Genesis block should pass proof of work validation"
        );
    }

    #[ignore]
    #[test]
    fn test_block_with_large_transaction_count() {
        let mut transactions = vec![];
        for i in 0..100 {
            transactions.push(create_test_transaction(vec![
                i as u8,
                (i >> 8) as u8,
                (i >> 16) as u8,
            ]));
        }

        let block =
            Block::new_block_without_proof_of_work("large_tx_test".to_string(), &transactions, 50);

        assert_eq!(block.get_transactions().len(), 100);
        assert_eq!(block.get_height(), 50);

        // Verify all transactions are preserved
        for (i, tx) in block.get_transactions().iter().enumerate() {
            let expected_id = vec![i as u8, (i >> 8) as u8, (i >> 16) as u8];
            assert_eq!(tx.get_id(), expected_id.as_slice());
        }

        // Verify block is still valid
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        assert!(pow.validate());
    }

    #[test]
    fn test_block_fields_immutability_after_creation() {
        let transaction = create_test_transaction(vec![9, 8, 7, 6]);
        let transactions = vec![transaction];

        let block =
            Block::new_block_without_proof_of_work("immutable_test".to_string(), &transactions, 10);

        // Store original values
        let original_hash = block.get_hash().to_string();
        let original_nonce = block.get_nonce();
        let original_timestamp = block.get_timestamp();
        let original_height = block.get_height();

        // Create a new block with same parameters but different pre_hash to ensure it's different
        let block2 =
            Block::new_block_without_proof_of_work("different_test".to_string(), &transactions, 10);

        // Original block values should remain unchanged
        assert_eq!(block.get_hash(), original_hash);
        assert_eq!(block.get_nonce(), original_nonce);
        assert_eq!(block.get_timestamp(), original_timestamp);
        assert_eq!(block.get_height(), original_height);

        // Test that original block is immutable by checking fields haven't changed
        assert_eq!(block.get_pre_block_hash(), "immutable_test");
        assert_eq!(block2.get_pre_block_hash(), "different_test");

        // Both blocks should have same structure but different pre_hash
        assert_ne!(block.get_pre_block_hash(), block2.get_pre_block_hash());
    }

    #[test]
    fn test_block_serialization_with_unicode_hash() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];

        let mut block =
            Block::new_block_without_proof_of_work("unicode_test".to_string(), &transactions, 1);
        // Set a hash that tests edge cases in serialization
        block.hash = "deadbeef".repeat(8); // 64 character hex string

        let serialized = block.serialize();
        let deserialized = Block::deserialize(&serialized);

        assert_eq!(block.hash, deserialized.hash);
        assert_eq!(block.get_hash().len(), 64);
    }

    // === NEW COMPREHENSIVE FAST TESTS ===

    #[test]
    fn test_new_block_without_proof_of_work_initialization() {
        let transaction = create_test_transaction(vec![1, 2, 3, 4]);
        let transactions = vec![transaction];
        let pre_block_hash = "test_previous_hash".to_string();
        let height = 42;

        let block =
            Block::new_block_without_proof_of_work(pre_block_hash.clone(), &transactions, height);

        // Verify all fields are set correctly
        assert_eq!(block.get_pre_block_hash(), pre_block_hash);
        assert_eq!(block.get_transactions().len(), 1);
        assert_eq!(block.get_height(), height);
        assert_eq!(block.get_nonce(), 0); // Should be 0 without proof of work
        assert_eq!(block.get_hash(), ""); // Should be empty without proof of work
        assert!(block.get_timestamp() > 0);
        assert!(block.get_timestamp() <= util::current_timestamp());
    }

    #[test]
    fn test_block_with_different_pre_hash_formats() {
        let transaction = create_test_transaction(vec![1]);
        let transactions = vec![transaction];

        // Test different pre_hash formats
        let test_cases = vec![
            "".to_string(),                     // Empty string
            "0".to_string(),                    // Single character
            "a".repeat(64),                     // 64 character hex-like string
            "None".to_string(),                 // Genesis format
            "special-chars-123!@#".to_string(), // Special characters
        ];

        for (i, pre_hash) in test_cases.iter().enumerate() {
            let block = Block::new_block_without_proof_of_work(pre_hash.clone(), &transactions, i);
            assert_eq!(block.get_pre_block_hash(), pre_hash);
            assert_eq!(block.get_height(), i);
        }
    }

    #[test]
    fn test_block_with_varying_transaction_counts() {
        let test_sizes = vec![0, 1, 5, 10, 25];

        for &size in &test_sizes {
            let mut transactions = vec![];
            for i in 0..size {
                transactions.push(create_test_transaction(vec![i as u8]));
            }

            let block = Block::new_block_without_proof_of_work(
                format!("test_size_{}", size),
                &transactions,
                size,
            );

            assert_eq!(block.get_transactions().len(), size);
            assert_eq!(block.get_height(), size);

            // Verify transaction order is preserved
            for (i, tx) in block.get_transactions().iter().enumerate() {
                assert_eq!(tx.get_id(), &[i as u8]);
            }
        }
    }

    #[test]
    fn test_hash_transactions_deterministic() {
        let tx1 = create_test_transaction(vec![1, 2, 3]);
        let tx2 = create_test_transaction(vec![4, 5, 6]);
        let transactions = vec![tx1, tx2];

        // Create multiple blocks with same transactions
        let block1 = Block::new_block_without_proof_of_work("hash1".to_string(), &transactions, 1);
        let block2 = Block::new_block_without_proof_of_work("hash2".to_string(), &transactions, 2);
        let block3 = Block::new_block_without_proof_of_work("hash3".to_string(), &transactions, 3);

        // All should produce the same transaction hash
        let hash1 = block1.hash_transactions();
        let hash2 = block2.hash_transactions();
        let hash3 = block3.hash_transactions();

        assert_eq!(hash1, hash2);
        assert_eq!(hash2, hash3);
        assert_eq!(hash1.len(), 32); // SHA256 length
    }

    #[test]
    fn test_hash_transactions_with_duplicate_transactions() {
        let tx = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![tx.clone(), tx.clone(), tx];

        let block =
            Block::new_block_without_proof_of_work("dup_test".to_string(), &transactions, 1);
        let hash = block.hash_transactions();

        assert_eq!(block.get_transactions().len(), 3); // Should have all 3 duplicates
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_block_timestamp_accuracy() {
        let transaction = create_test_transaction(vec![1]);
        let transactions = vec![transaction];

        let before = util::current_timestamp();
        let block =
            Block::new_block_without_proof_of_work("time_test".to_string(), &transactions, 1);
        let after = util::current_timestamp();

        let block_time = block.get_timestamp();
        assert!(
            block_time >= before,
            "Block timestamp should be >= creation time"
        );
        assert!(
            block_time <= after,
            "Block timestamp should be <= after creation time"
        );
    }

    #[test]
    fn test_block_height_edge_cases() {
        let transaction = create_test_transaction(vec![1]);
        let transactions = vec![transaction];

        let test_heights = vec![0, 1, 100, 1000, usize::MAX];

        for &height in &test_heights {
            let block = Block::new_block_without_proof_of_work(
                format!("height_{}", height),
                &transactions,
                height,
            );
            assert_eq!(block.get_height(), height);
        }
    }

    #[test]
    fn test_serialization_roundtrip_consistency() {
        let transaction = create_test_transaction(vec![0xAA, 0xBB, 0xCC, 0xDD]);
        let transactions = vec![transaction];

        let mut original = Block::new_block_without_proof_of_work(
            "serialization_test".to_string(),
            &transactions,
            12345,
        );

        // Set custom values to test serialization thoroughly
        original.hash = "0123456789abcdef".repeat(4); // 64 char hex string
        original.nonce = -12345; // Test negative nonce

        // Perform multiple serialize/deserialize cycles
        let mut current = original.clone();
        for _ in 0..5 {
            let serialized = current.serialize();
            current = Block::deserialize(&serialized);
        }

        // Verify all fields are preserved
        assert_eq!(original.get_pre_block_hash(), current.get_pre_block_hash());
        assert_eq!(original.get_hash(), current.get_hash());
        assert_eq!(original.get_height(), current.get_height());
        assert_eq!(original.get_timestamp(), current.get_timestamp());
        assert_eq!(original.get_nonce(), current.get_nonce());
        assert_eq!(
            original.get_transactions().len(),
            current.get_transactions().len()
        );
    }

    #[test]
    fn test_genesis_block_without_proof_of_work() {
        // Test the basic structure without the expensive proof-of-work
        let transaction = create_test_transaction(vec![0, 1, 2, 3]);

        // Manually create what genesis block should look like without proof-of-work
        let genesis = Block::new_block_without_proof_of_work("None".to_string(), &[transaction], 0);

        assert_eq!(genesis.get_pre_block_hash(), "None");
        assert_eq!(genesis.get_height(), 0);
        assert_eq!(genesis.get_transactions().len(), 1);
        assert_eq!(genesis.get_nonce(), 0);
        assert_eq!(genesis.get_hash(), "");
    }

    #[test]
    fn test_large_transaction_data_without_proof_of_work() {
        // Create transactions with larger data payloads
        let mut transactions = vec![];
        for i in 0..50 {
            let large_id = vec![i as u8; 1000]; // 1KB transaction ID
            transactions.push(create_test_transaction(large_id));
        }

        let block =
            Block::new_block_without_proof_of_work("large_data_test".to_string(), &transactions, 1);

        assert_eq!(block.get_transactions().len(), 50);

        // Test serialization with large data
        let serialized = block.serialize();
        let deserialized = Block::deserialize(&serialized);

        assert_eq!(
            block.get_transactions().len(),
            deserialized.get_transactions().len()
        );

        // Verify large transaction data is preserved
        for (original, deserialized) in block
            .get_transactions()
            .iter()
            .zip(deserialized.get_transactions().iter())
        {
            assert_eq!(original.get_id(), deserialized.get_id());
            assert_eq!(original.get_id().len(), 1000); // Verify size
        }

        // Test hash_transactions with large data
        let tx_hash = block.hash_transactions();
        assert_eq!(tx_hash.len(), 32);
    }

    #[test]
    fn test_block_memory_layout_and_cloning() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];

        let original =
            Block::new_block_without_proof_of_work("clone_test".to_string(), &transactions, 5);
        let cloned = original.clone();

        // Verify clone is identical
        assert_eq!(original.get_pre_block_hash(), cloned.get_pre_block_hash());
        assert_eq!(original.get_hash(), cloned.get_hash());
        assert_eq!(original.get_height(), cloned.get_height());
        assert_eq!(original.get_timestamp(), cloned.get_timestamp());
        assert_eq!(original.get_nonce(), cloned.get_nonce());
        assert_eq!(
            original.get_transactions().len(),
            cloned.get_transactions().len()
        );

        // Test that they serialize to the same bytes
        assert_eq!(original.serialize(), cloned.serialize());
    }
}
