use crate::transaction::Transaction;


#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct Block {
    pub(crate) timestamp: i64,
    pub(crate) pre_block_hash: String,
    pub(crate) hash: String,
    pub(crate) transactions: Vec<Transaction>,
    pub(crate) nonce: i64,
    pub(crate) height: usize,
}

