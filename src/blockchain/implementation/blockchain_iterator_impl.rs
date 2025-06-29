use sled::Db;

use crate::{BLOCKS_TREE, Block, blockchain::BlockchainIterator};

impl BlockchainIterator {
    pub fn new(db: Db, current_hash: String) -> Self {
        Self { db, current_hash }
    }
}

impl Iterator for BlockchainIterator {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if we've reached the end marker
        if self.current_hash == "\0" {
            return None;
        }
        
        let block_tree = self.db.open_tree(BLOCKS_TREE).ok()?;
        let data = block_tree.get(&self.current_hash).ok()??;
        let block = Block::try_deserialize(&data).ok()?;
        
        // Update current_hash to the previous block's hash for next iteration
        let next_hash = block.get_pre_block_hash().to_string();
        
        // Check if we've reached the end (empty pre_block_hash means we're done)
        if next_hash.is_empty() {
            // Mark that we've reached the end for the next call
            self.current_hash = "\0".to_string(); // Use null character as end marker
        } else {
            self.current_hash = next_hash;
        }
        
        Some(block)
    }
}
