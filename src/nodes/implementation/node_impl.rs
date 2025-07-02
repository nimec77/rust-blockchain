use std::net::SocketAddr;

use crate::nodes::Node;

impl Node {
    pub fn new(addr: String) -> Node {
        Node { addr }
    }

    pub fn get_addr(&self) -> String {
        self.addr.clone()
    }

    pub fn parse_socket_addr(&self) -> SocketAddr {
        self.addr.parse().unwrap()
    }
}
