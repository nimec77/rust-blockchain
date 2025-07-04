use rust_blockchain::{Block, Blockchain, TXOutput, UTXOSet, BLOCKS_TREE, TIP_BLOCK_HASH_KEY};
use crate::test_helpers::*;

// =============================================================================
// CONSTRUCTOR AND BASIC FUNCTIONALITY TESTS
// =============================================================================

#[test]
fn test_utxo_set_new() {
    let test_name = "utxo_set_new";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    
    let utxo_set = UTXOSet::new(blockchain.clone());
    
    // Verify the UTXOSet was created successfully and can access the blockchain
    assert_eq!(utxo_set.get_blockchain().get_tip_hash(), "");
    assert_eq!(utxo_set.count_transactions(), 0);
}

#[test]
fn test_utxo_set_get_blockchain() {
    let test_name = "utxo_set_get_blockchain";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    
    let utxo_set = UTXOSet::new(blockchain.clone());
    let retrieved_blockchain = utxo_set.get_blockchain();
    
    // Verify we get back a working blockchain instance
    assert_eq!(retrieved_blockchain.get_tip_hash(), blockchain.get_tip_hash());
    
    // Test that we can use the blockchain from the UTXOSet
    let tree = retrieved_blockchain.get_db().open_tree("test_tree").unwrap();
    assert!(tree.is_empty());
}

// =============================================================================
// FIND_SPENDABLE_OUTPUTS TESTS
// =============================================================================

#[test]
fn test_find_spendable_outputs_empty_utxo_set() {
    let test_name = "find_spendable_outputs_empty";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    let pub_key_hash = vec![1, 2, 3, 4, 5];
    let amount = 100;
    
    let (accumulated, outputs) = utxo_set.find_spendable_outputs(&pub_key_hash, amount);
    
    assert_eq!(accumulated, 0);
    assert!(outputs.is_empty());
}

#[test]
fn test_find_spendable_outputs_sufficient_balance() {
    let test_name = "find_spendable_outputs_sufficient";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    // Create and add UTXOs to the database
    let pub_key_hash = vec![1, 2, 3, 4, 5];
    let outputs = vec![
        TXOutput { value: 50, pub_key_hash: pub_key_hash.clone() },
        TXOutput { value: 30, pub_key_hash: pub_key_hash.clone() },
        TXOutput { value: 20, pub_key_hash: pub_key_hash.clone() },
    ];
    
    let txid = vec![1, 2, 3, 4];
    add_utxos_to_db(&utxo_set, &txid, &outputs);
    
    let (accumulated, spendable_outputs) = utxo_set.find_spendable_outputs(&pub_key_hash, 70);
    
    assert_eq!(accumulated, 80); // Should accumulate 50 + 30, then stop (80 >= 70)
    assert_eq!(spendable_outputs.len(), 1);
    
    let txid_hex = data_encoding::HEXLOWER.encode(&txid);
    assert!(spendable_outputs.contains_key(&txid_hex));
    assert_eq!(spendable_outputs.get(&txid_hex).unwrap().len(), 2); // Only first two outputs
}

#[test]
fn test_find_spendable_outputs_insufficient_balance() {
    let test_name = "find_spendable_outputs_insufficient";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    let pub_key_hash = vec![1, 2, 3, 4, 5];
    let outputs = vec![
        TXOutput { value: 30, pub_key_hash: pub_key_hash.clone() },
        TXOutput { value: 20, pub_key_hash: pub_key_hash.clone() },
    ];
    
    let txid = vec![1, 2, 3, 4];
    add_utxos_to_db(&utxo_set, &txid, &outputs);
    
    let (accumulated, spendable_outputs) = utxo_set.find_spendable_outputs(&pub_key_hash, 100);
    
    assert_eq!(accumulated, 50); // Only 30 + 20 available
    assert_eq!(spendable_outputs.len(), 1);
}

#[test]
fn test_find_spendable_outputs_wrong_key_hash() {
    let test_name = "find_spendable_outputs_wrong_key";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    let pub_key_hash = vec![1, 2, 3, 4, 5];
    let different_key_hash = vec![6, 7, 8, 9, 10];
    let outputs = vec![
        TXOutput { value: 50, pub_key_hash: different_key_hash.clone() },
    ];
    
    let txid = vec![1, 2, 3, 4];
    add_utxos_to_db(&utxo_set, &txid, &outputs);
    
    let (accumulated, spendable_outputs) = utxo_set.find_spendable_outputs(&pub_key_hash, 50);
    
    assert_eq!(accumulated, 0);
    assert!(spendable_outputs.is_empty());
}

