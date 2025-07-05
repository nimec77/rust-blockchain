use crate::transaction::{TXInput, TXOutput};

pub const SUBSIDY: i32 = 10;

#[derive(Clone, bincode::Encode, bincode::Decode)]
pub struct Transaction {
    pub(in crate::transaction) id: Vec<u8>,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

