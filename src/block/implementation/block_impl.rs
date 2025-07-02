use bincode::config::standard;
use sled::IVec;

use crate::{block::Block, proof_of_work::ProofOfWork, transaction::Transaction, util};

impl Block {
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: util::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;

        block
    }

    pub fn new_block_without_proof_of_work(
        pre_block_hash: String,
        transactions: &[Transaction],
        height: usize,
    ) -> Block {
        Block {
            timestamp: util::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, standard()).unwrap()
    }

    pub fn deserialize(bytes: &[u8]) -> Block {
        let (blk, _) = bincode::decode_from_slice(bytes, standard()).unwrap();

        blk
    }

    pub fn try_deserialize(bytes: &[u8]) -> Result<Block, bincode::error::DecodeError> {
        let (blk, _) = bincode::decode_from_slice(bytes, standard())?;

        Ok(blk)
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }

    pub fn set_transactions_for_test(&mut self, transactions: &[Transaction]) {
        self.transactions = transactions.to_vec();
    }

    pub fn get_pre_block_hash(&self) -> &str {
        self.pre_block_hash.as_str()
    }

    pub fn get_hash(&self) -> &str {
        self.hash.as_str()
    }

    pub fn set_hash_for_test(&mut self, hash: &str) {
        self.hash = hash.to_string();
    }

    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_nonce(&self) -> i64 {
        self.nonce
    }

    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id());
        }
        util::sha256_digest(txhashs.as_slice())
    }

    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];

        Block::new_block_without_proof_of_work(String::from("None"), &transactions, 0)
    }
}

impl From<Block> for IVec {
    fn from(b: Block) -> Self {
        let bytes = b.serialize();
        IVec::from(bytes)
    }
}
