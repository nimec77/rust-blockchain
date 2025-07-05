use rust_blockchain::BlockInTransit;
use std::sync::Arc;
use std::thread;

// =============================================================================
// BLOCK IN TRANSIT CONSTRUCTOR TESTS
// =============================================================================

#[test]
fn test_block_in_transit_new() {
    let bit = BlockInTransit::new();
    assert!(bit.is_empty());
    assert_eq!(bit.len(), 0);
}

#[test]
fn test_block_in_transit_default() {
    let bit = BlockInTransit::default();
    assert!(bit.is_empty());
    assert_eq!(bit.len(), 0);
}

// =============================================================================
// BLOCK IN TRANSIT ADD_BLOCKS TESTS
// =============================================================================

#[test]
fn test_add_blocks_single_block() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![1, 2, 3, 4]];
    
    bit.add_blocks(&blocks);
    
    assert!(!bit.is_empty());
    assert_eq!(bit.len(), 1);
    assert_eq!(bit.first(), Some(vec![1, 2, 3, 4]));
}

#[test]
fn test_add_blocks_multiple_blocks() {
    let bit = BlockInTransit::new();
    let blocks = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    
    bit.add_blocks(&blocks);
    
    assert!(!bit.is_empty());
    assert_eq!(bit.len(), 3);
    assert_eq!(bit.first(), Some(vec![1, 2, 3, 4]));
}

#[test]
fn test_add_blocks_empty_slice() {
    let bit = BlockInTransit::new();
    let blocks: Vec<Vec<u8>> = vec![];
    
    bit.add_blocks(&blocks);
    
    assert!(bit.is_empty());
    assert_eq!(bit.len(), 0);
}

#[test]
fn test_add_blocks_multiple_calls() {
    let bit = BlockInTransit::new();
    let blocks1 = vec![vec![1, 2, 3, 4]];
    let blocks2 = vec![vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    
    bit.add_blocks(&blocks1);
    bit.add_blocks(&blocks2);
    
    assert_eq!(bit.len(), 3);
    assert_eq!(bit.first(), Some(vec![1, 2, 3, 4]));
}

#[test]
fn test_add_blocks_duplicate_blocks() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![1, 2, 3, 4], vec![1, 2, 3, 4]];
    
    bit.add_blocks(&blocks);
    
    assert_eq!(bit.len(), 2); // Should allow duplicates
}

#[test]
fn test_add_blocks_empty_block_hash() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![]];
    
    bit.add_blocks(&blocks);
    
    assert_eq!(bit.len(), 1);
    assert_eq!(bit.first(), Some(vec![]));
}

#[test]
fn test_add_blocks_large_block_hash() {
    let bit = BlockInTransit::new();
    let large_hash = vec![0u8; 256]; // 256 bytes
    let blocks = vec![large_hash.clone()];
    
    bit.add_blocks(&blocks);
    
    assert_eq!(bit.len(), 1);
    assert_eq!(bit.first(), Some(large_hash));
}

// =============================================================================
// BLOCK IN TRANSIT FIRST TESTS
// =============================================================================

#[test]
fn test_first_empty_container() {
    let bit = BlockInTransit::new();
    assert_eq!(bit.first(), None);
}

#[test]
fn test_first_single_block() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![1, 2, 3, 4]];
    
    bit.add_blocks(&blocks);
    
    assert_eq!(bit.first(), Some(vec![1, 2, 3, 4]));
}

#[test]
fn test_first_multiple_blocks() {
    let bit = BlockInTransit::new();
    let blocks = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    
    bit.add_blocks(&blocks);
    
    assert_eq!(bit.first(), Some(vec![1, 2, 3, 4]));
}

#[test]
fn test_first_returns_copy() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![1, 2, 3, 4]];
    
    bit.add_blocks(&blocks);
    
    let first_block = bit.first().unwrap();
    assert_eq!(first_block, vec![1, 2, 3, 4]);
    
    // Modify the returned value - should not affect the original
    let mut modified_block = first_block;
    modified_block[0] = 99;
    
    assert_eq!(bit.first(), Some(vec![1, 2, 3, 4])); // Original unchanged
}

// =============================================================================
// BLOCK IN TRANSIT REMOVE TESTS
// =============================================================================

#[test]
fn test_remove_existing_block() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
    
    bit.add_blocks(&blocks);
    bit.remove(&[1, 2, 3, 4]);
    
    assert_eq!(bit.len(), 1);
    assert_eq!(bit.first(), Some(vec![5, 6, 7, 8]));
}

#[test]
fn test_remove_non_existing_block() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![1, 2, 3, 4]];
    
    bit.add_blocks(&blocks);
    bit.remove(&[5, 6, 7, 8]);
    
    assert_eq!(bit.len(), 1);
    assert_eq!(bit.first(), Some(vec![1, 2, 3, 4]));
}

#[test]
fn test_remove_from_empty_container() {
    let bit = BlockInTransit::new();
    bit.remove(&[1, 2, 3, 4]);
    
    assert!(bit.is_empty());
}

