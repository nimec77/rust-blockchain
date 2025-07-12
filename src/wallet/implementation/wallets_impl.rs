use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufWriter, Read, Write},
    path::PathBuf,
};

use crate::{
    util::current_dir,
    wallet::{Wallet, Wallets, data::wallets::WALLET_FILE},
};

impl Wallets {
    pub fn new() -> Wallets {
        let mut wallets = Wallets {
            wallets: HashMap::new(),
            file_path: None,
        };
        wallets.load_from_file();

        wallets
    }

    pub fn new_with_file_path(file_path: PathBuf) -> Wallets {
        let mut wallets = Wallets {
            wallets: HashMap::new(),
            file_path: Some(file_path),
        };
        wallets.load_from_file();

        wallets
    }

    pub fn create_wallet(&mut self) -> String {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        self.wallets.insert(address.clone(), wallet);
        self.save_to_file();

        address
    }

    pub fn get_addresses(&self) -> Vec<String> {
        let mut addresses = vec![];
        for address in self.wallets.keys() {
            addresses.push(address.clone())
        }

        addresses
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        if let Some(wallet) = self.wallets.get(address) {
            return Some(wallet);
        }
        None
    }

    fn get_wallet_file_path(&self) -> PathBuf {
        match &self.file_path {
            Some(path) => path.clone(),
            None => current_dir().join(WALLET_FILE),
        }
    }

    pub fn load_from_file(&mut self) {
        let path = self.get_wallet_file_path();
        if !path.exists() {
            return;
        }
        let mut file = File::open(path).unwrap();
        let metadata = file.metadata().expect("unable to read metadata");
        let mut buf = vec![0; metadata.len() as usize];
        let _ = file.read(&mut buf).expect("buffer overflow");
        let (wallets, _) = bincode::decode_from_slice(&buf[..], bincode::config::standard())
            .expect("unable to deserialize file data");
        self.wallets = wallets;
    }

    fn save_to_file(&self) {
        let path = self.get_wallet_file_path();
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&path)
            .expect("unable to open wallet.dat");
        let mut writer = BufWriter::new(file);
        let wallets_bytes = bincode::encode_to_vec(&self.wallets, bincode::config::standard())
            .expect("unable to serialize wallets");
        writer.write_all(wallets_bytes.as_slice()).unwrap();
        let _ = writer.flush();
    }
}

impl Default for Wallets {
    fn default() -> Self {
        Self::new()
    }
}
