
#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}
