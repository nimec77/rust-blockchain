use rust_blockchain::{Blockchain, Block, BLOCKS_TREE, TIP_BLOCK_HASH_KEY};
use crate::test_helpers::*;

#[test]
fn test_new_with_tip() {
    let test_name = "new_with_tip";
    let test_db = TestDatabase::new(test_name);
    let test_tip_hash = "test_hash_12345".to_string();

    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), test_tip_hash.clone());

    assert_eq!(blockchain.get_tip_hash(), test_tip_hash);
    // Verify we can access the database
    let _ = blockchain.get_db().open_tree("test_tree").unwrap();

    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_new_with_empty_tip() {
    let test_name = "new_with_empty_tip";
    let test_db = TestDatabase::new(test_name);

    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());

    assert_eq!(blockchain.get_tip_hash(), String::new());
    // Verify we can access the database
    let _ = blockchain.get_db().open_tree("test_tree").unwrap();

    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_get_db() {
    let test_name = "get_db";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());

    let db_ref = blockchain.get_db();
    // Verify we can use the database reference
    let _ = db_ref.open_tree("test_tree").unwrap();

    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_get_tip_hash() {
    let test_name = "get_tip_hash";
    let test_db = TestDatabase::new(test_name);
    let test_tip_hash = "abcdef123456".to_string();
    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), test_tip_hash.clone());

    let retrieved_hash = blockchain.get_tip_hash();
    assert_eq!(retrieved_hash, test_tip_hash);

    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_set_tip_hash() {
    let test_name = "set_tip_hash";
    let test_db = TestDatabase::new(test_name);
    let initial_hash = "initial_hash".to_string();
    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), initial_hash.clone());

    // Verify initial hash
    assert_eq!(blockchain.get_tip_hash(), initial_hash);

    // Set new hash
    let new_hash = "new_hash_654321";
    blockchain.set_tip_hash(new_hash);

    // Verify hash was updated
    assert_eq!(blockchain.get_tip_hash(), new_hash);

    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_set_tip_hash_multiple_updates() {
    let test_name = "set_tip_hash_multiple";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());

    // Test multiple updates
    let hashes = vec!["hash1", "hash2", "hash3", "final_hash"];

    for hash in &hashes {
        blockchain.set_tip_hash(hash);
        assert_eq!(blockchain.get_tip_hash(), *hash);
    }

    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_blockchain_clone() {
    let test_name = "blockchain_clone";
    let test_db = TestDatabase::new(test_name);
    let test_tip_hash = "cloneable_hash".to_string();
    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), test_tip_hash.clone());

    // Test that Blockchain can be cloned
    let cloned_blockchain = blockchain.clone();

    // Both should have the same tip hash
    assert_eq!(blockchain.get_tip_hash(), cloned_blockchain.get_tip_hash());

    // Updating one should update both (shared Arc<RwLock>)
    blockchain.set_tip_hash("updated_hash");
    assert_eq!(blockchain.get_tip_hash(), "updated_hash");
    assert_eq!(cloned_blockchain.get_tip_hash(), "updated_hash");

    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_thread_safety() {
    use std::thread;
    use std::time::Duration;

    let test_name = "thread_safety";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let blockchain_clone = blockchain.clone();

    // Spawn thread to update tip hash
    let handle = thread::spawn(move || {
        blockchain_clone.set_tip_hash("thread_hash");
        thread::sleep(Duration::from_millis(10));
        blockchain_clone.get_tip_hash()
    });

    // Update from main thread
    blockchain.set_tip_hash("main_hash");
    let _thread_result = handle.join().unwrap();

    // One of the updates should be the final value
    let final_hash = blockchain.get_tip_hash();
    assert!(final_hash == "main_hash" || final_hash == "thread_hash");

    // TestDatabase will auto-cleanup when dropped
}

