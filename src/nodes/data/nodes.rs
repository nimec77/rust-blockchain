use std::sync::RwLock;

use crate::Node;

pub struct Nodes {
    pub(in crate::nodes) inner: RwLock<Vec<Node>>,
}
