#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct TXInput {
    pub txid: Vec<u8>,
    pub vout: usize,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_txinput_new() {
        let txid = b"test_transaction_id";
        let vout = 42;
        
        let tx_input = TXInput::new(txid, vout);
        
        assert_eq!(tx_input.txid, txid.to_vec());
        assert_eq!(tx_input.vout, vout);
        assert_eq!(tx_input.signature, vec![]);
        assert_eq!(tx_input.pub_key, vec![]);
    }

    #[test]
    fn test_txinput_new_empty_txid() {
        let txid = b"";
        let vout = 0;
        
        let tx_input = TXInput::new(txid, vout);
        
        assert_eq!(tx_input.txid, vec![]);
        assert_eq!(tx_input.vout, 0);
        assert_eq!(tx_input.signature, vec![]);
        assert_eq!(tx_input.pub_key, vec![]);
    }

    #[test]
    fn test_txinput_new_large_vout() {
        let txid = b"large_vout_test";
        let vout = usize::MAX;
        
        let tx_input = TXInput::new(txid, vout);
        
        assert_eq!(tx_input.txid, txid.to_vec());
        assert_eq!(tx_input.vout, usize::MAX);
    }

    #[test]
    fn test_get_txid() {
        let txid = b"sample_transaction_id";
        let vout = 1;
        let tx_input = TXInput::new(txid, vout);
        
        assert_eq!(tx_input.get_txid(), txid);
    }

    #[test]
    fn test_get_txid_empty() {
        let txid = b"";
        let vout = 0;
        let tx_input = TXInput::new(txid, vout);
        
        assert_eq!(tx_input.get_txid(), b"");
    }

    #[test]
    fn test_get_vout() {
        let txid = b"test_txid";
        let vout = 123;
        let tx_input = TXInput::new(txid, vout);
        
        assert_eq!(tx_input.get_vout(), 123);
    }

    #[test]
    fn test_get_vout_zero() {
        let txid = b"zero_vout_test";
        let vout = 0;
        let tx_input = TXInput::new(txid, vout);
        
        assert_eq!(tx_input.get_vout(), 0);
    }

    #[test]
    fn test_get_pub_key_default_empty() {
        let txid = b"pub_key_test";
        let vout = 5;
        let tx_input = TXInput::new(txid, vout);
        
        // pub_key should be empty by default from new()
        assert_eq!(tx_input.get_pub_key(), b"");
    }

    #[test]
    fn test_get_pub_key_with_manual_assignment() {
        let txid = b"manual_pub_key_test";
        let vout = 7;
        let mut tx_input = TXInput::new(txid, vout);
        
        // Manually assign a pub_key value to test the getter
        let test_pub_key = b"test_public_key_data";
        tx_input.pub_key = test_pub_key.to_vec();
        
        assert_eq!(tx_input.get_pub_key(), test_pub_key);
    }

    #[test]
    fn test_txinput_clone() {
        let txid = b"clone_test_txid";
        let vout = 99;
        let mut original = TXInput::new(txid, vout);
        original.signature = b"test_signature".to_vec();
        original.pub_key = b"test_pub_key".to_vec();
        
        let cloned = original.clone();
        
        assert_eq!(original.txid, cloned.txid);
        assert_eq!(original.vout, cloned.vout);
        assert_eq!(original.signature, cloned.signature);
        assert_eq!(original.pub_key, cloned.pub_key);
        
        // Ensure they are separate instances
        assert_ne!(original.txid.as_ptr(), cloned.txid.as_ptr());
    }

    #[test]
    fn test_txinput_serialization() {
        let txid = b"serialization_test";
        let vout = 42;
        let mut tx_input = TXInput::new(txid, vout);
        tx_input.signature = b"test_signature_data".to_vec();
        tx_input.pub_key = b"test_public_key".to_vec();
        
        // Test that it can be encoded and decoded
        let encoded = bincode::encode_to_vec(&tx_input, bincode::config::standard()).unwrap();
        let (decoded, _): (TXInput, usize) = bincode::decode_from_slice(&encoded, bincode::config::standard()).unwrap();
        
        assert_eq!(tx_input.txid, decoded.txid);
        assert_eq!(tx_input.vout, decoded.vout);
        assert_eq!(tx_input.signature, decoded.signature);
        assert_eq!(tx_input.pub_key, decoded.pub_key);
    }
}
