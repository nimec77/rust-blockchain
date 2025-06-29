mod data;
mod implementation;

// Re-export the main struct and constants
pub use data::proof_of_work::{ProofOfWork, MAX_NONCE, TARGET_BITS};
