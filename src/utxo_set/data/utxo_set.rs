use crate::Blockchain;

pub const UTXO_TREE: &str = "chainstate";

pub struct UTXOSet {
   pub(crate) blockchain: Blockchain,
}
