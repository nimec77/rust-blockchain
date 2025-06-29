use crate::{block::Block, common::BincodeBigInt};

// Maximum number of nonce iterations to try
pub const MAX_NONCE: i64 = i64::MAX;

// Target difficulty - number of leading zeros in hash (adjustable)
pub const TARGET_BITS: usize = 24;

#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct ProofOfWork {
    pub block: Block,
    pub target: BincodeBigInt,
}
