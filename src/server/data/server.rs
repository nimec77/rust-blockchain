use once_cell::sync::Lazy;

use crate::{BlockInTransit, Blockchain, MemoryPool, Nodes};

pub const NODE_VERSION: usize = 1;
pub const CENTRAL_NODE: &str = "127.0.0.1:2001";

pub const TRANSACTION_THRESHOLD: usize = 2;

pub static GLOBAL_NODES: Lazy<Nodes> = Lazy::new(|| {
    let nodes = Nodes::new();

    nodes.add_node(String::from(CENTRAL_NODE));

    nodes
});

pub static GLOBAL_MEMORY_POOL: Lazy<MemoryPool> = Lazy::new(MemoryPool::new);

pub static GLOBAL_BLOCKS_IN_TRANSIT: Lazy<BlockInTransit> = Lazy::new(BlockInTransit::new);

pub const TCP_WRITE_TIMEOUT: u64 = 1000;

pub struct Server {
    pub(in crate::server) blockchain: Blockchain,
}
