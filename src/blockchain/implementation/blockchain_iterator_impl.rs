use sled::Db;

use crate::{BLOCKS_TREE, Block, blockchain::BlockchainIterator};

impl BlockchainIterator {
    pub fn new(db: Db, current_hash: String) -> Self {
        Self { db, current_hash }
    }

    pub fn next(&mut self) -> Option<Block> {
        let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        let data = block_tree.get(self.current_hash.clone()).unwrap();
        let block = Block::deserialize(data.as_ref()?.to_vec().as_slice());
        self.current_hash = block.get_pre_block_hash().to_string();

        Some(block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::{TXInput, TXOutput, Transaction};
    use tempfile::TempDir;

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

    fn create_test_block(pre_hash: String, height: usize) -> Block {
        let transaction = create_test_transaction(vec![1, 2, 3, 4]);
        let transactions = vec![transaction];
        Block::new_block_without_proof_of_work(pre_hash, &transactions, height)
    }

    fn setup_test_db() -> (Db, TempDir) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let db = sled::open(temp_dir.path()).expect("Failed to open database");
        (db, temp_dir)
    }

    #[test]
    fn test_new() {
        let (db, _temp_dir) = setup_test_db();
        let current_hash = "test_hash".to_string();

        let iterator = BlockchainIterator::new(db.clone(), current_hash.clone());

        assert_eq!(iterator.current_hash, current_hash);
        // Note: We can't directly test db equality since Db doesn't implement PartialEq
        // but we can test that the iterator was created successfully
    }

    #[test]
    fn test_next_with_valid_block() {
        let (db, _temp_dir) = setup_test_db();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();

        // Create a test block
        let mut block = create_test_block("previous_hash".to_string(), 1);
        block.hash = "current_hash".to_string();

        // Store the block in the database
        blocks_tree
            .insert("current_hash", block.serialize())
            .unwrap();

        // Create iterator
        let mut iterator = BlockchainIterator::new(db, "current_hash".to_string());

        // Test next()
        let result = iterator.next();

        assert!(result.is_some());
        let retrieved_block = result.unwrap();
        assert_eq!(retrieved_block.get_hash(), "current_hash");
        assert_eq!(retrieved_block.get_pre_block_hash(), "previous_hash");
        assert_eq!(retrieved_block.get_height(), 1);

        // Verify that current_hash was updated to the previous block's hash
        assert_eq!(iterator.current_hash, "previous_hash");
    }

    #[test]
    fn test_next_with_nonexistent_block() {
        let (db, _temp_dir) = setup_test_db();
        let mut iterator = BlockchainIterator::new(db, "nonexistent_hash".to_string());

        // This should return None since the block doesn't exist
        let result = iterator.next();
        assert!(result.is_none());
    }

    #[test]
    fn test_next_iterating_multiple_blocks() {
        let (db, _temp_dir) = setup_test_db();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();

        // Create a chain of blocks: genesis -> block1 -> block2
        let mut genesis_block = create_test_block("".to_string(), 0);
        genesis_block.hash = "genesis_hash".to_string();

        let mut block1 = create_test_block("genesis_hash".to_string(), 1);
        block1.hash = "block1_hash".to_string();

        let mut block2 = create_test_block("block1_hash".to_string(), 2);
        block2.hash = "block2_hash".to_string();

        // Store blocks in database
        blocks_tree
            .insert("genesis_hash", genesis_block.serialize())
            .unwrap();
        blocks_tree
            .insert("block1_hash", block1.serialize())
            .unwrap();
        blocks_tree
            .insert("block2_hash", block2.serialize())
            .unwrap();

        // Create iterator starting from the latest block
        let mut iterator = BlockchainIterator::new(db, "block2_hash".to_string());

        // First iteration should get block2
        let result1 = iterator.next();
        assert!(result1.is_some());
        let retrieved_block1 = result1.unwrap();
        assert_eq!(retrieved_block1.get_hash(), "block2_hash");
        assert_eq!(retrieved_block1.get_height(), 2);
        assert_eq!(iterator.current_hash, "block1_hash");

        // Second iteration should get block1
        let result2 = iterator.next();
        assert!(result2.is_some());
        let retrieved_block2 = result2.unwrap();
        assert_eq!(retrieved_block2.get_hash(), "block1_hash");
        assert_eq!(retrieved_block2.get_height(), 1);
        assert_eq!(iterator.current_hash, "genesis_hash");

        // Third iteration should get genesis block
        let result3 = iterator.next();
        assert!(result3.is_some());
        let retrieved_block3 = result3.unwrap();
        assert_eq!(retrieved_block3.get_hash(), "genesis_hash");
        assert_eq!(retrieved_block3.get_height(), 0);
        assert_eq!(iterator.current_hash, "");
    }

    #[test]
    fn test_next_with_empty_database() {
        let (db, _temp_dir) = setup_test_db();
        let mut iterator = BlockchainIterator::new(db, "any_hash".to_string());

        // Should return None for empty database
        let result = iterator.next();
        assert!(result.is_none());
    }

    #[test]
    fn test_iterator_state_changes_after_next() {
        let (db, _temp_dir) = setup_test_db();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();

        // Create a test block
        let mut block = create_test_block("previous_hash".to_string(), 1);
        block.hash = "current_hash".to_string();

        // Store the block
        blocks_tree
            .insert("current_hash", block.serialize())
            .unwrap();

        let mut iterator = BlockchainIterator::new(db, "current_hash".to_string());

        // Check initial state
        assert_eq!(iterator.current_hash, "current_hash");

        // Call next() and verify state change
        let _result = iterator.next();
        assert_eq!(iterator.current_hash, "previous_hash");
    }

    #[test]
    fn test_iterator_with_corrupted_data() {
        let (db, _temp_dir) = setup_test_db();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();

        // Insert corrupted data that can't be deserialized as a Block
        blocks_tree
            .insert("corrupted_hash", b"invalid_block_data")
            .unwrap();

        let mut iterator = BlockchainIterator::new(db, "corrupted_hash".to_string());

        // This should panic due to deserialization error
        // We expect the deserialize call to fail
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| iterator.next()));

