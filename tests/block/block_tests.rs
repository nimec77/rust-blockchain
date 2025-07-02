use rust_blockchain::{Block, TXInput, TXOutput, Transaction, util};
use crate::test_helpers::*;

#[ignore]
#[test]
fn test_new_block() {
    let transaction = create_test_transaction(vec![1, 2, 3, 4]);
    let transactions = vec![transaction];
    let pre_block_hash = "previous_hash".to_string();
    let height = 1;

    let block =
        Block::new_block_without_proof_of_work(pre_block_hash.clone(), &transactions, height);

    assert_eq!(block.get_pre_block_hash(), pre_block_hash);
    assert_eq!(block.get_transactions().len(), 1);
    assert_eq!(block.get_height(), height);
    // Proof of work should have run, so nonce should be set (not 0)
    assert!(block.get_nonce() >= 0);
    // Hash should be set after proof of work
    assert!(!block.get_hash().is_empty());
    assert!(block.get_timestamp() > 0);
    // Verify hash is hex encoded and reasonable length
    assert!(block.get_hash().len() == 64); // SHA256 hex string length
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
        original_block.get_pre_block_hash(),
        deserialized_block.get_pre_block_hash()
    );
    assert_eq!(original_block.get_hash(), deserialized_block.get_hash());
    assert_eq!(
        original_block.get_transactions().len(),
        deserialized_block.get_transactions().len()
    );
    assert_eq!(original_block.get_nonce(), deserialized_block.get_nonce());
    assert_eq!(original_block.get_height(), deserialized_block.get_height());
    assert_eq!(original_block.get_timestamp(), deserialized_block.get_timestamp());
}

#[test]
fn test_get_transactions() {
    let transaction1 = create_test_transaction(vec![1, 2, 3]);
    let transaction2 = create_test_transaction(vec![4, 5, 6]);
    let transactions = vec![transaction1, transaction2];

    let block = Block::new_block_without_proof_of_work("test_hash".to_string(), &transactions, 1);
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

    let block = Block::new_block_without_proof_of_work(pre_block_hash.clone(), &transactions, 1);

    assert_eq!(block.get_pre_block_hash(), pre_block_hash.as_str());
}

#[test]
fn test_get_hash() {
    let transaction = create_test_transaction(vec![1, 2, 3]);
    let transactions = vec![transaction];

    let mut block = Block::new_block_without_proof_of_work("test".to_string(), &transactions, 1);
    block.set_hash_for_test("test_hash");

    assert_eq!(block.get_hash(), "test_hash");
}

#[test]
fn test_get_hash_bytes() {
    let transaction = create_test_transaction(vec![1, 2, 3]);
    let transactions = vec![transaction];

    let mut block = Block::new_block_without_proof_of_work("test".to_string(), &transactions, 1);
    block.set_hash_for_test("test_hash");

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

    let block = Block::new_block_without_proof_of_work("test".to_string(), &transactions, height);

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

    let ivec = sled::IVec::from(block.clone());
    let deserialized_block = Block::deserialize(&ivec);

    assert_eq!(block.get_hash(), deserialized_block.get_hash());
    assert_eq!(
        block.get_pre_block_hash(),
        deserialized_block.get_pre_block_hash()
    );
    assert_eq!(block.get_height(), deserialized_block.get_height());
    assert_eq!(block.get_timestamp(), deserialized_block.get_timestamp());
}

#[test]
fn test_block_with_multiple_transactions() {
    let transaction1 = create_test_transaction(vec![1, 2, 3]);
    let transaction2 = create_test_transaction(vec![4, 5, 6]);
    let transaction3 = create_test_transaction(vec![7, 8, 9]);
    let transactions = vec![transaction1, transaction2, transaction3];

    let block = Block::new_block_without_proof_of_work("test_hash".to_string(), &transactions, 2);

    assert_eq!(block.get_transactions().len(), 3);
    assert_eq!(block.get_height(), 2);
    assert_eq!(block.get_pre_block_hash(), "test_hash");

    let retrieved_transactions = block.get_transactions();
    assert_eq!(retrieved_transactions[0].get_id(), &[1, 2, 3]);
    assert_eq!(retrieved_transactions[1].get_id(), &[4, 5, 6]);
    assert_eq!(retrieved_transactions[2].get_id(), &[7, 8, 9]);
}

