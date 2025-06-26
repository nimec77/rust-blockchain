#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct TXOutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}
