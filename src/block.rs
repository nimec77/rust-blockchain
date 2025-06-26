use bincode::config::standard;
use sled::IVec;

use crate::models::transaction::Transaction;

#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

impl Block {
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: crate::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };
        // let pow = ProofOfWork::new_proof_of_work(block.clone());
        // let (nonce, hash) = pow.run();
        // block.nonce = nonce;
        // block.hash = hash;

        block
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, standard()).unwrap()
    }

    pub fn deserialize(bytes: &[u8]) -> Block {
        let (blk, _) = bincode::decode_from_slice(bytes, standard()).unwrap();

        blk
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }

    pub fn get_pre_block_hash(&self) -> &str {
        self.pre_block_hash.as_str()
    }

    pub fn get_hash(&self) -> &str {
        self.hash.as_str()
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

    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id());
        }
        crate::sha256_digest(txhashs.as_slice())
    }

    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];

        Block::new_block(String::from("None"), &transactions, 0)
    }
}

impl From<Block> for IVec {
    fn from(b: Block) -> Self {
        let bytes = b.serialize();
        IVec::from(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{tx_input::TXInput, tx_output::TXOutput};

    fn create_test_transaction(id: Vec<u8>) -> Transaction {
        let tx_input = TXInput {
            txid: vec![1, 2, 3],
            vout: 0,
            signature: vec![4, 5, 6],
            pub_key: vec![7, 8, 9],
        };
        let tx_output = TXOutput {
            value: 100,
            pub_key_hash: vec![10, 11, 12],
        };
        
        Transaction {
            id,
            vin: vec![tx_input],
            vout: vec![tx_output],
        }
    }

    #[test]
    fn test_new_block() {
        let transaction = create_test_transaction(vec![1, 2, 3, 4]);
        let transactions = vec![transaction];
        let pre_block_hash = "previous_hash".to_string();
        let height = 1;

        let block = Block::new_block(pre_block_hash.clone(), &transactions, height);

        assert_eq!(block.pre_block_hash, pre_block_hash);
        assert_eq!(block.transactions.len(), 1);
        assert_eq!(block.height, height);
        assert_eq!(block.nonce, 0);
        assert_eq!(block.hash, String::new());
        assert!(block.timestamp > 0);
    }

    #[test]
    fn test_serialize_deserialize() {
        let transaction = create_test_transaction(vec![1, 2, 3, 4]);
        let transactions = vec![transaction];
        let pre_block_hash = "test_hash".to_string();
        let height = 5;

        let original_block = Block::new_block(pre_block_hash, &transactions, height);
        
        // Test serialization
        let serialized = original_block.serialize();
        assert!(!serialized.is_empty());

        // Test deserialization
        let deserialized_block = Block::deserialize(&serialized);
        
        assert_eq!(original_block.pre_block_hash, deserialized_block.pre_block_hash);
        assert_eq!(original_block.hash, deserialized_block.hash);
        assert_eq!(original_block.transactions.len(), deserialized_block.transactions.len());
        assert_eq!(original_block.nonce, deserialized_block.nonce);
        assert_eq!(original_block.height, deserialized_block.height);
        assert_eq!(original_block.timestamp, deserialized_block.timestamp);
    }

    #[test]
    fn test_get_transactions() {
        let transaction1 = create_test_transaction(vec![1, 2, 3]);
        let transaction2 = create_test_transaction(vec![4, 5, 6]);
        let transactions = vec![transaction1, transaction2];
        
        let block = Block::new_block("test_hash".to_string(), &transactions, 1);
        let retrieved_transactions = block.get_transactions();
        
        assert_eq!(retrieved_transactions.len(), 2);
        assert_eq!(retrieved_transactions[0].get_id(), &[1, 2, 3]);
        assert_eq!(retrieved_transactions[1].get_id(), &[4, 5, 6]);
    }

    #[test]
    fn test_get_pre_block_hash() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];
        let pre_block_hash = "previous_block_hash".to_string();
        
        let block = Block::new_block(pre_block_hash.clone(), &transactions, 1);
        
        assert_eq!(block.get_pre_block_hash(), pre_block_hash.as_str());
    }

