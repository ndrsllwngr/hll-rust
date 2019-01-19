use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::thread;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use super::node::OtherNode;
use super::protocols::Message;

//TODO find out if copy & clone is the right solution for the error happening when removing this
pub struct Network {
    //TODO to be implemented
    addr: SocketAddr,
}

impl Network {
    pub fn new(addr: SocketAddr) -> Network {
        //TODO implement correctly
        //let pool = ;
        Network { addr }
    }

    pub fn send(&self, _from: &OtherNode, _to: &OtherNode, _msg: &Message) {
        //TODO implement
    }


    fn handle_request(&self, stream: TcpStream, client_addr: SocketAddr) {
        let mut reader = BufReader::new(stream);

        loop {
            let mut buffer = String::new();
            match reader.read_line(&mut buffer) {
                Ok(len) => {
                    // break when line is finished
                    if len == 0 {
                        break;
                    } else {
                        info!("New message from {}: {}", client_addr.to_string(), buffer);
                        let parsed_message: Message = serde_json::from_str(&buffer).unwrap();
                        parsed_message.print()
                        // TODO parse message and handle it in Node

                    }
                }
                Err(e) => {
                    error!("Error reading message from {}: {}",client_addr, e)
                }
            }
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

                    self.handle_request(stream, addr);
                }
                Err(e) => {
                    error!("Connection failed: {:?}", e)
                }
            };
        };
    }

    //TODO this works partially with netcat, but netcat stops listening after it recieves the message
    //TODO investigate
    pub fn send_string_to_socket(addr: SocketAddr, msg: String) {
        //TODO aparently streams dont have to be closed, but check again
        let thread = thread::spawn(move || {
            match TcpStream::connect(addr) {
                Ok(stream) => {
                    let mut writer = BufWriter::new(stream);
                    writer.write_all(msg.as_bytes()).unwrap();
                    info!("Sent msg: {}", msg);
                }
                Err(e) => {
                    error!("Unable to send msg - Failed to connect: {}", e);
                }
            }
        });
    }
}
