use crate::transaction::{TXInput, TXOutput};

#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct Transaction {
    pub(crate) id: Vec<u8>,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