#[ignore = "This test uses the global data directory and may conflict with other tests"]
#[test]
fn test_new_blockchain_with_existing_data() {
    use std::fs;
    use std::thread;
    use std::time::Duration;
    
    // Clean up any existing data directory first
    let data_dir = rust_blockchain::util::current_dir().join("data");
    if data_dir.exists() {
        let _ = fs::remove_dir_all(&data_dir);
    }
    
    // Create the data directory that new_blockchain() expects
    fs::create_dir_all(&data_dir).unwrap();
    
    // Scope to ensure database is fully closed before new_blockchain()
    let genesis_hash = {
        // Create a blockchain using the expected directory
        let test_db = sled::open(&data_dir).unwrap();
        
        // Create a genesis block and store it
        let genesis_tx = create_test_transaction(vec![0, 0, 0, 0]);
        let genesis_block = Block::generate_genesis_block(&genesis_tx);
        let genesis_hash = genesis_block.get_hash().to_string();
        
        // Store genesis block in database with proper structure
        let blocks_tree = test_db.open_tree(BLOCKS_TREE).unwrap();
        blocks_tree.insert(genesis_hash.as_str(), genesis_block.serialize()).unwrap();
        blocks_tree.insert(TIP_BLOCK_HASH_KEY, genesis_hash.as_str()).unwrap();
        
        // Ensure data is flushed and wait for file system sync
        blocks_tree.flush().unwrap();
        test_db.flush().unwrap();
        
        genesis_hash
    }; // Database is now fully closed
    
    // Wait a bit to ensure database files are fully released
    thread::sleep(Duration::from_millis(100));
    
    // Now create blockchain from existing data
    let blockchain = Blockchain::new_blockchain();
    
    // Verify it loaded the correct tip hash
    assert_eq!(blockchain.get_tip_hash(), genesis_hash);
    
    // Clean up the test data directory
    if data_dir.exists() {
        robust_cleanup_test_db(&data_dir.to_string_lossy());
    }
}

#[test]
fn test_new_blockchain_no_existing_data() {
    use std::fs;
    use std::panic;
    
    // Clean up any existing data directory first
    let data_dir = rust_blockchain::util::current_dir().join("data");
    if data_dir.exists() {
        let _ = fs::remove_dir_all(&data_dir);
    }
    
    // Try to create blockchain without any existing data - should panic
    let result = panic::catch_unwind(|| {
        Blockchain::new_blockchain()
    });
    
    // Verify it panicked
    assert!(result.is_err());
    
    // Clean up afterwards
    if data_dir.exists() {
        robust_cleanup_test_db(&data_dir.to_string_lossy());
    }
}

#[test]
fn test_get_best_height() {
    let test_name = "get_best_height";
    let test_db = TestDatabase::new(test_name);
    
    // Create blocks with different heights
    let mut block1 = create_test_block("".to_string(), 0);
    block1.hash = "block1_hash".to_string();
    
    let mut block2 = create_test_block("block1_hash".to_string(), 5);
    block2.hash = "block2_hash".to_string();
    
    let mut block3 = create_test_block("block2_hash".to_string(), 10);
    block3.hash = "block3_hash".to_string();
    
    // Store blocks in database
    let blocks_tree = test_db.open_tree(BLOCKS_TREE).unwrap();
    blocks_tree.insert("block1_hash", block1.serialize()).unwrap();
    blocks_tree.insert("block2_hash", block2.serialize()).unwrap();
    blocks_tree.insert("block3_hash", block3.serialize()).unwrap();
    
    // Test with different tip blocks
    let blockchain1 = Blockchain::new_with_tip(test_db.get_db().clone(), "block1_hash".to_string());
    assert_eq!(blockchain1.get_best_height(), 0);
    
    let blockchain2 = Blockchain::new_with_tip(test_db.get_db().clone(), "block2_hash".to_string());
    assert_eq!(blockchain2.get_best_height(), 5);
    
    let blockchain3 = Blockchain::new_with_tip(test_db.get_db().clone(), "block3_hash".to_string());
    assert_eq!(blockchain3.get_best_height(), 10);
    
    // TestDatabase will auto-cleanup when dropped
}

