use crate::test_helpers::{
    create_coinbase_transaction, create_multiple_test_transactions, create_spending_transaction,
    create_test_transaction,
};
use data_encoding::HEXLOWER;
use rust_blockchain::{MemoryPool, TXInput};
use std::sync::Arc;
use std::thread;

// =============================================================================
// MEMORY POOL CONSTRUCTOR TESTS
// =============================================================================

#[test]
fn test_memory_pool_new() {
    let pool = MemoryPool::new();
    assert!(pool.is_empty());
    assert_eq!(pool.len(), 0);
}

#[test]
fn test_memory_pool_default() {
    let pool = MemoryPool::default();
    assert!(pool.is_empty());
    assert_eq!(pool.len(), 0);
}

// =============================================================================
// MEMORY POOL ADD TESTS
// =============================================================================

#[test]
fn test_memory_pool_add_single_transaction() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);

    pool.add(tx.clone());

    assert!(!pool.is_empty());
    assert_eq!(pool.len(), 1);

    let tx_hex = HEXLOWER.encode(tx.get_id());
    assert!(pool.contains(&tx_hex));
}

#[test]
fn test_memory_pool_add_multiple_transactions() {
    let pool = MemoryPool::new();
    let transactions = create_multiple_test_transactions(3);

    for tx in &transactions {
        pool.add(tx.clone());
    }

    assert!(!pool.is_empty());
    assert_eq!(pool.len(), 3);

    // Verify all transactions are present
    for tx in &transactions {
        let tx_hex = HEXLOWER.encode(tx.get_id());
        assert!(pool.contains(&tx_hex));
    }
}

#[test]
fn test_memory_pool_add_duplicate_transaction() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);

    pool.add(tx.clone());
    pool.add(tx.clone()); // Add the same transaction again

    // Should still only have one transaction (overwritten)
    assert_eq!(pool.len(), 1);

    let tx_hex = HEXLOWER.encode(tx.get_id());
    assert!(pool.contains(&tx_hex));
}

#[test]
fn test_memory_pool_add_coinbase_transaction() {
    let pool = MemoryPool::new();
    let coinbase_tx = create_coinbase_transaction(50, vec![1, 2, 3, 4, 5]);

    pool.add(coinbase_tx.clone());

    assert_eq!(pool.len(), 1);
    let tx_hex = HEXLOWER.encode(coinbase_tx.get_id());
    assert!(pool.contains(&tx_hex));
}

#[test]
fn test_memory_pool_add_spending_transaction() {
    let pool = MemoryPool::new();
    let spending_tx =
        create_spending_transaction(vec![(vec![1, 2, 3], 0)], vec![(100, vec![4, 5, 6])]);

    pool.add(spending_tx.clone());

    assert_eq!(pool.len(), 1);
    let tx_hex = HEXLOWER.encode(spending_tx.get_id());
    assert!(pool.contains(&tx_hex));
}

// =============================================================================
// MEMORY POOL CONTAINS TESTS
// =============================================================================

#[test]
fn test_memory_pool_contains_existing_transaction() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);
    let tx_hex = HEXLOWER.encode(tx.get_id());

    pool.add(tx);

    assert!(pool.contains(&tx_hex));
}

#[test]
fn test_memory_pool_contains_non_existing_transaction() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);
    let tx_hex = HEXLOWER.encode(tx.get_id());

    // Don't add the transaction
    assert!(!pool.contains(&tx_hex));
}

#[test]
fn test_memory_pool_contains_empty_pool() {
    let pool = MemoryPool::new();
    assert!(!pool.contains("any_hex_id"));
}

#[test]
fn test_memory_pool_contains_invalid_hex() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);
    pool.add(tx);

    // Test with invalid hex string
    assert!(!pool.contains("invalid_hex"));
    assert!(!pool.contains(""));
}

// =============================================================================
// MEMORY POOL GET TESTS
// =============================================================================

#[test]
fn test_memory_pool_get_existing_transaction() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);
    let tx_hex = HEXLOWER.encode(tx.get_id());

    pool.add(tx.clone());

    let retrieved_tx = pool.get(&tx_hex);
    assert!(retrieved_tx.is_some());
    assert_eq!(retrieved_tx.unwrap().get_id(), tx.get_id());
}

#[test]
fn test_memory_pool_get_non_existing_transaction() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);
    let tx_hex = HEXLOWER.encode(tx.get_id());

    // Don't add the transaction
    let retrieved_tx = pool.get(&tx_hex);
    assert!(retrieved_tx.is_none());
}