#[test]
fn test_remove_first_block() {
    let bit = BlockInTransit::new();
    let blocks = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    
    bit.add_blocks(&blocks);
    bit.remove(&[1, 2, 3, 4]);
    
    assert_eq!(bit.len(), 2);
    assert_eq!(bit.first(), Some(vec![5, 6, 7, 8]));
}

#[test]
fn test_remove_middle_block() {
    let bit = BlockInTransit::new();
    let blocks = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    
    bit.add_blocks(&blocks);
    bit.remove(&[5, 6, 7, 8]);
    
    assert_eq!(bit.len(), 2);
    assert_eq!(bit.first(), Some(vec![1, 2, 3, 4]));
}

#[test]
fn test_remove_last_block() {
    let bit = BlockInTransit::new();
    let blocks = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    
    bit.add_blocks(&blocks);
    bit.remove(&[9, 10, 11, 12]);
    
    assert_eq!(bit.len(), 2);
    assert_eq!(bit.first(), Some(vec![1, 2, 3, 4]));
}

#[test]
fn test_remove_only_matching_block() {
    let bit = BlockInTransit::new();
    let blocks = vec![
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4], // Duplicate
        vec![5, 6, 7, 8],
    ];
    
    bit.add_blocks(&blocks);
    bit.remove(&[1, 2, 3, 4]);
    
    assert_eq!(bit.len(), 2);
    // Should only remove the first occurrence
    assert_eq!(bit.first(), Some(vec![1, 2, 3, 4]));
}

#[test]
fn test_remove_empty_block_hash() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![], vec![1, 2, 3, 4]];
    
    bit.add_blocks(&blocks);
    bit.remove(&[]);
    
    assert_eq!(bit.len(), 1);
    assert_eq!(bit.first(), Some(vec![1, 2, 3, 4]));
}

// =============================================================================
// BLOCK IN TRANSIT CLEAR TESTS
// =============================================================================

#[test]
fn test_clear_empty_container() {
    let bit = BlockInTransit::new();
    bit.clear();
    
    assert!(bit.is_empty());
    assert_eq!(bit.len(), 0);
}

#[test]
fn test_clear_single_block() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![1, 2, 3, 4]];
    
    bit.add_blocks(&blocks);
    bit.clear();
    
    assert!(bit.is_empty());
    assert_eq!(bit.len(), 0);
    assert_eq!(bit.first(), None);
}

#[test]
fn test_clear_multiple_blocks() {
    let bit = BlockInTransit::new();
    let blocks = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    
    bit.add_blocks(&blocks);
    bit.clear();
    
    assert!(bit.is_empty());
    assert_eq!(bit.len(), 0);
    assert_eq!(bit.first(), None);
}

#[test]
fn test_clear_and_add_again() {
    let bit = BlockInTransit::new();
    let blocks1 = vec![vec![1, 2, 3, 4]];
    let blocks2 = vec![vec![5, 6, 7, 8]];
    
    bit.add_blocks(&blocks1);
    bit.clear();
    bit.add_blocks(&blocks2);
    
    assert_eq!(bit.len(), 1);
    assert_eq!(bit.first(), Some(vec![5, 6, 7, 8]));
}

// =============================================================================
// BLOCK IN TRANSIT LEN TESTS
// =============================================================================

#[test]
fn test_len_empty_container() {
    let bit = BlockInTransit::new();
    assert_eq!(bit.len(), 0);
}

#[test]
fn test_len_single_block() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![1, 2, 3, 4]];
    
    bit.add_blocks(&blocks);
    assert_eq!(bit.len(), 1);
}

#[test]
fn test_len_multiple_blocks() {
    let bit = BlockInTransit::new();
    let blocks = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    
    bit.add_blocks(&blocks);
    assert_eq!(bit.len(), 3);
}

#[test]
fn test_len_after_remove() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
    
    bit.add_blocks(&blocks);
    assert_eq!(bit.len(), 2);
    
    bit.remove(&[1, 2, 3, 4]);
    assert_eq!(bit.len(), 1);
}

#[test]
fn test_len_after_clear() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
    
    bit.add_blocks(&blocks);
    assert_eq!(bit.len(), 2);
    
    bit.clear();
    assert_eq!(bit.len(), 0);
}

// =============================================================================
// BLOCK IN TRANSIT IS_EMPTY TESTS
// =============================================================================

#[test]
fn test_is_empty_new_container() {
    let bit = BlockInTransit::new();
    assert!(bit.is_empty());
}

#[test]
fn test_is_empty_after_add() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![1, 2, 3, 4]];
    
    bit.add_blocks(&blocks);
    assert!(!bit.is_empty());
}

#[test]
fn test_is_empty_after_remove_all() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![1, 2, 3, 4]];
    
    bit.add_blocks(&blocks);
    bit.remove(&[1, 2, 3, 4]);
    assert!(bit.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    let bit = BlockInTransit::new();
    let blocks = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
    
    bit.add_blocks(&blocks);
    bit.clear();
    assert!(bit.is_empty());
}

