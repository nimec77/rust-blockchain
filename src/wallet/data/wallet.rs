pub const VERSION: u8 = 0x00;
pub const ADDRESS_CHECK_SUM_LEN: usize = 4;

#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct Wallet {
    pub(crate) pkcs8: Vec<u8>,
    pub(crate) public_key: Vec<u8>,
}