#[test]
fn test_memory_pool_get_empty_pool() {
    let pool = MemoryPool::new();
    let retrieved_tx = pool.get("any_hex_id");
    assert!(retrieved_tx.is_none());
}

#[test]
fn test_memory_pool_get_cloned_transaction() {
    let pool = MemoryPool::new();
    let mut tx = create_test_transaction(vec![1, 2, 3, 4]);
    let tx_hex = HEXLOWER.encode(tx.get_id());

    pool.add(tx.clone());

    // Modify original transaction
    tx.vin.push(TXInput::new(b"new_input", 0));

    // Retrieved transaction should be unchanged
    let retrieved_tx = pool.get(&tx_hex).unwrap();
    assert_eq!(retrieved_tx.vin.len(), 1); // Original length
    assert_ne!(retrieved_tx.vin.len(), tx.vin.len()); // Different from modified
}

// =============================================================================
// MEMORY POOL REMOVE TESTS
// =============================================================================

#[test]
fn test_memory_pool_remove_existing_transaction() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);
    let tx_hex = HEXLOWER.encode(tx.get_id());

    pool.add(tx);
    assert_eq!(pool.len(), 1);
    assert!(pool.contains(&tx_hex));

    pool.remove(&tx_hex);

    assert_eq!(pool.len(), 0);
    assert!(!pool.contains(&tx_hex));
    assert!(pool.is_empty());
}

#[test]
fn test_memory_pool_remove_non_existing_transaction() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);
    let tx_hex = HEXLOWER.encode(tx.get_id());

    // Don't add the transaction
    pool.remove(&tx_hex);

    // Should not panic or change anything
    assert_eq!(pool.len(), 0);
    assert!(pool.is_empty());
}

#[test]
fn test_memory_pool_remove_from_multiple_transactions() {
    let pool = MemoryPool::new();
    let transactions = create_multiple_test_transactions(3);

    // Add all transactions
    for tx in &transactions {
        pool.add(tx.clone());
    }
    assert_eq!(pool.len(), 3);

    // Remove one transaction
    let tx_to_remove = &transactions[1];
    let tx_hex = HEXLOWER.encode(tx_to_remove.get_id());
    pool.remove(&tx_hex);

    assert_eq!(pool.len(), 2);
    assert!(!pool.contains(&tx_hex));

    // Verify other transactions are still present
    for (i, tx) in transactions.iter().enumerate() {
        if i != 1 {
            let other_tx_hex = HEXLOWER.encode(tx.get_id());
            assert!(pool.contains(&other_tx_hex));
        }
    }
}

#[test]
fn test_memory_pool_remove_empty_pool() {
    let pool = MemoryPool::new();

    // Should not panic
    pool.remove("any_hex_id");
    pool.remove("");

    assert!(pool.is_empty());
}

// =============================================================================
// MEMORY POOL GET_ALL TESTS
// =============================================================================

#[test]
fn test_memory_pool_get_all_empty_pool() {
    let pool = MemoryPool::new();
    let all_transactions = pool.get_all();

    assert!(all_transactions.is_empty());
}

#[test]
fn test_memory_pool_get_all_single_transaction() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);

    pool.add(tx.clone());
    let all_transactions = pool.get_all();

    assert_eq!(all_transactions.len(), 1);
    assert_eq!(all_transactions[0].get_id(), tx.get_id());
}

#[test]
fn test_memory_pool_get_all_multiple_transactions() {
    let pool = MemoryPool::new();
    let transactions = create_multiple_test_transactions(5);

    for tx in &transactions {
        pool.add(tx.clone());
    }

    let all_transactions = pool.get_all();
    assert_eq!(all_transactions.len(), 5);

    // Verify all transactions are present (order might be different)
    for tx in &transactions {
        let found = all_transactions.iter().any(|t| t.get_id() == tx.get_id());
        assert!(found, "Transaction not found in get_all result");
    }
}

#[test]
fn test_memory_pool_get_all_returns_clones() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);

    pool.add(tx.clone());
    let mut all_transactions = pool.get_all();

    // Modify the returned transaction
    all_transactions[0].vin.push(TXInput::new(b"new_input", 0));

    // Original transaction in pool should be unchanged
    let original_tx = pool.get_all();
    assert_eq!(original_tx[0].vin.len(), 1); // Original length
    assert_ne!(original_tx[0].vin.len(), all_transactions[0].vin.len());
}

// =============================================================================
// MEMORY POOL LEN TESTS
// =============================================================================

#[test]
fn test_memory_pool_len_empty() {
    let pool = MemoryPool::new();
    assert_eq!(pool.len(), 0);
}

