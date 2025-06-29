mod data;
mod implementation;

// Re-export the main struct and constants
pub use data::blockchain::{Blockchain, TIP_BLOCK_HASH_KEY, BLOCKS_TREE};
