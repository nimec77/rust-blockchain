use crate::{transaction::TXOutput, util, wallet};

impl TXOutput {
   
    pub fn new(value: i32, address: &str) -> TXOutput {
        let mut output = TXOutput {
            value,
            pub_key_hash: vec![],
        };
        output.lock(address);
        
        output
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_pub_key_hash(&self) -> &[u8] {
        self.pub_key_hash.as_slice()
    }

    fn lock(&mut self, address: &str) {
        let payload = util::base58_decode(address);
        
        // Check if payload has minimum required length: version (1) + checksum (4) = 5
        if payload.len() < 1 + wallet::ADDRESS_CHECK_SUM_LEN {
            // Invalid address - set empty pub_key_hash
            self.pub_key_hash = vec![];
            return;
        }
        
        let pub_key_hash = payload[1..payload.len() - wallet::ADDRESS_CHECK_SUM_LEN].to_vec();
        self.pub_key_hash = pub_key_hash;
    }

    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash.eq(pub_key_hash)
    }
}
