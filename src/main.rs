mod block;
mod models;
mod proof_of_work;

use block::Block;
use proof_of_work::ProofOfWork;
use sha2::{Sha256, Digest};

pub fn current_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn main() {
    println!("Hello, world!");
}
