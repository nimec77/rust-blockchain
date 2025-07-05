use std::{collections::HashMap, sync::RwLock};

use crate::Transaction;

pub struct MemoryPool {
    pub(in crate::memory_pool) inner: RwLock<HashMap<String, Transaction>>,
}
