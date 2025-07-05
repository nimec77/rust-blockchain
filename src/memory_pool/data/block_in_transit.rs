use std::sync::RwLock;

pub struct BlockInTransit {
    pub(in crate::memory_pool) inner: RwLock<Vec<Vec<u8>>>,
}
