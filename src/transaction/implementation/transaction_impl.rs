use bincode::config::standard;

use crate::{transaction::Transaction, util::{ecdsa_p256_sha256_sign_verify, sha256_digest}, Blockchain, TXInput, TXOutput};

impl Transaction {
    pub fn get_id(&self) -> &[u8] {
        &self.id
    }

    fn trimmed_copy(&self) -> Transaction {
        let mut inputs = vec![];
        let mut outputs = vec![];
        for input in &self.vin {
            let txinput = TXInput::new(input.get_txid(), input.get_vout());
            inputs.push(txinput);
        }
        for output in &self.vout {
            outputs.push(output.clone());
        }
        Transaction {
            id: self.id.clone(),
            vin: inputs,
            vout: outputs,
        }
    }

    fn hash(&mut self) -> Vec<u8> {
        let tx_copy = Transaction {
            id: vec![],
            vin: self.vin.clone(),
            vout: self.vout.clone(),
        };
        sha256_digest(tx_copy.serialize().as_slice())
    }

    pub fn verify(&self, blockchain: &Blockchain) -> bool {
        if self.is_coinbase() {
            return true;
        }
        let mut tx_copy = self.trimmed_copy();
        for (idx, vin) in self.vin.iter().enumerate() {
            let prev_tx_option = blockchain.find_transaction(vin.get_txid());
            if prev_tx_option.is_none() {
                panic!("ERROR: Previous transaction is not correct")
            }
            let prev_tx = prev_tx_option.unwrap();
            tx_copy.vin[idx].signature = vec![];
            tx_copy.vin[idx].pub_key = prev_tx.vout[vin.vout].pub_key_hash.clone();
            tx_copy.id = tx_copy.hash();
            tx_copy.vin[idx].pub_key = vec![];

            let verify = ecdsa_p256_sha256_sign_verify(
                vin.pub_key.as_slice(),
                vin.signature.as_slice(),
                tx_copy.get_id(),
            );
            if !verify {
                return false;
            }
        }
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

    pub fn serialize(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, standard()).unwrap()
    }

    pub fn deserialize(bytes: &[u8]) -> Transaction {
        let (tx, _) = bincode::decode_from_slice(bytes, standard()).unwrap();

        tx
    }

    pub fn try_deserialize(bytes: &[u8]) -> Result<Transaction, bincode::error::DecodeError> {
        let (tx, _) = bincode::decode_from_slice(bytes, standard())?;

        Ok(tx)
    }
}
