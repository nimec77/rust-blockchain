pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

pub struct TXOutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}

pub fn run(&self) -> (i64, String) {
    let mut nonce = 0;
    let mut hash = Vec::new();
    println!("Mining the block");
    while nonce < MAX_NONCE {
        let data = self.prepare_data(nonce);
        hash = crate::sha256_digest(data.as_slice());
        let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());
        if hash_int.lt(self.target.borrow()) {
            println!("{}", HEXLOWER.encode(hash.as_slice()));
            break;
        } else {
            nonce += 1;
        }
    }
    println!();
    return (nonce, HEXLOWER.encode(hash.as_slice()));
}
