#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct TXInput {
    pub txid: Vec<u8>,
    pub vout: usize,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
}