#[test]
fn test_find_spendable_outputs_multiple_transactions() {
    let test_name = "find_spendable_outputs_multiple_tx";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    let pub_key_hash = vec![1, 2, 3, 4, 5];
    
    // Add UTXOs from first transaction
    let txid1 = vec![1, 1, 1, 1];
    let outputs1 = vec![
        TXOutput { value: 40, pub_key_hash: pub_key_hash.clone() },
    ];
    add_utxos_to_db(&utxo_set, &txid1, &outputs1);
    
    // Add UTXOs from second transaction
    let txid2 = vec![2, 2, 2, 2];
    let outputs2 = vec![
        TXOutput { value: 35, pub_key_hash: pub_key_hash.clone() },
    ];
    add_utxos_to_db(&utxo_set, &txid2, &outputs2);
    
    let (accumulated, spendable_outputs) = utxo_set.find_spendable_outputs(&pub_key_hash, 60);
    
    assert_eq!(accumulated, 75); // 40 + 35
    assert_eq!(spendable_outputs.len(), 2);
    
    let txid1_hex = data_encoding::HEXLOWER.encode(&txid1);
    let txid2_hex = data_encoding::HEXLOWER.encode(&txid2);
    assert!(spendable_outputs.contains_key(&txid1_hex));
    assert!(spendable_outputs.contains_key(&txid2_hex));
}

// =============================================================================
// FIND_UTXO TESTS
// =============================================================================

#[test]
fn test_find_utxo_empty_set() {
    let test_name = "find_utxo_empty";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    let pub_key_hash = vec![1, 2, 3, 4, 5];
    let utxos = utxo_set.find_utxo(&pub_key_hash);
    
    assert!(utxos.is_empty());
}

#[test]
fn test_find_utxo_matching_outputs() {
    let test_name = "find_utxo_matching";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    let pub_key_hash = vec![1, 2, 3, 4, 5];
    let outputs = vec![
        TXOutput { value: 50, pub_key_hash: pub_key_hash.clone() },
        TXOutput { value: 30, pub_key_hash: pub_key_hash.clone() },
    ];
    
    let txid = vec![1, 2, 3, 4];
    add_utxos_to_db(&utxo_set, &txid, &outputs);
    
    let utxos = utxo_set.find_utxo(&pub_key_hash);
    
    assert_eq!(utxos.len(), 2);
    let values: Vec<i32> = utxos.iter().map(|o| o.value).collect();
    assert!(values.contains(&50));
    assert!(values.contains(&30));
}

#[test]
fn test_find_utxo_no_matching_key() {
    let test_name = "find_utxo_no_matching";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    let pub_key_hash = vec![1, 2, 3, 4, 5];
    let different_key_hash = vec![6, 7, 8, 9, 10];
    let outputs = vec![
        TXOutput { value: 50, pub_key_hash: different_key_hash },
    ];
    
    let txid = vec![1, 2, 3, 4];
    add_utxos_to_db(&utxo_set, &txid, &outputs);
    
    let utxos = utxo_set.find_utxo(&pub_key_hash);
    
    assert!(utxos.is_empty());
}

#[test]
fn test_find_utxo_mixed_key_hashes() {
    let test_name = "find_utxo_mixed";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    let target_key_hash = vec![1, 2, 3, 4, 5];
    let other_key_hash = vec![6, 7, 8, 9, 10];
    let outputs = vec![
        TXOutput { value: 50, pub_key_hash: target_key_hash.clone() },
        TXOutput { value: 30, pub_key_hash: other_key_hash },
        TXOutput { value: 25, pub_key_hash: target_key_hash.clone() },
    ];
    
    let txid = vec![1, 2, 3, 4];
    add_utxos_to_db(&utxo_set, &txid, &outputs);
    
    let utxos = utxo_set.find_utxo(&target_key_hash);
    
    assert_eq!(utxos.len(), 2);
    let values: Vec<i32> = utxos.iter().map(|o| o.value).collect();
    assert!(values.contains(&50));
    assert!(values.contains(&25));
    assert!(!values.contains(&30)); // Should not include other key's output
}

// =============================================================================
// COUNT_TRANSACTIONS TESTS
// =============================================================================

#[test]
fn test_count_transactions_empty() {
    let test_name = "count_transactions_empty";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    let count = utxo_set.count_transactions();
    assert_eq!(count, 0);
}

#[test]
fn test_count_transactions_single() {
    let test_name = "count_transactions_single";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    let outputs = vec![TXOutput { value: 50, pub_key_hash: vec![1, 2, 3] }];
    let txid = vec![1, 2, 3, 4];
    add_utxos_to_db(&utxo_set, &txid, &outputs);
    
    let count = utxo_set.count_transactions();
    assert_eq!(count, 1);
}