#[test]
fn test_is_empty_consistency_with_len() {
    let bit = BlockInTransit::new();
    
    // Empty container
    let len = bit.len();
    assert_eq!(bit.is_empty(), len == 0);
    
    // Add blocks
    let blocks = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
    bit.add_blocks(&blocks);
    let len = bit.len();
    assert_eq!(bit.is_empty(), len == 0);
    
    // Remove one block
    bit.remove(&[1, 2, 3, 4]);
    let len = bit.len();
    assert_eq!(bit.is_empty(), len == 0);
    
    // Remove all blocks
    bit.remove(&[5, 6, 7, 8]);
    let len = bit.len();
    assert_eq!(bit.is_empty(), len == 0);
}

// =============================================================================
// BLOCK IN TRANSIT CONCURRENCY TESTS
// =============================================================================

#[test]
fn test_concurrent_add_blocks() {
    let bit = Arc::new(BlockInTransit::new());
    let mut handles = vec![];
    
    for i in 0..10 {
        let bit_clone = Arc::clone(&bit);
        let handle = thread::spawn(move || {
            let blocks = vec![vec![i as u8; 4]];
            bit_clone.add_blocks(&blocks);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert_eq!(bit.len(), 10);
}

#[test]
fn test_concurrent_read_write() {
    let bit = Arc::new(BlockInTransit::new());
    let mut handles = vec![];
    
    // Add some initial blocks
    let initial_blocks = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]];
    bit.add_blocks(&initial_blocks);
    
    // Spawn writer threads
    for i in 0..5 {
        let bit_clone = Arc::clone(&bit);
        let handle = thread::spawn(move || {
            let blocks = vec![vec![i as u8 + 10; 4]];
            bit_clone.add_blocks(&blocks);
        });
        handles.push(handle);
    }
    
    // Spawn reader threads
    for _ in 0..5 {
        let bit_clone = Arc::clone(&bit);
        let handle = thread::spawn(move || {
            let _first = bit_clone.first();
            let _len = bit_clone.len();
            let _empty = bit_clone.is_empty();
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert_eq!(bit.len(), 7); // 2 initial + 5 added
}

#[test]
fn test_concurrent_add_remove() {
    let bit = Arc::new(BlockInTransit::new());
    let mut handles = vec![];
    
    // Add initial blocks
    let initial_blocks: Vec<Vec<u8>> = (0..20).map(|i| vec![i as u8; 4]).collect();
    bit.add_blocks(&initial_blocks);
    
    // Spawn add threads
    for i in 20..25 {
        let bit_clone = Arc::clone(&bit);
        let handle = thread::spawn(move || {
            let blocks = vec![vec![i as u8; 4]];
            bit_clone.add_blocks(&blocks);
        });
        handles.push(handle);
    }
    
    // Spawn remove threads
    for i in 0..5 {
        let bit_clone = Arc::clone(&bit);
        let handle = thread::spawn(move || {
            let block_to_remove = vec![i as u8; 4];
            bit_clone.remove(&block_to_remove);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert_eq!(bit.len(), 20); // 20 initial - 5 removed + 5 added
}

// =============================================================================
// BLOCK IN TRANSIT EDGE CASES
// =============================================================================

#[test]
fn test_large_number_of_blocks() {
    let bit = BlockInTransit::new();
    let blocks: Vec<Vec<u8>> = (0..1000).map(|i| vec![i as u8; 32]).collect();
    
    bit.add_blocks(&blocks);
    
    assert_eq!(bit.len(), 1000);
    assert_eq!(bit.first(), Some(vec![0; 32]));
}

#[test]
fn test_mixed_block_sizes() {
    let bit = BlockInTransit::new();
    let blocks = vec![
        vec![1],                    // 1 byte
        vec![2, 3],                 // 2 bytes
        vec![4, 5, 6, 7, 8],       // 5 bytes
        vec![9; 32],               // 32 bytes
        vec![10; 64],              // 64 bytes
    ];
    
    bit.add_blocks(&blocks);
    
    assert_eq!(bit.len(), 5);
    assert_eq!(bit.first(), Some(vec![1]));
}

#[test]
fn test_workflow_integration() {
    let bit = BlockInTransit::new();
    
    // Simulate a typical workflow
    let pending_blocks = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    
    // Add blocks to transit
    bit.add_blocks(&pending_blocks);
    assert_eq!(bit.len(), 3);
    
    // Process first block
    let first_block = bit.first().unwrap();
    assert_eq!(first_block, vec![1, 2, 3, 4]);
    
    // Remove processed block
    bit.remove(&first_block);
    assert_eq!(bit.len(), 2);
    
    // Process second block
    let second_block = bit.first().unwrap();
    assert_eq!(second_block, vec![5, 6, 7, 8]);
    bit.remove(&second_block);
    assert_eq!(bit.len(), 1);
    
    // Add more blocks
    let more_blocks = vec![vec![13, 14, 15, 16]];
    bit.add_blocks(&more_blocks);
    assert_eq!(bit.len(), 2);
    
    // Clear all
    bit.clear();
    assert!(bit.is_empty());
} 
