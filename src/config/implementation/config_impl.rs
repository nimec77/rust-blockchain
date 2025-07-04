use std::{collections::HashMap, env, sync::RwLock};

use crate::config::{
    Config,
    data::config::{DEFAULT_NODE_ADDR, MINING_ADDRESS_KEY, NODE_ADDRESS_KEY},
};

impl Config {
    pub fn new() -> Config {
        let mut node_addr = String::from(DEFAULT_NODE_ADDR);
        if let Ok(addr) = env::var("NODE_ADDRESS") {
            node_addr = addr;
        }
        let mut map = HashMap::new();
        map.insert(String::from(NODE_ADDRESS_KEY), node_addr);

        Config {
            inner: RwLock::new(map),
        }
    }

    pub fn get_node_addr(&self) -> String {
        let inner = self.inner.read().unwrap();
        inner.get(NODE_ADDRESS_KEY).unwrap().clone()
    }

    pub fn set_mining_addr(&self, addr: String) {
        let mut inner = self.inner.write().unwrap();
        let _ = inner.insert(String::from(MINING_ADDRESS_KEY), addr);
    }

    pub fn get_mining_addr(&self) -> Option<String> {
        let inner = self.inner.read().unwrap();
        if let Some(addr) = inner.get(MINING_ADDRESS_KEY) {
            return Some(addr.clone());
        }
        None
    }

    pub fn is_miner(&self) -> bool {
        let inner = self.inner.read().unwrap();
        inner.contains_key(MINING_ADDRESS_KEY)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
