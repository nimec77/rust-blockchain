mod common;
mod transaction;
mod block;
mod proof_of_work;
mod util;
mod blockchain;

// Convenience re-exports for commonly used types
pub use block::Block;
pub use transaction::{Transaction, TXInput, TXOutput};
pub use proof_of_work::{ProofOfWork, MAX_NONCE, TARGET_BITS};
pub use blockchain::{Blockchain, TIP_BLOCK_HASH_KEY, BLOCKS_TREE};
pub use common::BincodeBigInt;
