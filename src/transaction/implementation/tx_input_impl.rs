use crate::transaction::TXInput;

impl TXInput {
    pub fn new(txid: &[u8], vout: usize) -> TXInput {
        TXInput {
            txid: txid.to_vec(),
            vout,
            signature: vec![],
            pub_key: vec![],
        }
    }

    pub fn get_txid(&self) -> &[u8] {
        self.txid.as_slice()
    }

    pub fn get_vout(&self) -> usize {
        self.vout
    }

    pub fn get_pub_key(&self) -> &[u8] {
        self.pub_key.as_slice()
    }

    // pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
    //     let locking_hash = wallet::hash_pub_key(self.pub_key.as_slice());
    //     return locking_hash.eq(pub_key_hash);
    // }
}
