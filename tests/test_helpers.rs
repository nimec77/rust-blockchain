#![allow(dead_code)]

use rust_blockchain::{BLOCKS_TREE, Block, TXInput, TXOutput, Transaction};
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
    let mut block = Block::new_block_without_proof_of_work(pre_hash, &transactions, height);

    // Generate a unique hash for each test block based on its contents
    let hash_input = format!(
        "{}|{}|{}",
        block.get_pre_block_hash(),
        block.get_height(),
        block.get_timestamp()
    );
    let hash_bytes = rust_blockchain::util::sha256_digest(hash_input.as_bytes());
    block.set_hash(&data_encoding::HEXLOWER.encode(&hash_bytes));

    block
}

/// Creates a test block with default parameters.
/// This matches the hardcoded version from proof_of_work_tests.rs.
pub fn create_default_test_block() -> Block {
    create_test_block("test_previous_hash".to_string(), 1)
}

/// Creates a genesis block for testing.
pub fn create_test_genesis_block() -> Block {
    let transaction = create_test_transaction(vec![0, 0, 0, 0]);
    Block::generate_genesis_block(&transaction)
}

// =============================================================================
// DATABASE HELPERS
// =============================================================================

/// RAII wrapper for test databases that ensures cleanup on drop.
pub struct TestDatabase {
    pub db: Option<Db>, // Changed to Option to allow clean shutdown
    pub path: String,
}

impl TestDatabase {
    /// Creates a new test database with automatic cleanup.
    pub fn new(test_name: &str) -> Self {
        let path = format!("test_db_{}_{}", test_name, std::process::id());
        let db = sled::open(&path).unwrap();
        Self { db: Some(db), path }
    }

    /// Creates a test database with a custom path.
    pub fn new_with_path(path: String) -> Self {
        let db = sled::open(&path).unwrap();
        Self { db: Some(db), path }
    }

    /// Gets a reference to the database.
    pub fn get_db(&self) -> &Db {
        self.db.as_ref().expect("Database has been closed")
    }

    /// Manually triggers cleanup (usually not needed due to Drop impl).
    pub fn cleanup(&mut self) {
        if let Some(db) = self.db.take() {
            // Flush and close the database cleanly
            let _ = db.flush();
            drop(db);

            // Give the OS time to release file handles
            std::thread::sleep(std::time::Duration::from_millis(10));

            // Now try to remove the directory
            robust_cleanup_test_db(&self.path);
        }
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        self.cleanup();
    }
}

impl std::ops::Deref for TestDatabase {
    type Target = Db;

    fn deref(&self) -> &Self::Target {
        self.get_db()
    }
}

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

/// Robust cleanup function that handles locked databases and retries if necessary.
pub fn robust_cleanup_test_db(test_path: &str) {
    use std::{thread, time::Duration};

    if !Path::new(test_path).exists() {
        return;
    }

    // Try cleanup multiple times with increasing delays to handle locked files
    for attempt in 0..10 {
        match fs::remove_dir_all(test_path) {
            Ok(()) => {
                return; // Success
            }
            Err(e) => {
                // Check if it's a "file not found" error, which means cleanup succeeded
                if e.kind() == std::io::ErrorKind::NotFound {
                    return;
                }

                if attempt == 9 {
                    // Final attempt failed, but don't panic in cleanup
                    eprintln!(
                        "Warning: Failed to clean up test database '{}' after {} attempts: {}",
                        test_path,
                        attempt + 1,
                        e
                    );
                    eprintln!(
                        "This may cause leftover test files. You can manually delete: {test_path}"
                    );
                } else {
                    // Wait with exponential backoff and try again
                    let delay = Duration::from_millis(50 * (1 << attempt)); // 50ms, 100ms, 200ms, etc.
                    thread::sleep(delay);
                }
            }
        }
    }
}

