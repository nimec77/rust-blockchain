use std::sync::RwLock;

pub struct BlockInTransit {
    pub(crate) inner: RwLock<Vec<Vec<u8>>>,
}