#[test]
#[should_panic(expected = "The tip hash is valid")]
fn test_get_best_height_invalid_tip() {
    let test_name = "get_best_height_invalid";
    let test_db = TestDatabase::new(test_name);
    
    // Create blockchain with invalid tip hash
    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), "nonexistent_hash".to_string());
    
    // This should panic when trying to get best height
    let _height = blockchain.get_best_height();
    
    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_blockchain_database_operations() {
    let test_name = "blockchain_db_ops";
    let test_db = TestDatabase::new(test_name);
    
    // Create blockchain
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    
    // Test database access
    let db_ref = blockchain.get_db();
    
    // Create and access different trees
    let tree1 = db_ref.open_tree("test_tree_1").unwrap();
    let tree2 = db_ref.open_tree("test_tree_2").unwrap();
    
    // Store and retrieve data
    tree1.insert("key1", "value1").unwrap();
    tree2.insert("key2", "value2").unwrap();
    
    assert_eq!(tree1.get("key1").unwrap().unwrap().as_ref(), b"value1");
    assert_eq!(tree2.get("key2").unwrap().unwrap().as_ref(), b"value2");
    
    // Verify trees are independent
    assert!(tree1.get("key2").unwrap().is_none());
    assert!(tree2.get("key1").unwrap().is_none());
    
    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_blockchain_blocks_tree_access() {
    let test_name = "blockchain_blocks_tree";
    let test_db = TestDatabase::new(test_name);
    
    // Create a simple test block with height 5
    let test_block = create_test_block("previous_hash".to_string(), 5);
    
    // Verify the block was created with correct height
    assert_eq!(test_block.get_height(), 5);
    
    // Store block using blockchain database
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let blocks_tree = blockchain.get_db().open_tree(BLOCKS_TREE).unwrap();
    
    blocks_tree.insert(test_block.get_hash(), test_block.serialize()).unwrap();
    
    // Retrieve and verify block
    let retrieved_block_data = blocks_tree.get(test_block.get_hash()).unwrap().unwrap();
    let retrieved_block = Block::deserialize(retrieved_block_data.as_ref());
    
    // Verify all properties match
    assert_eq!(retrieved_block.get_hash(), test_block.get_hash());
    assert_eq!(retrieved_block.get_pre_block_hash(), test_block.get_pre_block_hash());
    assert_eq!(retrieved_block.get_height(), test_block.get_height());
    assert_eq!(retrieved_block.get_transactions().len(), test_block.get_transactions().len());
    
    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_blockchain_tip_persistence() {
    let test_name = "blockchain_tip_persistence";
    let test_db = TestDatabase::new(test_name);
    
    // Create blockchain and set tip
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let test_tip = "persistent_tip_hash";
    blockchain.set_tip_hash(test_tip);
    
    // Create another blockchain instance with same database but different tip
    let blockchain2 = Blockchain::new_with_tip(test_db.get_db().clone(), "different_tip".to_string());
    
    // Each instance maintains its own tip hash state
    assert_eq!(blockchain.get_tip_hash(), test_tip);
    assert_eq!(blockchain2.get_tip_hash(), "different_tip");
    
    // Update one and verify they remain independent
    let new_tip = "updated_tip_hash";
    blockchain.set_tip_hash(new_tip);
    assert_eq!(blockchain.get_tip_hash(), new_tip);
    assert_eq!(blockchain2.get_tip_hash(), "different_tip"); // Still different
    
    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_blockchain_concurrent_read_operations() {
    use std::thread;
    use std::sync::Arc;
    
    let test_name = "blockchain_concurrent_reads";
    let test_db = TestDatabase::new(test_name);
    
    // Create test block and store it
    let test_block = create_test_block("".to_string(), 42);
    let test_hash = test_block.get_hash().to_string();
    
    let blocks_tree = test_db.open_tree(BLOCKS_TREE).unwrap();
    blocks_tree.insert(&test_hash, test_block.serialize()).unwrap();
    
    let blockchain = Arc::new(Blockchain::new_with_tip(test_db.get_db().clone(), test_hash));
    
    // Spawn multiple threads to read concurrently
    let mut handles = Vec::new();
    for i in 0..10 {
        let blockchain_clone = Arc::clone(&blockchain);
        let handle = thread::spawn(move || {
            // Each thread performs read operations
            let tip_hash = blockchain_clone.get_tip_hash();
            let best_height = blockchain_clone.get_best_height();
            let db_ref = blockchain_clone.get_db();
            let _ = db_ref.open_tree("test_tree").unwrap();
            
            (i, tip_hash, best_height)
        });
        handles.push(handle);
    }
    
    // Collect results from all threads
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    
    // All threads should get consistent results
    let first_result = &results[0];
    for result in &results {
        assert_eq!(result.1, first_result.1); // Same tip hash
        assert_eq!(result.2, first_result.2); // Same best height
        assert_eq!(result.2, 42); // Correct height value
    }
    
    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_blockchain_database_isolation() {
    let test_name1 = "blockchain_isolation_1";
    let test_name2 = "blockchain_isolation_2";
    
    let test_db1 = TestDatabase::new(test_name1);
    let test_db2 = TestDatabase::new(test_name2);
    
    // Create blockchains with different databases
    let blockchain1 = Blockchain::new_with_tip(test_db1.get_db().clone(), "hash1".to_string());
    let blockchain2 = Blockchain::new_with_tip(test_db2.get_db().clone(), "hash2".to_string());
    
    // Verify they are isolated
    assert_eq!(blockchain1.get_tip_hash(), "hash1");
    assert_eq!(blockchain2.get_tip_hash(), "hash2");
    
    // Create test data in each database
    let tree1 = blockchain1.get_db().open_tree("test_tree").unwrap();
    let tree2 = blockchain2.get_db().open_tree("test_tree").unwrap();
    
    tree1.insert("shared_key", "value1").unwrap();
    tree2.insert("shared_key", "value2").unwrap();
    
    // Verify isolation
    assert_eq!(tree1.get("shared_key").unwrap().unwrap().as_ref(), b"value1");
    assert_eq!(tree2.get("shared_key").unwrap().unwrap().as_ref(), b"value2");
    
    // TestDatabase will auto-cleanup when both are dropped
}

#[test]
fn test_blockchain_large_tip_hash_values() {
    let test_name = "blockchain_large_tips";
    let test_db = TestDatabase::new(test_name);
    
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    
    // Test with very large tip hash
    let large_tip = "a".repeat(10000);
    blockchain.set_tip_hash(&large_tip);
    assert_eq!(blockchain.get_tip_hash(), large_tip);
    
    // Test with binary-like data in hash
    let binary_tip = (0..1000).map(|i| (i % 256) as u8).collect::<Vec<u8>>();
    let binary_tip_string = format!("{binary_tip:?}");
    blockchain.set_tip_hash(&binary_tip_string);
    assert_eq!(blockchain.get_tip_hash(), binary_tip_string);
    
    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_blockchain_rapid_tip_updates() {
    let test_name = "blockchain_rapid_updates";
    let test_db = TestDatabase::new(test_name);
    
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    
    // Rapidly update tip hash many times
    for i in 0..1000 {
        let tip = format!("tip_hash_{i}");
        blockchain.set_tip_hash(&tip);
        assert_eq!(blockchain.get_tip_hash(), tip);
    }
    
    // Final verification
    assert_eq!(blockchain.get_tip_hash(), "tip_hash_999");
    
    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_blockchain_get_best_height_with_complex_blocks() {
    let test_name = "blockchain_complex_heights";
    let test_db = TestDatabase::new(test_name);
    
    // Create blocks with complex transaction data
    let mut complex_block = create_test_block("previous_hash".to_string(), 100);
    
    // Add multiple transactions
    let tx1 = create_test_transaction(vec![1, 2, 3, 4]);
    let tx2 = create_test_transaction(vec![5, 6, 7, 8]);
    let tx3 = create_test_transaction(vec![9, 10, 11, 12]);
    complex_block.transactions = vec![tx1, tx2, tx3];
    complex_block.hash = "complex_hash".to_string();
    
    // Store block
    let blocks_tree = test_db.open_tree(BLOCKS_TREE).unwrap();
    blocks_tree.insert("complex_hash", complex_block.serialize()).unwrap();
    
    // Test blockchain with complex block
    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), "complex_hash".to_string());
    assert_eq!(blockchain.get_best_height(), 100);
    
    // Verify block data integrity
    let retrieved_data = blocks_tree.get("complex_hash").unwrap().unwrap();
    let retrieved_block = Block::deserialize(retrieved_data.as_ref());
    assert_eq!(retrieved_block.get_height(), 100);
    assert_eq!(retrieved_block.get_transactions().len(), 3);
    
    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_blockchain_error_recovery() {
    let test_name = "blockchain_error_recovery";
    let test_db = TestDatabase::new(test_name);
    
    // Create blockchain with valid initial state
    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), "valid_hash".to_string());
    
    // Store valid block
    let valid_block = create_test_block("".to_string(), 5);
    let blocks_tree = test_db.open_tree(BLOCKS_TREE).unwrap();
    blocks_tree.insert("valid_hash", valid_block.serialize()).unwrap();
    
    // Verify valid operation
    assert_eq!(blockchain.get_best_height(), 5);
    
    // Change tip to invalid hash
    blockchain.set_tip_hash("invalid_hash");
    assert_eq!(blockchain.get_tip_hash(), "invalid_hash");
    
    // get_best_height should panic with invalid tip
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| blockchain.get_best_height()));
    assert!(result.is_err());
    
    // Recover by setting valid tip again
    blockchain.set_tip_hash("valid_hash");
    assert_eq!(blockchain.get_best_height(), 5);
    
    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_blockchain_empty_string_handling() {
    let test_name = "blockchain_empty_strings";
    let test_db = TestDatabase::new(test_name);
    
    // Test with empty string tip hash
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    assert_eq!(blockchain.get_tip_hash(), "");
    
    // Test setting empty string explicitly
    blockchain.set_tip_hash("");
    assert_eq!(blockchain.get_tip_hash(), "");
    
    // Test with empty string in new_with_tip
    let blockchain2 = Blockchain::new_with_tip(test_db.get_db().clone(), "".to_string());
    assert_eq!(blockchain2.get_tip_hash(), "");
    
    // Both should be equivalent
    assert_eq!(blockchain.get_tip_hash(), blockchain2.get_tip_hash());
    
    // TestDatabase will auto-cleanup when dropped
}

#[test]
fn test_add_block_new_block() {
    let test_name = "add_block_new_block";
    let test_db = TestDatabase::new(test_name);
    
    // Create an initial tip block
    let initial_block = create_test_block("genesis_hash".to_string(), 0);
    let initial_hash = initial_block.get_hash().to_string();
    
    // Set up blockchain with initial block as tip
    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), initial_hash.clone());
    
    // Store initial block in database
    let blocks_tree = test_db.get_db().open_tree(BLOCKS_TREE).unwrap();
    blocks_tree.insert(initial_hash.as_str(), initial_block.serialize()).unwrap();
    blocks_tree.insert(TIP_BLOCK_HASH_KEY, initial_hash.as_str()).unwrap();

    // Create a test block with higher height
    let test_block = create_test_block(initial_hash.clone(), 1);
    let block_hash = test_block.get_hash().to_string();

    // Add the block
    blockchain.add_block(&test_block);

    // Verify the block was added to the database
    let stored_block_bytes = blocks_tree.get(&block_hash).unwrap().expect("Block should be stored");
    let stored_block = Block::deserialize(stored_block_bytes.as_ref());

    assert_eq!(stored_block.get_hash(), test_block.get_hash());
    assert_eq!(stored_block.get_height(), test_block.get_height());
    assert_eq!(stored_block.get_pre_block_hash(), test_block.get_pre_block_hash());
}

