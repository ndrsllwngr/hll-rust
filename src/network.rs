use tokio::io::copy;
use tokio::net::TcpListener;
use tokio::prelude::*;
use std::net::SocketAddr;
use super::protocols::Message;
use super::node::OtherNode;

pub struct Network {
    //TODO to be implemented
    addr:SocketAddr,
}

impl Network {
    pub fn new(addr: SocketAddr) -> Network {
        //TODO implement correctly
        return Network {
            addr,
        };
    }

    pub fn send(&self, _msg: Message, _to: OtherNode) {
        //TODO implement
    }

    pub fn start_listening_on_socket(self) {
        //TODO improve & check if working
        info!("Bind the server socket.");
        let listener = TcpListener::bind(&self.addr).expect("unable to bind TCP listener");

        // Pull out a stream of sockets for incoming connections
        let server = listener
            .incoming()
            .map_err(|e| error!("accept failed = {:?}", e))
            .for_each(|sock| {
                // Split up the reading and writing parts of the
                // socket.
                let (reader, writer) = sock.split();

                // A future that echos the data and returns how
                // many bytes were copied...
                let bytes_copied = copy(reader, writer);

                // ... after which we'll print what happened.
                let handle_conn = bytes_copied
                    .map(|amt| info!("wrote {:?} bytes", amt))
                    .map_err(|err| error!("IO error {:?}", err));

                // Spawn the future as a concurrent task.
                tokio::spawn(handle_conn)
            });

        // Start the Tokio runtime
        info!("Server is running.");
        tokio::run(server);
    }
}