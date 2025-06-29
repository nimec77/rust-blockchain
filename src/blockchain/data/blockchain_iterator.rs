use sled::Db;

pub struct BlockchainIterator {
    pub db: Db,
    pub current_hash: String,
}