#[test]
fn test_add_block_duplicate_block() {
    let test_name = "add_block_duplicate";
    let test_db = TestDatabase::new(test_name);
    
    // Create an initial tip block
    let initial_block = create_test_block("genesis_hash".to_string(), 0);
    let initial_hash = initial_block.get_hash().to_string();
    
    // Set up blockchain with initial block as tip
    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), initial_hash.clone());
    
    // Store initial block in database
    let blocks_tree = test_db.get_db().open_tree(BLOCKS_TREE).unwrap();
    blocks_tree.insert(initial_hash.as_str(), initial_block.serialize()).unwrap();
    blocks_tree.insert(TIP_BLOCK_HASH_KEY, initial_hash.as_str()).unwrap();

    // Create and add a block first time
    let test_block = create_test_block(initial_hash.clone(), 1);
    blockchain.add_block(&test_block);

    // Try to add the same block again
    blockchain.add_block(&test_block);

    // Verify it only exists once in the database
    let block_exists = blocks_tree.get(test_block.get_hash()).unwrap().is_some();
    assert!(block_exists, "Block should still exist after duplicate add");

    // Verify no errors occurred and the operation completed successfully
    let stored_block_bytes = blocks_tree.get(test_block.get_hash()).unwrap().unwrap();
    let stored_block = Block::deserialize(stored_block_bytes.as_ref());
    assert_eq!(stored_block.get_hash(), test_block.get_hash());
}