/// Cleans up all test databases by finding and removing directories that match the test pattern.
pub fn cleanup_all_test_dbs() {
    let current_dir = std::env::current_dir().unwrap();

    if let Ok(entries) = fs::read_dir(&current_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir()
                && let Some(dir_name) = path.file_name().and_then(|n| n.to_str())
                && dir_name.starts_with("test_db_")
            {
                println!("Cleaning up leftover test database: {dir_name}");
                robust_cleanup_test_db(&path.to_string_lossy());
            }
        }
    }
}

/// Cleanup script that can be called manually or programmatically.
/// Returns the count of directories cleaned up.
pub fn cleanup_script() -> usize {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let mut cleaned_count = 0;

    println!("🧹 Cleaning up test databases...");

    if let Ok(entries) = fs::read_dir(&current_dir) {
        let test_dirs: Vec<(String, String)> = entries
            .flatten()
            .filter_map(|entry| {
                let path = entry.path();
                if path.is_dir()
                    && let Some(dir_name) = path.file_name().and_then(|n| n.to_str())
                    && dir_name.starts_with("test_db_")
                {
                    return Some((path.to_string_lossy().to_string(), dir_name.to_string()));
                }
                None
            })
            .collect();

        if test_dirs.is_empty() {
            println!("✅ No test databases found to clean up.");
        } else {
            println!(
                "🗑️  Found {} test database(s) to clean up:",
                test_dirs.len()
            );

            for (path_str, dir_name) in test_dirs {
                print!("   Removing: {dir_name} ... ");
                robust_cleanup_test_db(&path_str);

                // Check if it was actually removed
                if !Path::new(&path_str).exists() {
                    println!("✅");
                    cleaned_count += 1;
                } else {
                    println!("⚠️  (may still exist)");
                }
            }

            println!("🎉 Cleanup completed! Removed {cleaned_count} database(s).");
        }
    } else {
        println!("❌ Failed to read current directory for cleanup.");
    }

    cleaned_count
}

/// Setup function to call at the beginning of test runs.
pub fn setup_test_environment() {
    // Clean up any leftover test databases from previous runs
    cleanup_all_test_dbs();
}

