use std::collections::HashMap;
use std::path::PathBuf;

use crate::wallet::Wallet;

pub const WALLET_FILE: &str = "wallet.dat";

#[derive(Clone, bincode::Encode, bincode::Decode)]
#[derive(Default)]
pub struct Wallets {
    pub(crate) wallets: HashMap<String, Wallet>,
    pub(crate) file_path: Option<PathBuf>,
}
