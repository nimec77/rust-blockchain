use std::sync::RwLock;

use crate::{Node, nodes::data::nodes::Nodes};

impl Nodes {
    pub fn new() -> Nodes {
        Nodes {
            inner: RwLock::new(vec![]),
        }
    }

    pub fn add_node(&self, addr: String) {
        let mut inner = self.inner.write().unwrap();
        if !inner.iter().any(|x| x.get_addr().eq(addr.as_str())) {
            inner.push(Node::new(addr));
        }
    }

    pub fn evict_node(&self, addr: &str) {
        let mut inner = self.inner.write().unwrap();
        if let Some(idx) = inner.iter().position(|x| x.get_addr().eq(addr)) {
            inner.remove(idx);
        }
    }

    pub fn first(&self) -> Option<Node> {
        let inner = self.inner.read().unwrap();
        if let Some(node) = inner.first() {
            return Some(node.clone());
        }
        None
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        self.inner.read().unwrap().to_vec()
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.read().unwrap().is_empty()
    }

    pub fn node_is_known(&self, addr: &str) -> bool {
        let inner = self.inner.read().unwrap();
        if inner.iter().any(|x| x.get_addr().eq(addr)) {
            return true;
        }
        false
    }
}

impl Default for Nodes {
    fn default() -> Self {
        Self::new()
    }
}
