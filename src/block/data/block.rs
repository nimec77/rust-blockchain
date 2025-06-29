use crate::transaction::Transaction;


#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct Block {
    pub timestamp: i64,
    pub pre_block_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
    pub nonce: i64,
    pub height: usize,
}