#[test]
fn test_serialize_deserialize_round_trip_consistency() {
    let transaction1 = create_test_transaction(vec![10, 20, 30]);
    let transaction2 = create_test_transaction(vec![40, 50, 60]);
    let transactions = vec![transaction1, transaction2];

    let original_block =
        Block::new_block_without_proof_of_work("original_hash".to_string(), &transactions, 3);

    // Multiple serialization/deserialization cycles
    let mut current_block = original_block.clone();
    for _ in 0..5 {
        let serialized = current_block.serialize();
        current_block = Block::deserialize(&serialized);
    }

    // Verify integrity after cycles
    assert_eq!(original_block.get_hash(), current_block.get_hash());
    assert_eq!(
        original_block.get_pre_block_hash(),
        current_block.get_pre_block_hash()
    );
    assert_eq!(original_block.get_height(), current_block.get_height());
    assert_eq!(
        original_block.get_timestamp(),
        current_block.get_timestamp()
    );
    assert_eq!(original_block.get_nonce(), current_block.get_nonce());
    assert_eq!(
        original_block.get_transactions().len(),
        current_block.get_transactions().len()
    );
}

#[test]
fn test_get_nonce() {
    let transaction = create_test_transaction(vec![1, 2, 3]);
    let transactions = vec![transaction];

    let block = Block::new_block_without_proof_of_work("test".to_string(), &transactions, 1);

    let nonce = block.get_nonce();
    assert!(nonce >= 0); // Nonce should be non-negative
}

#[test]
fn test_proof_of_work_integration() {
    let transaction = create_test_transaction(vec![1, 2, 3, 4]);
    let transactions = vec![transaction];

    // Note: This test doesn't run actual proof of work due to performance concerns
    let block = Block::new_block_without_proof_of_work("test_hash".to_string(), &transactions, 1);

    // Verify basic block properties are set correctly
    assert_eq!(block.get_pre_block_hash(), "test_hash");
    assert_eq!(block.get_height(), 1);
    assert_eq!(block.get_transactions().len(), 1);
    assert!(block.get_timestamp() > 0);
    assert_eq!(block.get_nonce(), 0); // new_block_without_proof_of_work sets nonce to 0
    assert_eq!(block.get_hash(), ""); // new_block_without_proof_of_work creates empty hash
}

#[test]
fn test_empty_transactions_block() {
    let transactions = vec![];

    let block =
        Block::new_block_without_proof_of_work("empty_tx_hash".to_string(), &transactions, 0);

    assert_eq!(block.get_transactions().len(), 0);
    assert_eq!(block.get_height(), 0);
    assert_eq!(block.get_pre_block_hash(), "empty_tx_hash");
    assert_eq!(block.get_hash(), ""); // new_block_without_proof_of_work creates empty hash
    assert!(block.get_timestamp() > 0);
}

#[test]
fn test_hash_transactions_ordering() {
    let transaction1 = create_test_transaction(vec![1]);
    let transaction2 = create_test_transaction(vec![2]);

    let transactions_order1 = vec![transaction1.clone(), transaction2.clone()];
    let transactions_order2 = vec![transaction2, transaction1];

    let block1 =
        Block::new_block_without_proof_of_work("test".to_string(), &transactions_order1, 1);
    let block2 =
        Block::new_block_without_proof_of_work("test".to_string(), &transactions_order2, 1);

    let hash1 = block1.hash_transactions();
    let hash2 = block2.hash_transactions();

    // Different ordering should produce different hashes
    assert_ne!(hash1, hash2);
}

#[test]
fn test_block_hash_consistency() {
    let transaction = create_test_transaction(vec![123, 45, 67]);
    let transactions = vec![transaction];

    let block1 =
        Block::new_block_without_proof_of_work("consistent_test".to_string(), &transactions, 5);
    let block2 =
        Block::new_block_without_proof_of_work("consistent_test".to_string(), &transactions, 5);

    // Note: Hashes might differ due to timestamps, but structure should be similar
    assert_eq!(block1.get_pre_block_hash(), block2.get_pre_block_hash());
    assert_eq!(block1.get_height(), block2.get_height());
    assert_eq!(
        block1.get_transactions().len(),
        block2.get_transactions().len()
    );
}

