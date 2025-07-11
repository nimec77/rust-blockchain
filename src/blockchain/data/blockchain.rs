use std::sync::{Arc, RwLock};

use sled::Db;

pub const TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";
pub const BLOCKS_TREE: &str = "blocks";


#[derive(Clone)]
pub struct Blockchain {
    pub(in crate::blockchain) tip_hash: Arc<RwLock<String>>, // Optimized: Arc<str> instead of String
    pub(in crate::blockchain) db: Db,
}