#[test]
fn test_memory_pool_len_single_transaction() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);

    pool.add(tx);
    assert_eq!(pool.len(), 1);
}

#[test]
fn test_memory_pool_len_multiple_transactions() {
    let pool = MemoryPool::new();
    let transactions = create_multiple_test_transactions(10);

    for (i, tx) in transactions.iter().enumerate() {
        pool.add(tx.clone());
        assert_eq!(pool.len(), i + 1);
    }
}

#[test]
fn test_memory_pool_len_after_remove() {
    let pool = MemoryPool::new();
    let transactions = create_multiple_test_transactions(3);

    // Add transactions
    for tx in &transactions {
        pool.add(tx.clone());
    }
    assert_eq!(pool.len(), 3);

    // Remove one transaction
    let tx_hex = HEXLOWER.encode(transactions[0].get_id());
    pool.remove(&tx_hex);
    assert_eq!(pool.len(), 2);

    // Remove another transaction
    let tx_hex = HEXLOWER.encode(transactions[1].get_id());
    pool.remove(&tx_hex);
    assert_eq!(pool.len(), 1);

    // Remove last transaction
    let tx_hex = HEXLOWER.encode(transactions[2].get_id());
    pool.remove(&tx_hex);
    assert_eq!(pool.len(), 0);
}

// =============================================================================
// MEMORY POOL IS_EMPTY TESTS
// =============================================================================

#[test]
fn test_memory_pool_is_empty_new_pool() {
    let pool = MemoryPool::new();
    assert!(pool.is_empty());
}

#[test]
fn test_memory_pool_is_empty_after_add() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);

    pool.add(tx);
    assert!(!pool.is_empty());
}

#[test]
fn test_memory_pool_is_empty_after_add_remove() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![1, 2, 3, 4]);
    let tx_hex = HEXLOWER.encode(tx.get_id());

    pool.add(tx);
    assert!(!pool.is_empty());

    pool.remove(&tx_hex);
    assert!(pool.is_empty());
}

#[test]
fn test_memory_pool_is_empty_consistency_with_len() {
    let pool = MemoryPool::new();
    let transactions = create_multiple_test_transactions(5);

    // Test consistency throughout operations
    let len = pool.len();
    assert_eq!(pool.is_empty(), len == 0);

    for tx in &transactions {
        pool.add(tx.clone());
        let len = pool.len();
        assert_eq!(pool.is_empty(), len == 0);
    }

    for tx in &transactions {
        let tx_hex = HEXLOWER.encode(tx.get_id());
        pool.remove(&tx_hex);
        let len = pool.len();
        assert_eq!(pool.is_empty(), len == 0);
    }
}

// =============================================================================
// MEMORY POOL CONCURRENT ACCESS TESTS
// =============================================================================

#[test]
fn test_memory_pool_concurrent_add() {
    let pool = Arc::new(MemoryPool::new());
    let mut handles = Vec::new();

    for i in 0..10 {
        let pool_clone = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            let tx = create_test_transaction(vec![i as u8; 4]);
            pool_clone.add(tx);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(pool.len(), 10);
}

#[test]
fn test_memory_pool_concurrent_read_write() {
    let pool = Arc::new(MemoryPool::new());
    let mut handles = Vec::new();

    // Add some initial transactions
    for i in 0..5 {
        let tx = create_test_transaction(vec![i as u8; 4]);
        pool.add(tx);
    }

    // Concurrent readers
    for i in 0..3 {
        let pool_clone = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            let tx = create_test_transaction(vec![i as u8; 4]);
            let tx_hex = HEXLOWER.encode(tx.get_id());

            // Read operations
            let _ = pool_clone.contains(&tx_hex);
            let _ = pool_clone.get(&tx_hex);
            let _ = pool_clone.len();
            let _ = pool_clone.is_empty();
            let _ = pool_clone.get_all();
        });
        handles.push(handle);
    }

    // Concurrent writers
    for i in 10..13 {
        let pool_clone = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            let tx = create_test_transaction(vec![i as u8; 4]);
            pool_clone.add(tx);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Should have initial 5 + 3 new transactions
    assert_eq!(pool.len(), 8);
}

