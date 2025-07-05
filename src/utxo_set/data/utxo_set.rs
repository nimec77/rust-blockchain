use crate::Blockchain;

pub const UTXO_TREE: &str = "chainstate";

pub struct UTXOSet {
   pub(in crate::utxo_set) blockchain: Blockchain,
}
