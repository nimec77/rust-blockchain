use data_encoding::HEXLOWER;
use num_bigint::{BigInt, Sign};

use crate::{
    block::Block,
    common::BincodeBigInt,
    proof_of_work::{MAX_NONCE, ProofOfWork, TARGET_BITS},
    util,
};

impl ProofOfWork {
    /// Create a new proof-of-work instance for the given block
    pub fn new_proof_of_work(block: Block) -> ProofOfWork {
        // Calculate target: 1 << (256 - TARGET_BITS)
        let target_value = BigInt::from(1) << (256 - TARGET_BITS);
        let target = BincodeBigInt::new(target_value);

        ProofOfWork { block, target }
    }

    pub fn get_block(&self) -> &Block {
        &self.block
    }

    pub fn get_target(&self) -> &BincodeBigInt {
        &self.target
    }

    /// Prepare data for hashing by combining block fields with nonce
    pub fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let mut data = Vec::new();

        // Combine block data with nonce
        data.extend_from_slice(self.block.get_pre_block_hash().as_bytes());
        data.extend_from_slice(&self.block.hash_transactions());
        data.extend_from_slice(&self.block.get_timestamp().to_be_bytes());
        data.extend_from_slice(&(TARGET_BITS as u64).to_be_bytes());
        data.extend_from_slice(&nonce.to_be_bytes());

        data
    }

    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash = Vec::new();
        println!("Mining the block");
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = util::sha256_digest(data.as_slice());
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());

            if hash_int < *self.target.as_bigint() {
                println!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
            }
        }
        println!();
        (nonce, HEXLOWER.encode(hash.as_slice()))
    }

    /// Validate that a block's hash satisfies the proof-of-work requirement
    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.get_nonce());
        let hash = util::sha256_digest(data.as_slice());
        let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());

        hash_int < *self.target.as_bigint()
    }
}
