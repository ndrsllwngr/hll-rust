use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;
use std::io::{BufReader, BufRead, Write};
use serde::{Deserialize, Serialize};
use serde_json::Result;

use super::node::OtherNode;
use super::protocols::Message;

//TODO find out if copy & clone is the right solution for the error happening when removing this
#[derive(Copy, Clone)]
pub struct Network {
    //TODO to be implemented
    addr: SocketAddr,
}

impl Network {
    pub fn new(addr: SocketAddr) -> Network {
        //TODO implement correctly
        Network { addr }
    }

    pub fn send(&self, _from: &OtherNode, _to: &OtherNode, _msg: &Message) {
        //TODO implement
    }


    fn handle_request(self, mut stream: TcpStream, client_addr: SocketAddr) {
        let mut reader = BufReader::new(stream);

        loop {
            let mut buffer = String::new();
            let _ = reader.read_line(&mut buffer);
            info!("New message from {}: {}",client_addr.to_string(), buffer);
            let parsed_message: Message = serde_json::from_str(&buffer).unwrap();

            parsed_message.print()

            // TODO parse message and handle it in Node
        }
    }

    // HINT: this can be tested by connecting via bash terminal (preinstalled on Mac/Linux) by executing:
    // nc 127.0.0.1 34254
    // afterwards every message will be echoed in the console by handle_request
    pub fn start_listening_on_socket(self) {
        let listener = TcpListener::bind(self.addr).unwrap();
        info!("Started listening on {}", self.addr.to_string());

        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    info!("Connection by {}", addr.to_string());
                    thread::spawn(move || {
                        self.handle_request(stream, addr);
                    })
                }
                Err(e) => {
                    thread::spawn(move || {
                        error!("Connection failed: {:?}", e)
                    })
                }
            };
        };
    }

    //TODO this works partially with netcat, but netcat stops listening after it recieves the message
    //TODO investigate
    pub fn send_string_to_socket(self, addr:SocketAddr, msg:String){
        //TODO aparently streams dont have to be closed, but check again
        info!("About to send string to socket");
        match TcpStream::connect(addr) {
            Ok(mut stream) => {
                info!("Successfully connected to: {}",addr.to_string());

                stream.write(msg.as_bytes()).unwrap();
                info!("Sent msg: {}", msg);

            },
            Err(e) => {
                error!("Failed to connect: {}", e);
            }
        }
        info!("Terminated.");
    }
}
