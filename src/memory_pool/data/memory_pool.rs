use std::{collections::HashMap, sync::RwLock};

use crate::Transaction;

pub struct MemoryPool {
    pub(crate) inner: RwLock<HashMap<String, Transaction>>,
}
