use sled::Db;

pub struct BlockchainIterator {
    pub(in crate::blockchain) db: Db,
    pub(in crate::blockchain) current_hash: String,
}