#[test]
fn test_count_transactions_multiple() {
    let test_name = "count_transactions_multiple";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    // Add multiple transactions
    for i in 0..5 {
        let outputs = vec![TXOutput { value: 50, pub_key_hash: vec![1, 2, 3] }];
        let txid = vec![i, i, i, i];
        add_utxos_to_db(&utxo_set, &txid, &outputs);
    }
    
    let count = utxo_set.count_transactions();
    assert_eq!(count, 5);
}

// =============================================================================
// REINDEX TESTS
// =============================================================================

#[test]
fn test_reindex_empty_blockchain() {
    let test_name = "reindex_empty_blockchain";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    // Add some UTXOs to the UTXO set first
    let outputs = vec![TXOutput { value: 50, pub_key_hash: vec![1, 2, 3] }];
    let txid = vec![1, 2, 3, 4];
    add_utxos_to_db(&utxo_set, &txid, &outputs);
    
    // Verify there's one transaction before reindex
    assert_eq!(utxo_set.count_transactions(), 1);
    
    // Reindex (should clear everything since blockchain is empty)
    utxo_set.reindex();
    
    // Should be empty after reindex
    assert_eq!(utxo_set.count_transactions(), 0);
}

#[test]
fn test_reindex_with_blockchain_data() {
    let test_name = "reindex_with_data";
    let test_db = TestDatabase::new(test_name);
    
    // Create blockchain with some transactions
    let coinbase_tx = create_coinbase_transaction(50, vec![1, 2, 3, 4]);
    let genesis_block = Block::generate_genesis_block(&coinbase_tx);
    
    // Set up blockchain
    let blocks_tree = test_db.get_db().open_tree(BLOCKS_TREE).unwrap();
    blocks_tree.insert(genesis_block.get_hash(), genesis_block.serialize()).unwrap();
    blocks_tree.insert(TIP_BLOCK_HASH_KEY, genesis_block.get_hash()).unwrap();
    
    let blockchain = Blockchain::new_with_tip(
        test_db.get_db().clone(),
        genesis_block.get_hash().to_string(),
    );
    let utxo_set = UTXOSet::new(blockchain);
    
    // Initially empty UTXO set
    assert_eq!(utxo_set.count_transactions(), 0);
    
    // Reindex from blockchain
    utxo_set.reindex();
    
    // Should now have one transaction from the genesis block
    assert_eq!(utxo_set.count_transactions(), 1);
    
    // Verify the UTXO data is correct
    let utxos = utxo_set.find_utxo(&[1, 2, 3, 4]);
    assert_eq!(utxos.len(), 1);
    assert_eq!(utxos[0].value, 50);
}

// =============================================================================
// UPDATE TESTS
// =============================================================================

#[test]
fn test_update_with_coinbase_transaction() {
    let test_name = "update_coinbase";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    // Create a block with a coinbase transaction
    let coinbase_tx = create_coinbase_transaction(50, vec![1, 2, 3, 4]);
    let block = Block::generate_genesis_block(&coinbase_tx);
    
    // Update UTXO set with the block
    utxo_set.update(&block);
    
    // Should have added one transaction
    assert_eq!(utxo_set.count_transactions(), 1);
    
    // Verify the UTXO was added
    let utxos = utxo_set.find_utxo(&[1, 2, 3, 4]);
    assert_eq!(utxos.len(), 1);
    assert_eq!(utxos[0].value, 50);
}

#[test]
fn test_update_with_spending_transaction() {
    let test_name = "update_spending";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    // First, create and add a UTXO to spend
    let initial_outputs = vec![
        TXOutput { value: 100, pub_key_hash: vec![1, 2, 3] }
    ];
    let initial_txid = vec![9, 9, 9, 9];
    add_utxos_to_db(&utxo_set, &initial_txid, &initial_outputs);
    
    // Create a spending transaction
    let spending_tx = create_spending_transaction(
        vec![(initial_txid.clone(), 0)], // Spend the initial UTXO
        vec![(60, vec![4, 5, 6]), (40, vec![7, 8, 9])], // Create two new outputs
    );
    
    let block = Block::new_block_without_proof_of_work(
        "prev_hash".to_string(),
        std::slice::from_ref(&spending_tx),
        1,
    );
    
    // Update UTXO set with the spending transaction
    utxo_set.update(&block);
    
    // Should have two transactions now (original was removed, new one added)
    assert_eq!(utxo_set.count_transactions(), 1);
    
    // Original UTXO should be gone
    let original_utxos = utxo_set.find_utxo(&[1, 2, 3]);
    assert!(original_utxos.is_empty());
    
    // New UTXOs should exist
    let new_utxos1 = utxo_set.find_utxo(&[4, 5, 6]);
    assert_eq!(new_utxos1.len(), 1);
    assert_eq!(new_utxos1[0].value, 60);
    
    let new_utxos2 = utxo_set.find_utxo(&[7, 8, 9]);
    assert_eq!(new_utxos2.len(), 1);
    assert_eq!(new_utxos2[0].value, 40);
}

