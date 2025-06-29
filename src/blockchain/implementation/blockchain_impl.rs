use std::sync::{Arc, RwLock};

use sled::Db;

use crate::{
    blockchain::data::blockchain::{BLOCKS_TREE, Blockchain, TIP_BLOCK_HASH_KEY},
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn create_test_db(test_name: &str) -> Db {
        let test_path = format!("test_db_{}_{}", test_name, std::process::id());
        sled::open(&test_path).unwrap()
    }

    fn cleanup_test_db(test_path: &str) {
        if Path::new(test_path).exists() {
            let _ = fs::remove_dir_all(test_path);
        }
    }

    #[test]
    fn test_new_with_tip() {
        let test_name = "new_with_tip";
        let test_db = create_test_db(test_name);
        let test_tip_hash = "test_hash_12345".to_string();

        let blockchain = Blockchain::new_with_tip(test_db, test_tip_hash.clone());

        assert_eq!(blockchain.get_tip_hash(), test_tip_hash);
        // Verify we can access the database
        let _ = blockchain.get_db().open_tree("test_tree").unwrap();

        cleanup_test_db(&format!("test_db_{}_{}", test_name, std::process::id()));
    }

    #[test]
    fn test_new_with_empty_tip() {
        let test_name = "new_with_empty_tip";
        let test_db = create_test_db(test_name);

        let blockchain = Blockchain::new_with_empty_tip(test_db);

        assert_eq!(blockchain.get_tip_hash(), String::new());
        // Verify we can access the database
        let _ = blockchain.get_db().open_tree("test_tree").unwrap();

        cleanup_test_db(&format!("test_db_{}_{}", test_name, std::process::id()));
    }

    #[test]
    fn test_get_db() {
        let test_name = "get_db";
        let test_db = create_test_db(test_name);
        let blockchain = Blockchain::new_with_empty_tip(test_db);

        let db_ref = blockchain.get_db();
        // Verify we can use the database reference
        let _ = db_ref.open_tree("test_tree").unwrap();

        cleanup_test_db(&format!("test_db_{}_{}", test_name, std::process::id()));
    }

    #[test]
    fn test_get_tip_hash() {
        let test_name = "get_tip_hash";
        let test_db = create_test_db(test_name);
        let test_tip_hash = "abcdef123456".to_string();
        let blockchain = Blockchain::new_with_tip(test_db, test_tip_hash.clone());

        let retrieved_hash = blockchain.get_tip_hash();
        assert_eq!(retrieved_hash, test_tip_hash);

        cleanup_test_db(&format!("test_db_{}_{}", test_name, std::process::id()));
    }

    #[test]
    fn test_set_tip_hash() {
        let test_name = "set_tip_hash";
        let test_db = create_test_db(test_name);
        let initial_hash = "initial_hash".to_string();
        let blockchain = Blockchain::new_with_tip(test_db, initial_hash.clone());

        // Verify initial hash
        assert_eq!(blockchain.get_tip_hash(), initial_hash);

        // Set new hash
        let new_hash = "new_hash_654321";
        blockchain.set_tip_hash(new_hash);

        // Verify hash was updated
        assert_eq!(blockchain.get_tip_hash(), new_hash);

        cleanup_test_db(&format!("test_db_{}_{}", test_name, std::process::id()));
    }

    #[test]
    fn test_set_tip_hash_multiple_updates() {
        let test_name = "set_tip_hash_multiple";
        let test_db = create_test_db(test_name);
        let blockchain = Blockchain::new_with_empty_tip(test_db);

        // Test multiple updates
        let hashes = vec!["hash1", "hash2", "hash3", "final_hash"];

        for hash in &hashes {
            blockchain.set_tip_hash(hash);
            assert_eq!(blockchain.get_tip_hash(), *hash);
        }

        cleanup_test_db(&format!("test_db_{}_{}", test_name, std::process::id()));
    }

    #[test]
    fn test_blockchain_clone() {
        let test_name = "blockchain_clone";
        let test_db = create_test_db(test_name);
        let test_tip_hash = "cloneable_hash".to_string();
        let blockchain = Blockchain::new_with_tip(test_db, test_tip_hash.clone());

        // Test that Blockchain can be cloned
        let cloned_blockchain = blockchain.clone();

        // Both should have the same tip hash
        assert_eq!(blockchain.get_tip_hash(), cloned_blockchain.get_tip_hash());

        // Updating one should update both (shared Arc<RwLock>)
        blockchain.set_tip_hash("updated_hash");
        assert_eq!(blockchain.get_tip_hash(), "updated_hash");
        assert_eq!(cloned_blockchain.get_tip_hash(), "updated_hash");

        cleanup_test_db(&format!("test_db_{}_{}", test_name, std::process::id()));
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;
        use std::time::Duration;

        let test_name = "thread_safety";
        let test_db = create_test_db(test_name);
        let blockchain = Blockchain::new_with_empty_tip(test_db);
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

        cleanup_test_db(&format!("test_db_{}_{}", test_name, std::process::id()));
    }
}
