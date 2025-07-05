use std::{collections::HashMap, sync::RwLock};

use once_cell::sync::Lazy;

pub static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(Config::new);

pub static DEFAULT_NODE_ADDR: &str = "127.0.0.1:2001";

pub const NODE_ADDRESS_KEY: &str = "NODE_ADDRESS";
pub const MINING_ADDRESS_KEY: &str = "MINING_ADDRESS";

pub struct Config {
    pub inner: RwLock<HashMap<String, String>>,
}
