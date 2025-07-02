use std::sync::RwLock;

use crate::Node;

pub struct Nodes {
    pub(crate) inner: RwLock<Vec<Node>>,
}
