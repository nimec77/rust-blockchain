use sled::Db;

pub struct BlockchainIterator {
    pub(crate) db: Db,
    pub(crate) current_hash: String,
}