/// Teardown function to call at the end of test runs.
pub fn teardown_test_environment() {
    // Clean up all test databases
    cleanup_all_test_dbs();
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
    assert_eq!(
        block1.get_transactions().len(),
        block2.get_transactions().len()
    );
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
            ids.push(vec![
                i as u8,
                (i >> 8) as u8,
                (i >> 16) as u8,
                (i >> 24) as u8,
            ]);
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

// Helper function to setup test database with blocks
pub fn setup_test_db_with_blocks() -> (TestDatabase, Vec<Block>) {
    let test_db = TestDatabase::new("iterator_setup");
    let blocks_tree = test_db.open_tree(BLOCKS_TREE).unwrap();

    // Create a chain of 3 blocks
    let block1 = create_test_block("".to_string(), 0);
    let block2 = create_test_block(block1.get_hash().to_string(), 1);
    let block3 = create_test_block(block2.get_hash().to_string(), 2);

    // Store blocks in database
    blocks_tree
        .insert(block1.get_hash(), block1.serialize())
        .unwrap();
    blocks_tree
        .insert(block2.get_hash(), block2.serialize())
        .unwrap();
    blocks_tree
        .insert(block3.get_hash(), block3.serialize())
        .unwrap();

    let blocks = vec![block1, block2, block3];
    (test_db, blocks)
}

// Helper function to create a regular transaction that spends UTXOs
pub fn create_spending_transaction(
    inputs: Vec<(Vec<u8>, usize)>,
    outputs: Vec<(i32, Vec<u8>)>,
) -> Transaction {
    let mut vin = Vec::new();
    for (txid, vout_idx) in inputs {
        let mut tx_input = TXInput::new(&txid, vout_idx);
        tx_input.pub_key = vec![1, 2, 3]; // Non-empty pub_key for regular transaction
        tx_input.signature = vec![4, 5, 6]; // Mock signature
        vin.push(tx_input);
    }

    let mut vout = Vec::new();
    for (value, pub_key_hash) in outputs {
        vout.push(TXOutput {
            value,
            pub_key_hash,
        });
    }

    let mut transaction = Transaction {
        id: vec![],
        vin,
        vout,
    };

    // Generate a unique ID for the transaction
    let tx_data = format!(
        "spending_{}_{}_{}",
        transaction.vin.len(),
        transaction.vout.len(),
        rust_blockchain::util::current_timestamp()
    );
    transaction.id = rust_blockchain::util::sha256_digest(tx_data.as_bytes());

    transaction
}

// Helper function to create a coinbase transaction
pub fn create_coinbase_transaction(reward: i32, recipient_hash: Vec<u8>) -> Transaction {
    let mut coinbase_input = TXInput::new(&[], 0);
    coinbase_input.pub_key = vec![]; // Empty pub_key indicates coinbase

    let coinbase_output = TXOutput {
        value: reward,
        pub_key_hash: recipient_hash,
    };

    let mut transaction = Transaction {
        id: vec![],
        vin: vec![coinbase_input],
        vout: vec![coinbase_output],
    };

    // Generate a unique ID for the transaction
    let tx_data = format!(
        "coinbase_{}_{}_{}",
        reward,
        data_encoding::HEXLOWER.encode(&transaction.vout[0].pub_key_hash),
        rust_blockchain::util::current_timestamp()
    );
    transaction.id = rust_blockchain::util::sha256_digest(tx_data.as_bytes());

    transaction
}

// =============================================================================
// HELPER FUNCTIONS FOR MEMORY POOL TESTS
// =============================================================================

/// Creates multiple test transactions with different IDs
pub fn create_multiple_test_transactions(count: usize) -> Vec<Transaction> {
    let mut transactions = Vec::new();
    for i in 0..count {
        let id = vec![i as u8; 4]; // Create unique IDs
        transactions.push(create_test_transaction(id));
    }
    transactions
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

    #[test]
    fn test_test_database_wrapper() {
        let test_db = TestDatabase::new("wrapper_test");

        // Should be able to use it like a normal database
        let _ = test_db.open_tree("test_tree").unwrap();

        // Cleanup will happen automatically when test_db is dropped
    }

    #[test]
    fn test_robust_cleanup() {
        let test_path = format!("test_db_cleanup_test_{}", std::process::id());

        // Create a test database
        {
            let _db = sled::open(&test_path).unwrap();
            // Database will be closed when dropped
        }

        // Verify it exists
        assert!(Path::new(&test_path).exists());

        // Test robust cleanup
        robust_cleanup_test_db(&test_path);

        // Should be gone (may take a moment due to OS file system delays)
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert!(
            !Path::new(&test_path).exists() || {
                // Give it one more try if it still exists
                robust_cleanup_test_db(&test_path);
                std::thread::sleep(std::time::Duration::from_millis(100));
                !Path::new(&test_path).exists()
            }
        );
    }

    #[test]
    fn test_cleanup_script() {
        // Create a few test database directories
        let test_paths = [
            format!("test_db_demo_1_{}", std::process::id()),
            format!("test_db_demo_2_{}", std::process::id()),
            format!("test_db_demo_3_{}", std::process::id()),
        ];

        // Create test databases
        let mut dbs = Vec::new();
        for path in &test_paths {
            let db = sled::open(path).unwrap();
            dbs.push(db);
        }

        // Verify they exist
        for path in &test_paths {
            assert!(Path::new(path).exists(), "Database {path} should exist");
        }

        // Drop databases to close them
        drop(dbs);
        std::thread::sleep(std::time::Duration::from_millis(50));

        // Run cleanup script
        let cleaned_count = cleanup_script();

        // Should have cleaned up our test databases
        assert!(
            cleaned_count >= 3,
            "Should have cleaned at least 3 databases, got {cleaned_count}"
        );

        // Verify they're gone
        for path in &test_paths {
            assert!(
                !Path::new(path).exists(),
                "Database {path} should be cleaned up"
            );
        }
    }
}
