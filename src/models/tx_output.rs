use crate::util;

#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct TXOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>,
}

impl TXOutput {
   
    // pub fn new(value: i32, address: &str) -> TXOutput {
    //     let mut output = TXOutput {
    //         value,
    //         pub_key_hash: vec![],
    //     };
    //     output.lock(address);
    //     return output;
    // }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_pub_key_hash(&self) -> &[u8] {
        self.pub_key_hash.as_slice()
    }

    // fn lock(&mut self, address: &str) {
    //     let payload = util::base58_decode(address);
    //     let pub_key_hash = payload[1..payload.len() - wallet::ADDRESS_CHECK_SUM_LEN].to_vec();
    //     self.pub_key_hash = pub_key_hash;
    // }

    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash.eq(pub_key_hash)
    }
}