        assert!(result.is_err());
    }

    #[test]
    fn test_iterator_with_different_hash_formats() {
        let (db, _temp_dir) = setup_test_db();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();

        // Test with different hash formats
        let hash_formats = [
            "0000abc123",
            "ABC123DEF456",
            "a1b2c3d4e5f6",
            "123456789abcdef0",
        ];

        for (i, hash) in hash_formats.iter().enumerate() {
            let mut block = create_test_block(format!("prev_{i}"), i);
            block.hash = hash.to_string();
            blocks_tree
                .insert(hash.as_bytes(), block.serialize())
                .unwrap();

            let mut iterator = BlockchainIterator::new(db.clone(), hash.to_string());
            let result = iterator.next();

            assert!(result.is_some());
            let retrieved_block = result.unwrap();
            assert_eq!(retrieved_block.get_hash(), *hash);
        }
    }

    #[test]
    fn test_iterator_database_operations() {
        let (db, _temp_dir) = setup_test_db();

        // Test that iterator can be created with different databases
        let iterator1 = BlockchainIterator::new(db.clone(), "hash1".to_string());
        let iterator2 = BlockchainIterator::new(db.clone(), "hash2".to_string());

        assert_eq!(iterator1.current_hash, "hash1");
        assert_eq!(iterator2.current_hash, "hash2");
    }

    #[test]
    fn test_iterator_with_single_block_chain() {
        let (db, _temp_dir) = setup_test_db();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();

        // Create a single genesis block
        let mut genesis_block = create_test_block("".to_string(), 0);
        genesis_block.hash = "genesis_hash".to_string();

        blocks_tree
            .insert("genesis_hash", genesis_block.serialize())
            .unwrap();

        let mut iterator = BlockchainIterator::new(db, "genesis_hash".to_string());

        // Should successfully get the genesis block
        let result = iterator.next();
        assert!(result.is_some());
        let retrieved_block = result.unwrap();
        assert_eq!(retrieved_block.get_hash(), "genesis_hash");
        assert_eq!(retrieved_block.get_height(), 0);

        // current_hash should now be empty (genesis block's pre_block_hash)
        assert_eq!(iterator.current_hash, "");
    }
}
