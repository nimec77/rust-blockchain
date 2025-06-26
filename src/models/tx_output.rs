#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct TXOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>,
}
