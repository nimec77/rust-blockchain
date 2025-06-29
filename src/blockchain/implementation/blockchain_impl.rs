use std::sync::{Arc, RwLock};

use sled::{Db, Tree, transaction::TransactionResult};

use crate::{
    Block, Transaction,
    blockchain::{BLOCKS_TREE, Blockchain, TIP_BLOCK_HASH_KEY},
    util,
};

impl Blockchain {
    pub fn new_blockchain() -> Blockchain {
        let db = sled::open(util::current_dir().join("data")).unwrap();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
        let tip_bytes = blocks_tree
            .get(TIP_BLOCK_HASH_KEY)
            .unwrap()
            .expect("No existing blockchain found. Create one first.");
        let tip_hash = String::from_utf8(tip_bytes.to_vec()).unwrap();
        Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
        }
    }

    pub fn new_with_tip(db: Db, tip_hash: String) -> Self {
        Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
        }
    }

    pub fn new_with_empty_tip(db: Db) -> Self {
        Blockchain {
            tip_hash: Arc::new(RwLock::new(String::new())),
            db,
        }
    }

    pub fn get_db(&self) -> &Db {
        &self.db
    }

    pub fn get_tip_hash(&self) -> String {
        self.tip_hash.read().unwrap().clone()
    }

    pub fn set_tip_hash(&self, new_tip_hash: &str) {
        let mut tip_hash = self.tip_hash.write().unwrap();
        *tip_hash = new_tip_hash.to_string();
    }

    pub fn get_best_height(&self) -> usize {
        let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        let tip_block_bytes = block_tree
            .get(self.get_tip_hash())
            .unwrap()
            .expect("The tip hash is valid");
        let tip_block = Block::deserialize(tip_block_bytes.as_ref());
        tip_block.get_height()
    }

    fn update_blocks_tree(blocks_tree: &Tree, block: &Block) {
        let block_hash = block.get_hash();
        let _: TransactionResult<(), ()> = blocks_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block_hash, block.clone());
            let _ = tx_db.insert(TIP_BLOCK_HASH_KEY, block_hash);
            Ok(())
        });
    }

    pub fn mine_block(&self, transactions: &[Transaction]) -> Block {
        for transaction in transactions {
            if !transaction.verify(self) {
                panic!("ERROR: Invalid transaction")
            }
        }
        let best_height = self.get_best_height();

        let block = Block::new_block(self.get_tip_hash(), transactions, best_height + 1);
        let block_hash = block.get_hash();

        let blocks_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        Self::update_blocks_tree(&blocks_tree, &block);
        self.set_tip_hash(block_hash);

        block
    }

    pub fn add_block(&self, block: &Block) {
        let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        if block_tree.get(block.get_hash()).unwrap().is_some() {
            return;
        }
        let _: TransactionResult<(), ()> = block_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block.get_hash(), block.serialize()).unwrap();

            let tip_block_bytes = tx_db
                .get(self.get_tip_hash())
                .unwrap()
                .expect("The tip hash is not valid");
            let tip_block = Block::deserialize(tip_block_bytes.as_ref());
            if block.get_height() > tip_block.get_height() {
                let _ = tx_db.insert(TIP_BLOCK_HASH_KEY, block.get_hash()).unwrap();
                self.set_tip_hash(block.get_hash());
            }
            Ok(())
        });
    }
}
