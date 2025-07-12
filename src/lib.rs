mod common;
mod transaction;
mod block;
mod proof_of_work;
pub mod util;
mod blockchain;
pub mod nodes;
pub mod memory_pool;
pub mod config;
pub mod utxo_set;
pub mod server;
pub mod wallet;
pub mod command;

// Convenience re-exports for commonly used types
pub use block::Block;
pub use transaction::{Transaction, TXInput, TXOutput};
pub use proof_of_work::{ProofOfWork, MAX_NONCE, TARGET_BITS};
pub use blockchain::{Blockchain, TIP_BLOCK_HASH_KEY, BLOCKS_TREE, BlockchainIterator};
pub use common::BincodeBigInt;
pub use nodes::{Node, Nodes};
pub use memory_pool::{MemoryPool, BlockInTransit};
pub use utxo_set::{UTXOSet, UTXO_TREE};
pub use util::new_key_pair;
