use crate::{Blockchain, TXInput, TXOutput, transaction::Transaction};

impl Transaction {
    pub fn get_id(&self) -> &[u8] {
        &self.id
    }

    pub fn verify(&self, _: &Blockchain) -> bool {
        // if self.is_coinbase() {
        //     return true;
        // }
        // let mut tx_copy = self.trimmed_copy();
        // for (idx, vin) in self.vin.iter().enumerate() {
        //     let prev_tx_option = blockchain.find_transaction(vin.get_txid());
        //     if prev_tx_option.is_none() {
        //         panic!("ERROR: Previous transaction is not correct")
        //     }
        //     let prev_tx = prev_tx_option.unwrap();
        //     tx_copy.vin[idx].signature = vec![];
        //     tx_copy.vin[idx].pub_key = prev_tx.vout[vin.vout].pub_key_hash.clone();
        //     tx_copy.id = tx_copy.hash();
        //     tx_copy.vin[idx].pub_key = vec![];

        //     let verify = crate::ecdsa_p256_sha256_sign_verify(
        //         vin.pub_key.as_slice(),
        //         vin.signature.as_slice(),
        //         tx_copy.get_id(),
        //     );
        //     if !verify {
        //         return false;
        //     }
        // }
        true
    }

    pub fn get_vin(&self) -> &[TXInput] {
        self.vin.as_slice()
    }

    pub fn get_vout(&self) -> &[TXOutput] {
        self.vout.as_slice()
    }

    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].pub_key.is_empty()
    }
}
