use rust_blockchain::{Block, Transaction, TXInput, TXOutput};
use sled::Db;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

// =============================================================================
// TRANSACTION HELPERS
// =============================================================================

/// Creates a standard test transaction with the given ID.
/// This function was duplicated across block_tests.rs, blockchain_iterator_tests.rs, 
/// and proof_of_work_tests.rs with identical implementations.
pub fn create_test_transaction(id: Vec<u8>) -> Transaction {
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

// =============================================================================
// TXOutput HELPERS  
// =============================================================================

/// Creates a sample TXOutput with default values.
pub fn create_sample_output() -> TXOutput {
    TXOutput {
        value: 100,
        pub_key_hash: vec![1, 2, 3, 4, 5],
    }
}

/// Creates a TXOutput with a custom value.
pub fn create_output_with_value(value: i32) -> TXOutput {
    TXOutput {
        value,
        pub_key_hash: vec![10, 20, 30],
    }
}

/// Creates a TXOutput with a custom pub_key_hash.
pub fn create_output_with_key_hash(key_hash: Vec<u8>) -> TXOutput {
    TXOutput {
        value: 50,
        pub_key_hash: key_hash,
    }
}


// =============================================================================
// BLOCK HELPERS  
// =============================================================================

/// Creates a test block with the given parameters.
/// Consolidates similar functions from blockchain_iterator_tests.rs and proof_of_work_tests.rs.
pub fn create_test_block(pre_hash: String, height: usize) -> Block {
    let transaction = create_test_transaction(vec![1, 2, 3, 4]);
    let transactions = vec![transaction];
    Block::new_block_without_proof_of_work(pre_hash, &transactions, height)
}

/// Creates a test block with default parameters.
/// This matches the hardcoded version from proof_of_work_tests.rs.
pub fn create_default_test_block() -> Block {
    create_test_block("test_previous_hash".to_string(), 1)
}

/// Creates a test block with multiple transactions.
pub fn create_test_block_with_transactions(
    pre_hash: String,
    transactions: &[Transaction],
    height: usize,
) -> Block {
    Block::new_block_without_proof_of_work(pre_hash, transactions, height)
}

/// Creates a genesis block for testing.
pub fn create_test_genesis_block() -> Block {
    let transaction = create_test_transaction(vec![0, 0, 0, 0]);
    Block::generate_genesis_block(&transaction)
}

// =============================================================================
// DATABASE HELPERS
// =============================================================================

/// Creates a test database with a unique name based on the test name and process ID.
/// This consolidates the create_test_db function from blockchain_tests.rs.
pub fn create_test_db(test_name: &str) -> Db {
    let test_path = format!("test_db_{}_{}", test_name, std::process::id());
    sled::open(&test_path).unwrap()
}

/// Creates a temporary test database that will be automatically cleaned up.
/// This consolidates the setup_test_db function from blockchain_iterator_tests.rs.
pub fn setup_temp_test_db() -> (Db, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db = sled::open(temp_dir.path()).expect("Failed to open database");
    (db, temp_dir)
}

/// Cleans up a test database by removing its directory.
/// This consolidates the cleanup_test_db function from blockchain_tests.rs.
pub fn cleanup_test_db(test_path: &str) {
    if Path::new(test_path).exists() {
        let _ = fs::remove_dir_all(test_path);
    }
}

/// Creates a test database with automatic cleanup function.
pub fn create_test_db_with_cleanup(test_name: &str) -> (Db, impl Fn()) {
    let test_path = format!("test_db_{}_{}", test_name, std::process::id());
    let db = sled::open(&test_path).unwrap();
    let cleanup_path = test_path.clone();
    let cleanup = move || cleanup_test_db(&cleanup_path);
    (db, cleanup)
}

// =============================================================================
// ASSERTION HELPERS
// =============================================================================

/// Asserts that two blocks have the same core properties (excluding timestamps).
pub fn assert_blocks_equal_ignoring_timestamp(block1: &Block, block2: &Block) {
    assert_eq!(block1.get_hash(), block2.get_hash());
    assert_eq!(block1.get_pre_block_hash(), block2.get_pre_block_hash());
    assert_eq!(block1.get_height(), block2.get_height());
    assert_eq!(block1.get_nonce(), block2.get_nonce());
    assert_eq!(block1.get_transactions().len(), block2.get_transactions().len());
}

/// Asserts that serialization/deserialization works correctly for a block.
pub fn assert_block_serialization_works(block: &Block) {
    let serialized = block.serialize();
    assert!(!serialized.is_empty());
    
    let deserialized = Block::deserialize(&serialized);
    assert_blocks_equal_ignoring_timestamp(block, &deserialized);
    assert_eq!(block.get_timestamp(), deserialized.get_timestamp()); // Include timestamp check
}

// =============================================================================
// TEST DATA GENERATORS
// =============================================================================

/// Generates test data with various edge cases for comprehensive testing.
pub struct TestDataGenerator;

impl TestDataGenerator {
    /// Generates a series of test transaction IDs with different patterns.
    pub fn generate_test_ids(count: usize) -> Vec<Vec<u8>> {
        let mut ids = Vec::new();
        for i in 0..count {
            ids.push(vec![i as u8, (i >> 8) as u8, (i >> 16) as u8, (i >> 24) as u8]);
        }
        ids
    }
    
    /// Generates test blocks forming a valid blockchain.
    pub fn generate_blockchain_sequence(block_count: usize) -> Vec<Block> {
        let mut blocks = Vec::new();
        
        // Genesis block
        let genesis = create_test_genesis_block();
        blocks.push(genesis);
        
        // Subsequent blocks
        for i in 1..block_count {
            let prev_hash = blocks[i - 1].get_hash().to_string();
            let block = create_test_block(prev_hash, i);
            blocks.push(block);
        }
        
        blocks
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_transaction() {
        let id = vec![1, 2, 3, 4];
        let tx = create_test_transaction(id.clone());
        
        assert_eq!(tx.get_id(), id.as_slice());
        assert_eq!(tx.vin.len(), 1);
        assert_eq!(tx.vout.len(), 1);
        assert_eq!(tx.vin[0].get_txid(), &[1, 2, 3]);
        assert_eq!(tx.vout[0].get_value(), 100);
    }
    
    #[test]
    fn test_create_test_block() {
        let block = create_test_block("test_hash".to_string(), 5);
        
        assert_eq!(block.get_pre_block_hash(), "test_hash");
        assert_eq!(block.get_height(), 5);
        assert_eq!(block.get_transactions().len(), 1);
        assert!(block.get_timestamp() > 0);
    }
    
    #[test]
    fn test_setup_temp_test_db() {
        let (db, _temp_dir) = setup_temp_test_db();
        
        // Should be able to open a tree
        let _ = db.open_tree("test_tree").unwrap();
        
        // TempDir will automatically clean up when dropped
    }
    
    #[test]
    fn test_block_serialization_helper() {
        let block = create_default_test_block();
        assert_block_serialization_works(&block);
    }
    
    #[test]
    fn test_data_generator() {
        let ids = TestDataGenerator::generate_test_ids(5);
        assert_eq!(ids.len(), 5);
        assert_eq!(ids[0], vec![0, 0, 0, 0]);
        assert_eq!(ids[4], vec![4, 0, 0, 0]);
        
        let blocks = TestDataGenerator::generate_blockchain_sequence(3);
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0].get_height(), 0); // Genesis
        assert_eq!(blocks[1].get_height(), 1);
        assert_eq!(blocks[2].get_height(), 2);
    }
} 