#[test]
fn test_memory_pool_concurrent_add_remove() {
    let pool = Arc::new(MemoryPool::new());
    let mut handles = Vec::new();

    // Add initial transactions
    let initial_transactions = create_multiple_test_transactions(10);
    for tx in &initial_transactions {
        pool.add(tx.clone());
    }

    // Concurrent removers
    for tx in initial_transactions.iter().take(5) {
        let pool_clone = Arc::clone(&pool);
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            let tx_hex = HEXLOWER.encode(tx.get_id());
            pool_clone.remove(&tx_hex);
        });
        handles.push(handle);
    }

    // Concurrent adders
    for i in 20..25 {
        let pool_clone = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            let tx = create_test_transaction(vec![i as u8; 4]);
            pool_clone.add(tx);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Should have 10 - 5 + 5 = 10 transactions
    assert_eq!(pool.len(), 10);
}

// =============================================================================
// MEMORY POOL EDGE CASE TESTS
// =============================================================================

#[test]
fn test_memory_pool_large_transaction_id() {
    let pool = MemoryPool::new();
    let large_id = vec![255u8; 32]; // 32 bytes of max value
    let tx = create_test_transaction(large_id.clone());
    let tx_hex = HEXLOWER.encode(&large_id);

    pool.add(tx.clone());

    assert!(pool.contains(&tx_hex));
    assert_eq!(pool.get(&tx_hex).unwrap().get_id(), large_id.as_slice());
}

#[test]
fn test_memory_pool_empty_transaction_id() {
    let pool = MemoryPool::new();
    let empty_id = vec![];
    let tx = create_test_transaction(empty_id.clone());
    let tx_hex = HEXLOWER.encode(&empty_id);

    pool.add(tx.clone());

    assert!(pool.contains(&tx_hex));
    assert_eq!(pool.get(&tx_hex).unwrap().get_id(), empty_id.as_slice());
}

#[test]
fn test_memory_pool_single_byte_transaction_id() {
    let pool = MemoryPool::new();
    let single_byte_id = vec![42];
    let tx = create_test_transaction(single_byte_id.clone());
    let tx_hex = HEXLOWER.encode(&single_byte_id);

    pool.add(tx.clone());

    assert!(pool.contains(&tx_hex));
    assert_eq!(
        pool.get(&tx_hex).unwrap().get_id(),
        single_byte_id.as_slice()
    );
}

// =============================================================================
// MEMORY POOL INTEGRATION TESTS
// =============================================================================

#[test]
fn test_memory_pool_full_workflow() {
    let pool = MemoryPool::new();

    // Start with empty pool
    assert!(pool.is_empty());
    assert_eq!(pool.len(), 0);
    assert!(pool.get_all().is_empty());

    // Add transactions
    let tx1 = create_test_transaction(vec![1, 2, 3]);
    let tx2 = create_coinbase_transaction(50, vec![4, 5, 6]);
    let tx3 = create_spending_transaction(vec![(vec![7, 8, 9], 0)], vec![(100, vec![10, 11, 12])]);

    pool.add(tx1.clone());
    pool.add(tx2.clone());
    pool.add(tx3.clone());

    // Verify state
    assert!(!pool.is_empty());
    assert_eq!(pool.len(), 3);

    let all_tx = pool.get_all();
    assert_eq!(all_tx.len(), 3);

    // Test individual access
    let tx1_hex = HEXLOWER.encode(tx1.get_id());
    let tx2_hex = HEXLOWER.encode(tx2.get_id());
    let tx3_hex = HEXLOWER.encode(tx3.get_id());

    assert!(pool.contains(&tx1_hex));
    assert!(pool.contains(&tx2_hex));
    assert!(pool.contains(&tx3_hex));

    assert!(pool.get(&tx1_hex).is_some());
    assert!(pool.get(&tx2_hex).is_some());
    assert!(pool.get(&tx3_hex).is_some());

    // Remove one transaction
    pool.remove(&tx2_hex);
    assert_eq!(pool.len(), 2);
    assert!(!pool.contains(&tx2_hex));
    assert!(pool.get(&tx2_hex).is_none());

    // Verify others are still present
    assert!(pool.contains(&tx1_hex));
    assert!(pool.contains(&tx3_hex));

    // Clear remaining transactions
    pool.remove(&tx1_hex);
    pool.remove(&tx3_hex);

    assert!(pool.is_empty());
    assert_eq!(pool.len(), 0);
    assert!(pool.get_all().is_empty());
}

#[test]
fn test_memory_pool_hex_encoding_consistency() {
    let pool = MemoryPool::new();
    let tx = create_test_transaction(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);

    pool.add(tx.clone());

    // Test that hex encoding is consistent
    let expected_hex = HEXLOWER.encode(tx.get_id());
    assert!(pool.contains(&expected_hex));

    let retrieved_tx = pool.get(&expected_hex);
    assert!(retrieved_tx.is_some());
    assert_eq!(retrieved_tx.unwrap().get_id(), tx.get_id());
}
