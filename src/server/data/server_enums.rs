use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub enum OpType {
    Tx,
    Block,
}

#[derive(Debug, Encode, Decode)]
pub enum Package {
    Block {
        addr_from: String,
        block: Vec<u8>,
    },
    GetBlocks {
        addr_from: String,
    },
    GetData {
        addr_from: String,
        op_type: OpType,
        id: Vec<u8>,
    },
    Inv {
        addr_from: String,
        op_type: OpType,
        items: Vec<Vec<u8>>,
    },
    Tx {
        addr_from: String,
        transaction: Vec<u8>,
    },
    Version {
        addr_from: String,
        version: usize,
        best_height: usize,
    },
}