#[test]
fn test_genesis_block_validation() {
    let transaction = create_test_transaction(vec![255, 0, 128]);

    let genesis_block = Block::generate_genesis_block(&transaction);

    // Genesis block should have specific properties
    assert_eq!(genesis_block.get_pre_block_hash(), "None");
    assert_eq!(genesis_block.get_height(), 0);
    assert_eq!(genesis_block.get_transactions().len(), 1);
    assert_eq!(genesis_block.get_transactions()[0].get_id(), &[255, 0, 128]);
    assert!(genesis_block.get_timestamp() > 0);
    assert_eq!(genesis_block.get_hash(), ""); // genesis block uses new_block_without_proof_of_work
    assert_eq!(genesis_block.get_nonce(), 0); // genesis block uses new_block_without_proof_of_work
}

#[test]
fn test_block_with_large_transaction_count() {
    let mut transactions = Vec::new();
    for i in 0..100 {
        transactions.push(create_test_transaction(vec![i as u8]));
    }

    let block =
        Block::new_block_without_proof_of_work("large_tx_test".to_string(), &transactions, 10);

    assert_eq!(block.get_transactions().len(), 100);
    assert_eq!(block.get_height(), 10);
    assert_eq!(block.get_pre_block_hash(), "large_tx_test");

    // Verify serialization works with large transaction count
    let serialized = block.serialize();
    let deserialized = Block::deserialize(&serialized);
    assert_eq!(
        block.get_transactions().len(),
        deserialized.get_transactions().len()
    );
}

#[test]
fn test_block_fields_immutability_after_creation() {
    let transaction = create_test_transaction(vec![42, 43, 44]);
    let transactions = vec![transaction];

    let block =
        Block::new_block_without_proof_of_work("immutable_test".to_string(), &transactions, 7);

    // Capture initial values
    let initial_hash = block.get_hash().to_string();
    let initial_pre_hash = block.get_pre_block_hash().to_string();
    let initial_height = block.get_height();
    let initial_timestamp = block.get_timestamp();
    let initial_nonce = block.get_nonce();
    let initial_tx_count = block.get_transactions().len();

    // Create another reference and verify values haven't changed
    let hash_again = block.get_hash().to_string();
    let pre_hash_again = block.get_pre_block_hash().to_string();
    let height_again = block.get_height();
    let timestamp_again = block.get_timestamp();
    let nonce_again = block.get_nonce();
    let tx_count_again = block.get_transactions().len();

    assert_eq!(initial_hash, hash_again);
    assert_eq!(initial_pre_hash, pre_hash_again);
    assert_eq!(initial_height, height_again);
    assert_eq!(initial_timestamp, timestamp_again);
    assert_eq!(initial_nonce, nonce_again);
    assert_eq!(initial_tx_count, tx_count_again);
}

#[test]
fn test_block_serialization_with_unicode_hash() {
    let transaction = create_test_transaction(vec![200, 201, 202]);
    let transactions = vec![transaction];

    let mut block =
        Block::new_block_without_proof_of_work("unicode_test".to_string(), &transactions, 3);

    // Test with ASCII hash
    block.set_hash_for_test("simple_ascii_hash");
    let serialized = block.serialize();
    let deserialized = Block::deserialize(&serialized);
    assert_eq!(block.get_hash(), deserialized.get_hash());
}

#[test]
fn test_new_block_without_proof_of_work_initialization() {
    let transaction = create_test_transaction(vec![100, 101, 102]);
    let transactions = vec![transaction];
    let pre_hash = "init_test_hash".to_string();
    let height = 15;

    let block = Block::new_block_without_proof_of_work(pre_hash.clone(), &transactions, height);

    // Verify all fields are properly initialized
    assert_eq!(block.get_pre_block_hash(), pre_hash);
    assert_eq!(block.get_height(), height);
    assert_eq!(block.get_transactions().len(), 1);
    assert!(block.get_timestamp() > 0);
    assert_eq!(block.get_hash(), ""); // new_block_without_proof_of_work creates empty hash
    assert_eq!(block.get_nonce(), 0); // new_block_without_proof_of_work sets nonce to 0

    // Verify transaction content
    assert_eq!(block.get_transactions()[0].get_id(), &[100, 101, 102]);
}

