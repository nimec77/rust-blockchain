use std::{io::Write, net::{SocketAddr, TcpStream}, time::Duration};

use log::{error, info};

use crate::{config::GLOBAL_CONFIG, server::{data::server::{GLOBAL_NODES, TCP_WRITE_TIMEOUT}, OpType, Package}};


fn send_data(addr: SocketAddr, pkg: Package) {
    info!("send package: {:?}", &pkg);
    let stream = TcpStream::connect(addr);
    if stream.is_err() {
        error!("The {addr} is not valid");

        GLOBAL_NODES.evict_node(addr.to_string().as_str());
        return;
    }
    let mut stream = stream.unwrap();
    let _ = stream.set_write_timeout(Option::from(Duration::from_millis(TCP_WRITE_TIMEOUT)));
    let serialized = bincode::encode_to_vec(&pkg, bincode::config::standard()).unwrap();
    let _ = stream.write_all(&serialized);
    let _ = stream.flush();
}

pub(crate) fn send_get_data(addr: &str, op_type: OpType, id: &[u8]) {
    let socket_addr = addr.parse().unwrap();
    let node_addr = GLOBAL_CONFIG.get_node_addr().parse().unwrap();
    send_data(
        socket_addr,
        Package::GetData {
            addr_from: node_addr,
            op_type,
            id: id.to_vec(),
        },
    );
}
