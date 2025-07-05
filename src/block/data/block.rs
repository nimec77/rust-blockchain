use crate::transaction::Transaction;


#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct Block {
    pub(in crate::block) timestamp: i64,
    pub(in crate::block) pre_block_hash: String,
    pub(in crate::block) hash: String,
    pub(in crate::block) transactions: Vec<Transaction>,
    pub(in crate::block) nonce: i64,
    pub(in crate::block) height: usize,
}