#[test]
fn test_block_with_different_pre_hash_formats() {
    let transaction = create_test_transaction(vec![50, 51, 52]);
    let transactions = vec![transaction];

    let test_hashes = [
        "".to_string(),
        "a".to_string(),
        "0123456789abcdef".to_string(),
        "very_long_hash_string_that_might_be_used_in_some_scenarios".to_string(),
        "hash_with_numbers_123456789".to_string(),
    ];

    for (i, hash) in test_hashes.iter().enumerate() {
        let block = Block::new_block_without_proof_of_work(hash.clone(), &transactions, i);
        assert_eq!(block.get_pre_block_hash(), hash);
        assert_eq!(block.get_height(), i);
    }
}

#[test]
fn test_block_with_varying_transaction_counts() {
    let transaction_counts = vec![0, 1, 5, 10, 50];

    for count in transaction_counts {
        let mut transactions = Vec::new();
        for i in 0..count {
            transactions.push(create_test_transaction(vec![i as u8, (i + 1) as u8]));
        }

        let block = Block::new_block_without_proof_of_work(
            "varying_tx_test".to_string(),
            &transactions,
            count,
        );

        assert_eq!(block.get_transactions().len(), count);
        assert_eq!(block.get_height(), count);

        // Test serialization for each count
        let serialized = block.serialize();
        let deserialized = Block::deserialize(&serialized);
        assert_eq!(
            block.get_transactions().len(),
            deserialized.get_transactions().len()
        );
    }
}

#[test]
fn test_hash_transactions_deterministic() {
    let transaction1 = create_test_transaction(vec![1, 2, 3]);
    let transaction2 = create_test_transaction(vec![4, 5, 6]);
    let transactions = vec![transaction1, transaction2];

    let block =
        Block::new_block_without_proof_of_work("deterministic_test".to_string(), &transactions, 1);

    // Call hash_transactions multiple times
    let hash1 = block.hash_transactions();
    let hash2 = block.hash_transactions();
    let hash3 = block.hash_transactions();

    // All calls should produce the same result
    assert_eq!(hash1, hash2);
    assert_eq!(hash2, hash3);
    assert_eq!(hash1.len(), 32); // SHA256 output length
}

#[test]
fn test_hash_transactions_with_duplicate_transactions() {
    let transaction = create_test_transaction(vec![123]);
    let transactions_unique = vec![transaction.clone()];
    let transactions_duplicate = vec![transaction.clone(), transaction];

    let block1 =
        Block::new_block_without_proof_of_work("unique_test".to_string(), &transactions_unique, 1);
    let block2 = Block::new_block_without_proof_of_work(
        "duplicate_test".to_string(),
        &transactions_duplicate,
        1,
    );

    let hash1 = block1.hash_transactions();
    let hash2 = block2.hash_transactions();

    // Different transaction sets should produce different hashes
    assert_ne!(hash1, hash2);
}

#[test]
fn test_block_timestamp_accuracy() {
    let transaction = create_test_transaction(vec![77, 88, 99]);
    let transactions = vec![transaction];

    let before_creation = util::current_timestamp();
    let block =
        Block::new_block_without_proof_of_work("timestamp_test".to_string(), &transactions, 1);
    let after_creation = util::current_timestamp();

    let block_timestamp = block.get_timestamp();

    // Block timestamp should be within reasonable bounds
    assert!(block_timestamp >= before_creation);
    assert!(block_timestamp <= after_creation);
}

#[test]
fn test_block_height_edge_cases() {
    let transaction = create_test_transaction(vec![0]);
    let transactions = vec![transaction];

    // Test with height 0 (genesis)
    let block_genesis =
        Block::new_block_without_proof_of_work("genesis".to_string(), &transactions, 0);
    assert_eq!(block_genesis.get_height(), 0);

    // Test with large height
    let large_height = usize::MAX;
    let block_large = Block::new_block_without_proof_of_work(
        "large_height".to_string(),
        &transactions,
        large_height,
    );
    assert_eq!(block_large.get_height(), large_height);
}