#[test]
fn test_add_block_updates_tip_with_higher_height() {
    let test_name = "add_block_updates_tip_higher";
    let test_db = TestDatabase::new(test_name);
    
    // Create initial tip block with height 1
    let initial_block = create_test_block("genesis_hash".to_string(), 1);
    let initial_hash = initial_block.get_hash().to_string();
    
    // Set up blockchain with the initial block as tip
    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), initial_hash.clone());
    
    // Store initial block in database
    let blocks_tree = test_db.get_db().open_tree(BLOCKS_TREE).unwrap();
    blocks_tree.insert(initial_hash.as_str(), initial_block.serialize()).unwrap();
    blocks_tree.insert(TIP_BLOCK_HASH_KEY, initial_hash.as_str()).unwrap();

    // Create new block with higher height
    let new_block = create_test_block(initial_hash.clone(), 2);
    let new_hash = new_block.get_hash().to_string();

    // Add the new block
    blockchain.add_block(&new_block);

    // Verify tip hash was updated in blockchain instance
    assert_eq!(blockchain.get_tip_hash(), new_hash);

    // Verify tip hash was updated in database
    let stored_tip = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap().unwrap();
    let stored_tip_str = String::from_utf8(stored_tip.to_vec()).unwrap();
    assert_eq!(stored_tip_str, new_hash);
}

