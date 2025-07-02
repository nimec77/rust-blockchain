use std::{collections::HashMap, sync::RwLock};

use data_encoding::HEXLOWER;

use crate::{Transaction, memory_pool::MemoryPool};

impl MemoryPool {
    pub fn new() -> MemoryPool {
        MemoryPool {
            inner: RwLock::new(HashMap::new()),
        }
    }

    pub fn contains(&self, txid_hex: &str) -> bool {
        self.inner.read().unwrap().contains_key(txid_hex)
    }

    pub fn add(&self, tx: Transaction) {
        let txid_hex = HEXLOWER.encode(tx.get_id());
        self.inner.write().unwrap().insert(txid_hex, tx);
    }

    pub fn get(&self, txid_hex: &str) -> Option<Transaction> {
        if let Some(tx) = self.inner.read().unwrap().get(txid_hex) {
            return Some(tx.clone());
        }
        None
    }

    pub fn remove(&self, txid_hex: &str) {
        let mut inner = self.inner.write().unwrap();
        inner.remove(txid_hex);
    }

    pub fn get_all(&self) -> Vec<Transaction> {
        let inner = self.inner.read().unwrap();
        let mut txs = vec![];
        for (_, v) in inner.iter() {
            txs.push(v.clone());
        }
        txs
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.read().unwrap().is_empty()
    }
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new()
    }
}
