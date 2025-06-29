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
        let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        let data = block_tree.get(self.current_hash.clone()).unwrap();
        let block = Block::deserialize(data.as_ref()?.to_vec().as_slice());
        self.current_hash = block.get_pre_block_hash().to_string();

        Some(block)
    }
}