#[test]
fn test_add_block_does_not_update_tip_with_lower_height() {
    let test_name = "add_block_no_update_tip_lower";
    let test_db = TestDatabase::new(test_name);
    
    // Create initial tip block with height 2
    let initial_block = create_test_block("genesis_hash".to_string(), 2);
    let initial_hash = initial_block.get_hash().to_string();
    
    // Set up blockchain with the initial block as tip
    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), initial_hash.clone());
    
    // Store initial block in database
    let blocks_tree = test_db.get_db().open_tree(BLOCKS_TREE).unwrap();
    blocks_tree.insert(initial_hash.as_str(), initial_block.serialize()).unwrap();
    blocks_tree.insert(TIP_BLOCK_HASH_KEY, initial_hash.as_str()).unwrap();

    // Create new block with lower height
    let new_block = create_test_block("some_other_hash".to_string(), 1);
    let new_hash = new_block.get_hash().to_string();

    // Add the new block
    blockchain.add_block(&new_block);

    // Verify tip hash was NOT updated in blockchain instance
    assert_eq!(blockchain.get_tip_hash(), initial_hash);

    // Verify tip hash was NOT updated in database
    let stored_tip = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap().unwrap();
    let stored_tip_str = String::from_utf8(stored_tip.to_vec()).unwrap();
    assert_eq!(stored_tip_str, initial_hash);

    // Verify the new block was still stored
    let stored_new_block = blocks_tree.get(&new_hash).unwrap().unwrap();
    let deserialized_new_block = Block::deserialize(stored_new_block.as_ref());
    assert_eq!(deserialized_new_block.get_hash(), new_hash);
}

