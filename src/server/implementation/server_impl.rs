use std::{net::TcpListener, thread};

use crate::{server::{data::server::CENTRAL_NODE, Server}, Blockchain};

impl Server {
    pub fn new(blockchain: Blockchain) -> Server {
        Server { blockchain }
    }

    // pub fn run(&self, addr: &str) {
    //     let listener = TcpListener::bind(addr).unwrap();

    //     if addr.eq(CENTRAL_NODE) == false {
    //         let best_height = self.blockchain.get_best_height();
    //         send_version(CENTRAL_NODE, best_height);
    //     }
    //     for stream in listener.incoming() {
    //         let blockchain = self.blockchain.clone();
    //         thread::spawn(|| match stream {
    //             Ok(stream) => {
    //                 todo!()
    //             }
    //             Err(e) => {
    //                 todo!()
    //             }
    //         });
    //     }
    // }
}