#[test]
fn test_update_partial_spending() {
    let test_name = "update_partial_spending";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    // Create a transaction with multiple outputs
    let initial_outputs = vec![
        TXOutput { value: 50, pub_key_hash: vec![1, 1, 1] },
        TXOutput { value: 30, pub_key_hash: vec![2, 2, 2] },
        TXOutput { value: 20, pub_key_hash: vec![3, 3, 3] },
    ];
    let initial_txid = vec![8, 8, 8, 8];
    add_utxos_to_db(&utxo_set, &initial_txid, &initial_outputs);
    
    // Create a transaction that only spends one of the outputs
    let spending_tx = create_spending_transaction(
        vec![(initial_txid.clone(), 1)], // Only spend output at index 1 (value 30)
        vec![(25, vec![4, 4, 4]), (5, vec![5, 5, 5])], // Split into two new outputs
    );
    
    let block = Block::new_block_without_proof_of_work(
        "prev_hash".to_string(),
        std::slice::from_ref(&spending_tx),
        1,
    );
    
    // Update UTXO set
    utxo_set.update(&block);
    
    // Should have two transactions (original with remaining outputs, new spending tx)
    assert_eq!(utxo_set.count_transactions(), 2);
    
    // Output at index 1 should be gone, but outputs at indices 0 and 2 should remain
    let utxos1 = utxo_set.find_utxo(&[1, 1, 1]);
    assert_eq!(utxos1.len(), 1);
    assert_eq!(utxos1[0].value, 50);
    
    let utxos2 = utxo_set.find_utxo(&[2, 2, 2]);
    assert!(utxos2.is_empty()); // This was spent
    
    let utxos3 = utxo_set.find_utxo(&[3, 3, 3]);
    assert_eq!(utxos3.len(), 1);
    assert_eq!(utxos3[0].value, 20);
    
    // New outputs should exist
    let new_utxos1 = utxo_set.find_utxo(&[4, 4, 4]);
    assert_eq!(new_utxos1.len(), 1);
    assert_eq!(new_utxos1[0].value, 25);
    
    let new_utxos2 = utxo_set.find_utxo(&[5, 5, 5]);
    assert_eq!(new_utxos2.len(), 1);
    assert_eq!(new_utxos2[0].value, 5);
}

#[test]
fn test_update_complete_spending() {
    let test_name = "update_complete_spending";
    let test_db = TestDatabase::new(test_name);
    let blockchain = Blockchain::new_with_empty_tip(test_db.get_db().clone());
    let utxo_set = UTXOSet::new(blockchain);
    
    // Create a transaction with a single output
    let initial_outputs = vec![
        TXOutput { value: 100, pub_key_hash: vec![1, 2, 3] },
    ];
    let initial_txid = vec![7, 7, 7, 7];
    add_utxos_to_db(&utxo_set, &initial_txid, &initial_outputs);
    
    // Create a transaction that spends the only output
    let spending_tx = create_spending_transaction(
        vec![(initial_txid.clone(), 0)], // Spend the only output
        vec![(100, vec![4, 5, 6])], // Create one new output
    );
    
    let block = Block::new_block_without_proof_of_work(
        "prev_hash".to_string(),
        std::slice::from_ref(&spending_tx),
        1,
    );
    
    // Update UTXO set
    utxo_set.update(&block);
    
    // Should have one transaction (the original was completely spent and removed)
    assert_eq!(utxo_set.count_transactions(), 1);
    
    // Original UTXO should be gone
    let original_utxos = utxo_set.find_utxo(&[1, 2, 3]);
    assert!(original_utxos.is_empty());
    
    // New UTXO should exist
    let new_utxos = utxo_set.find_utxo(&[4, 5, 6]);
    assert_eq!(new_utxos.len(), 1);
    assert_eq!(new_utxos[0].value, 100);
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Helper function to add UTXOs directly to the database for testing
fn add_utxos_to_db(utxo_set: &UTXOSet, txid: &[u8], outputs: &[TXOutput]) {
    use bincode::config::standard;
    
    let db = utxo_set.get_blockchain().get_db();
    let utxo_tree = db.open_tree(rust_blockchain::utxo_set::UTXO_TREE).unwrap();
    
    let encoded_outputs = bincode::encode_to_vec(outputs, standard()).unwrap();
    utxo_tree.insert(txid, encoded_outputs).unwrap();
} 
