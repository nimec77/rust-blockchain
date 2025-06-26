use crate::models::{tx_input::TXInput, tx_output::TXOutput};

#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

impl Transaction {
    pub fn get_id(&self) -> &[u8] {
        &self.id
    }
}