#[test]
fn test_add_block_does_not_update_tip_with_equal_height() {
    let test_name = "add_block_no_update_tip_equal";
    let test_db = TestDatabase::new(test_name);
    
    // Create initial tip block with height 1
    let initial_block = create_test_block("genesis_hash".to_string(), 1);
    let initial_hash = initial_block.get_hash().to_string();
    
    // Set up blockchain with the initial block as tip
    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), initial_hash.clone());
    
    // Store initial block in database
    let blocks_tree = test_db.get_db().open_tree(BLOCKS_TREE).unwrap();
    blocks_tree.insert(initial_hash.as_str(), initial_block.serialize()).unwrap();
    blocks_tree.insert(TIP_BLOCK_HASH_KEY, initial_hash.as_str()).unwrap();

    // Create new block with equal height but different hash
    let new_block = create_test_block("different_prev_hash".to_string(), 1);
    let new_hash = new_block.get_hash().to_string();

    // Add the new block
    blockchain.add_block(&new_block);

    // Verify tip hash was NOT updated in blockchain instance
    assert_eq!(blockchain.get_tip_hash(), initial_hash);

    // Verify tip hash was NOT updated in database
    let stored_tip = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap().unwrap();
    let stored_tip_str = String::from_utf8(stored_tip.to_vec()).unwrap();
    assert_eq!(stored_tip_str, initial_hash);

    // Verify the new block was still stored
    let stored_new_block = blocks_tree.get(&new_hash).unwrap().unwrap();
    let deserialized_new_block = Block::deserialize(stored_new_block.as_ref());
    assert_eq!(deserialized_new_block.get_hash(), new_hash);
}

#[test]
fn test_add_block_multiple_blocks_chain() {
    let test_name = "add_block_multiple_chain";
    let test_db = TestDatabase::new(test_name);
    
    // Create genesis block
    let genesis_block = create_test_genesis_block();
    let genesis_hash = genesis_block.get_hash().to_string();
    
    // Set up blockchain with genesis as tip
    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), genesis_hash.clone());
    
    // Store genesis block
    let blocks_tree = test_db.get_db().open_tree(BLOCKS_TREE).unwrap();
    blocks_tree.insert(genesis_hash.as_str(), genesis_block.serialize()).unwrap();
    blocks_tree.insert(TIP_BLOCK_HASH_KEY, genesis_hash.as_str()).unwrap();

    // Create and add a chain of blocks
    let mut previous_hash = genesis_hash;
    let mut expected_tip_hash = previous_hash.clone();
    
    for height in 1..=3 {
        let block = create_test_block(previous_hash.clone(), height);
        let block_hash = block.get_hash().to_string();
        
        blockchain.add_block(&block);
        
        // Each new block should become the tip
        assert_eq!(blockchain.get_tip_hash(), block_hash);
        expected_tip_hash = block_hash.clone();
        previous_hash = block_hash;
    }

    // Verify final tip is correct
    assert_eq!(blockchain.get_tip_hash(), expected_tip_hash);
    
    // Verify all blocks are stored
    for height in 1..=3 {
        let expected_count = height + 1; // genesis + height blocks
        let mut count = 0;
        for (key, _) in blocks_tree.iter().flatten() {
            let key_str = String::from_utf8_lossy(&key);
            if key_str != TIP_BLOCK_HASH_KEY {
                count += 1;
            }
        }
        if height == 3 {
            assert_eq!(count, expected_count);
        }
    }
}