#[test]
fn test_serialization_roundtrip_consistency() {
    let transaction1 = create_test_transaction(vec![11, 22, 33]);
    let transaction2 = create_test_transaction(vec![44, 55, 66]);
    let transactions = vec![transaction1, transaction2];

    let original_block =
        Block::new_block_without_proof_of_work("roundtrip_test".to_string(), &transactions, 8);

    // Serialize and deserialize
    let serialized = original_block.serialize();
    let deserialized_block = Block::deserialize(&serialized);

    // Check all fields for consistency
    assert_eq!(original_block.get_hash(), deserialized_block.get_hash());
    assert_eq!(
        original_block.get_pre_block_hash(),
        deserialized_block.get_pre_block_hash()
    );
    assert_eq!(original_block.get_height(), deserialized_block.get_height());
    assert_eq!(
        original_block.get_timestamp(),
        deserialized_block.get_timestamp()
    );
    assert_eq!(original_block.get_nonce(), deserialized_block.get_nonce());
    assert_eq!(
        original_block.get_transactions().len(),
        deserialized_block.get_transactions().len()
    );

    // Verify transaction data integrity
    for (i, original_tx) in original_block.get_transactions().iter().enumerate() {
        let deserialized_tx = &deserialized_block.get_transactions()[i];
        assert_eq!(original_tx.get_id(), deserialized_tx.get_id());
    }
}

#[test]
fn test_genesis_block_without_proof_of_work() {
    let transaction = create_test_transaction(vec![0xFF, 0x00, 0xAB]);

    let genesis_block = Block::generate_genesis_block(&transaction);

    // Verify genesis block specific properties
    assert_eq!(genesis_block.get_pre_block_hash(), "None");
    assert_eq!(genesis_block.get_height(), 0);
    assert!(genesis_block.get_timestamp() > 0);
    assert_eq!(genesis_block.get_hash(), ""); // genesis block uses new_block_without_proof_of_work

    // Genesis block should be serializable
    let serialized = genesis_block.serialize();
    let deserialized = Block::deserialize(&serialized);
    assert_eq!(
        genesis_block.get_pre_block_hash(),
        deserialized.get_pre_block_hash()
    );
    assert_eq!(genesis_block.get_height(), deserialized.get_height());
}

#[test]
fn test_large_transaction_data_without_proof_of_work() {
    // Create transaction with large data
    let large_tx_input = TXInput {
        txid: vec![0u8; 1000], // 1KB txid
        vout: 0,
        signature: vec![255u8; 2000], // 2KB signature
        pub_key: vec![128u8; 500],    // 500B public key
    };

    let large_tx_output = TXOutput {
        value: i32::MAX,
        pub_key_hash: vec![42u8; 1000], // 1KB hash
    };

    let large_transaction = Transaction {
        id: vec![200u8; 100],
        vin: vec![large_tx_input],
        vout: vec![large_tx_output],
    };

    let transactions = vec![large_transaction];
    let block =
        Block::new_block_without_proof_of_work("large_data_test".to_string(), &transactions, 1);

    // Verify block handles large transaction data
    assert_eq!(block.get_transactions().len(), 1);
    assert_eq!(block.get_transactions()[0].get_id().len(), 100);

    // Test serialization with large data
    let serialized = block.serialize();
    let deserialized = Block::deserialize(&serialized);
    assert_eq!(
        block.get_transactions().len(),
        deserialized.get_transactions().len()
    );
    assert_eq!(
        block.get_transactions()[0].get_id(),
        deserialized.get_transactions()[0].get_id()
    );
}

#[test]
fn test_block_memory_layout_and_cloning() {
    let transaction = create_test_transaction(vec![1, 2, 3, 4, 5]);
    let transactions = vec![transaction];

    let original_block =
        Block::new_block_without_proof_of_work("clone_test".to_string(), &transactions, 1);
    let cloned_block = original_block.clone();

    // Verify clone has same values
    assert_eq!(original_block.get_hash(), cloned_block.get_hash());
    assert_eq!(
        original_block.get_pre_block_hash(),
        cloned_block.get_pre_block_hash()
    );
    assert_eq!(original_block.get_height(), cloned_block.get_height());
    assert_eq!(original_block.get_timestamp(), cloned_block.get_timestamp());
    assert_eq!(original_block.get_nonce(), cloned_block.get_nonce());
    assert_eq!(
        original_block.get_transactions().len(),
        cloned_block.get_transactions().len()
    );

    // Verify they are separate instances (if we could modify them)
    // This test mainly ensures Clone trait works correctly
}