    #[test]
    fn test_get_hash() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];
        
        let mut block = Block::new_block("test".to_string(), &transactions, 1);
        block.hash = "test_hash".to_string();
        
        assert_eq!(block.get_hash(), "test_hash");
    }

    #[test]
    fn test_get_hash_bytes() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];
        
        let mut block = Block::new_block("test".to_string(), &transactions, 1);
        block.hash = "test_hash".to_string();
        
        let hash_bytes = block.get_hash_bytes();
        assert_eq!(hash_bytes, "test_hash".as_bytes().to_vec());
    }

    #[test]
    fn test_get_timestamp() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];
        
        let block = Block::new_block("test".to_string(), &transactions, 1);
        let timestamp = block.get_timestamp();
        
        assert!(timestamp > 0);
        assert!(timestamp <= crate::current_timestamp());
    }

    #[test]
    fn test_get_height() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];
        let height = 42;
        
        let block = Block::new_block("test".to_string(), &transactions, height);
        
        assert_eq!(block.get_height(), height);
    }

    #[test]
    fn test_hash_transactions() {
        let transaction1 = create_test_transaction(vec![1, 2, 3]);
        let transaction2 = create_test_transaction(vec![4, 5, 6]);
        let transactions = vec![transaction1, transaction2];
        
        let block = Block::new_block("test".to_string(), &transactions, 1);
        let hash = block.hash_transactions();
        
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 32); // SHA256 produces 32 bytes
        
        // Test that the same transactions produce the same hash
        let block2 = Block::new_block("different_hash".to_string(), &transactions, 2);
        let hash2 = block2.hash_transactions();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_hash_transactions_empty() {
        let transactions = vec![];
        let block = Block::new_block("test".to_string(), &transactions, 1);
        let hash = block.hash_transactions();
        
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 32); // SHA256 produces 32 bytes even for empty input
    }

    #[test]
    fn test_generate_genesis_block() {
        let transaction = create_test_transaction(vec![1, 2, 3, 4]);
        
        let genesis_block = Block::generate_genesis_block(&transaction);
        
        assert_eq!(genesis_block.get_pre_block_hash(), "None");
        assert_eq!(genesis_block.get_height(), 0);
        assert_eq!(genesis_block.get_transactions().len(), 1);
        assert_eq!(genesis_block.get_transactions()[0].get_id(), &[1, 2, 3, 4]);
        assert!(genesis_block.get_timestamp() > 0);
    }

    #[test]
    fn test_from_block_to_ivec() {
        let transaction = create_test_transaction(vec![1, 2, 3]);
        let transactions = vec![transaction];
        
        let block = Block::new_block("test".to_string(), &transactions, 1);
        let ivec: IVec = block.clone().into();
        
        // Verify that we can deserialize back to the original block
        let deserialized_block = Block::deserialize(&ivec);
        
        assert_eq!(block.pre_block_hash, deserialized_block.pre_block_hash);
        assert_eq!(block.hash, deserialized_block.hash);
        assert_eq!(block.height, deserialized_block.height);
        assert_eq!(block.timestamp, deserialized_block.timestamp);
        assert_eq!(block.nonce, deserialized_block.nonce);
    }

    #[test]
    fn test_block_with_multiple_transactions() {
        let mut transactions = vec![];
        for i in 0..5 {
            transactions.push(create_test_transaction(vec![i as u8; 4]));
        }
        
        let block = Block::new_block("multi_tx_test".to_string(), &transactions, 10);
        
        assert_eq!(block.get_transactions().len(), 5);
        assert_eq!(block.get_height(), 10);
        assert_eq!(block.get_pre_block_hash(), "multi_tx_test");
        
        // Verify each transaction is preserved
        for (i, tx) in block.get_transactions().iter().enumerate() {
            assert_eq!(tx.get_id(), &vec![i as u8; 4]);
        }
    }

    #[test]
    fn test_serialize_deserialize_round_trip_consistency() {
        let transaction = create_test_transaction(vec![255, 128, 64, 32, 16]);
        let transactions = vec![transaction];
        
        let mut original_block = Block::new_block("consistency_test".to_string(), &transactions, 999);
        original_block.hash = "custom_hash_value".to_string();
        original_block.nonce = 12345;
        
        // Serialize and deserialize multiple times
        let mut current_block = original_block.clone();
        for _ in 0..3 {
            let serialized = current_block.serialize();
            current_block = Block::deserialize(&serialized);
        }
        
        // Verify all fields remain unchanged
        assert_eq!(original_block.pre_block_hash, current_block.pre_block_hash);
        assert_eq!(original_block.hash, current_block.hash);
        assert_eq!(original_block.height, current_block.height);
        assert_eq!(original_block.timestamp, current_block.timestamp);
        assert_eq!(original_block.nonce, current_block.nonce);
        assert_eq!(original_block.transactions.len(), current_block.transactions.len());
    }
}