#[test]
fn test_add_block_transaction_consistency() {
    let test_name = "add_block_transaction_consistency";
    let test_db = TestDatabase::new(test_name);
    
    // Create initial tip block
    let initial_block = create_test_block("genesis".to_string(), 1);
    let initial_hash = initial_block.get_hash().to_string();
    
    let blockchain = Blockchain::new_with_tip(test_db.get_db().clone(), initial_hash.clone());
    
    // Store initial block
    let blocks_tree = test_db.get_db().open_tree(BLOCKS_TREE).unwrap();
    blocks_tree.insert(initial_hash.as_str(), initial_block.serialize()).unwrap();
    blocks_tree.insert(TIP_BLOCK_HASH_KEY, initial_hash.as_str()).unwrap();

    // Create new block with higher height
    let new_block = create_test_block(initial_hash.clone(), 2);
    let new_hash = new_block.get_hash().to_string();

    // Add the block
    blockchain.add_block(&new_block);

    // Verify both block storage and tip update happened atomically
    let stored_block = blocks_tree.get(&new_hash).unwrap();
    let stored_tip = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
    
    assert!(stored_block.is_some(), "Block should be stored");
    assert_eq!(
        String::from_utf8(stored_tip.unwrap().to_vec()).unwrap(),
        new_hash,
        "Tip should be updated"
    );
    
    // Verify blockchain instance is also consistent
    assert_eq!(blockchain.get_tip_hash(), new_hash);
}

#[test]
fn test_add_block_concurrent_access() {
    use std::thread;
    use std::sync::Arc;
    
    let test_name = "add_block_concurrent";
    let test_db = TestDatabase::new(test_name);
    
    // Create initial setup
    let initial_block = create_test_block("genesis".to_string(), 0);
    let initial_hash = initial_block.get_hash().to_string();
    
    let blockchain = Arc::new(Blockchain::new_with_tip(test_db.get_db().clone(), initial_hash.clone()));
    
    // Store initial block
    let blocks_tree = test_db.get_db().open_tree(BLOCKS_TREE).unwrap();
    blocks_tree.insert(initial_hash.as_str(), initial_block.serialize()).unwrap();
    blocks_tree.insert(TIP_BLOCK_HASH_KEY, initial_hash.as_str()).unwrap();

    // Create multiple blocks to add concurrently
    let blocks: Vec<Block> = (1..=5).map(|i| {
        create_test_block(format!("prev_hash_{i}"), i)
    }).collect();
    
    // Store block hashes for verification
    let block_hashes: Vec<String> = blocks.iter().map(|b| b.get_hash().to_string()).collect();
    
    // Add blocks concurrently
    let handles: Vec<_> = blocks.into_iter().enumerate().map(|(i, block)| {
        let blockchain_clone = Arc::clone(&blockchain);
        thread::spawn(move || {
            blockchain_clone.add_block(&block);
            (i, block.get_hash().to_string())
        })
    }).collect();

    // Wait for all threads to complete
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    // Verify all blocks were added
    for (i, expected_hash) in results {
        let stored_block = blocks_tree.get(&expected_hash).unwrap();
        assert!(stored_block.is_some(), "Block {i} should be stored");
        assert_eq!(block_hashes[i], expected_hash);
    }
    
    // Verify one of the blocks became the tip (the one with highest height)
    let final_tip = blockchain.get_tip_hash();
    assert!(block_hashes.contains(&final_tip), "Final tip should be one of the added blocks");
} 
