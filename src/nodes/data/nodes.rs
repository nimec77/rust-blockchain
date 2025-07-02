use std::sync::RwLock;

use crate::Node;

pub struct Nodes {
    pub inner: RwLock<Vec<Node>>,
}
