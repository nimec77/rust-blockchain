use std::sync::{Arc, RwLock};

use sled::Db;

use crate::{
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
}
