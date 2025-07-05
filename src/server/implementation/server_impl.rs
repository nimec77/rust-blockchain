use std::{net::TcpListener, thread};

use log::error;

use crate::{
    Blockchain,
    server::{
        Server,
        data::server::CENTRAL_NODE,
        server_utils::{send_version, serve},
    },
};

impl Server {
    pub fn new(blockchain: Blockchain) -> Server {
        Server { blockchain }
    }

    pub fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();

        if !addr.eq(CENTRAL_NODE) {
            let best_height = self.blockchain.get_best_height();
            send_version(CENTRAL_NODE, best_height);
        }
        for stream in listener.incoming() {
            let blockchain = self.blockchain.clone();
            thread::spawn(move || match stream {
                Ok(stream) => {
                    let _ = serve(blockchain, stream);
                }
                Err(e) => {
                    error!("Error accepting connection: {e}");
                }
            });
        }
    }
}
