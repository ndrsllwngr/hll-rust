use std::{process, str, thread};
use std::io::{BufWriter, Write};
use std::io::BufReader;
use std::net;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use futures::{Future, Stream};
use num_bigint::BigInt;
use tokio::io;
use tokio::net::TcpListener;

use super::chord;
use super::node::*;
use super::protocols::*;

pub fn send_kill(target: SocketAddr) -> JoinHandle<()> {
    let msg = Message::Kill;
    send_string_to_socket(target, serde_json::to_string(&msg).unwrap())
}

pub fn send_response(sender: OtherNode, target:SocketAddr, response: Response) -> JoinHandle<()> {
    let msg = Message::ResponseMessage { sender, response };
    send_string_to_socket(target, serde_json::to_string(&msg).unwrap())
}

pub fn send_request(sender: OtherNode, target:SocketAddr, request: Request) -> JoinHandle<()> {
    let msg = Message::RequestMessage { sender, request };
    send_string_to_socket(target, serde_json::to_string(&msg).unwrap())
}

fn send_string_to_socket(addr: SocketAddr, msg: String) -> JoinHandle<()> {
    let builder = thread::Builder::new().name("Send".to_string());
    builder.spawn(move || {
        match net::TcpStream::connect(addr) {
            Ok(stream) => {
                let mut writer = BufWriter::new(stream);
                writer.write_all(msg.as_bytes()).unwrap();
                debug!("Sent msg: {}", msg);
            }
            Err(e) => {
                debug!("Unable to send msg to {} - Failed to connect: {}", addr, e);
            }
        }
    }).unwrap()
}

pub fn check_alive(addr: SocketAddr, sender: OtherNode) -> bool {
    match net::TcpStream::connect_timeout(&addr, chord::PING_TIMEOUT_INTERVAL) {
        Ok(stream) => {
            let msg = serde_json::to_string(&Message::Ping { sender }).unwrap();
            let mut writer = BufWriter::new(stream);
            writer.write_all(msg.as_bytes()).unwrap();
            true
        }
        Err(e) => {
            debug!("Check alive timeouted: {:?}", e);
            false
        }
    }
}

// HINT: this can be tested by connecting via bash terminal (preinstalled on Mac/Linux) by executing:
// nc 127.0.0.1 34254
// can be killed by sending "Kill" (with apostrophes)
// afterwards every message will be echoed in the console by handle_request
pub fn start_listening_on_socket(node_arc: Arc<Mutex<Node>>, port: i32, id: BigInt) -> Result<(), Box<std::error::Error>> {
    let listen_ip = format!("{}:{}", chord::LISTENING_ADDRESS, port)
        .parse::<SocketAddr>()
        .unwrap();

    let listener = TcpListener::bind(&listen_ip).unwrap();

    //TODO figure out if extensive cloning is working
    debug!("[Node #{}] Starting to listen on socket: {}", id.clone(), listen_ip);

    let server = listener.incoming().for_each(move |socket| {
        //debug!("[Node #{}] accepted socket; addr={:?}", id, socket.peer_addr()?);

        let buf = vec![];
        let buf_reader = BufReader::new(socket);

        let arc_clone = node_arc.clone();

        let connection = io::read_until(buf_reader, b'\n', buf)
            .and_then(move |(_socket, buf)| {
                let msg_string = str::from_utf8(&buf).unwrap();
                let message = serde_json::from_str(msg_string).unwrap();
                //info!("Look at me: {:?}",serde_json::to_string(&Message::Kill{}).unwrap());
                let mut node = arc_clone.lock().unwrap();
                match message {
                    Message::Kill => {
                        info!("Got kill message, shutting down...");
                        let node_clone = node.clone();
                        drop(node);

                        let handle_opt = node_clone.clone().graceful_shutdown();
                        if let Some(handle) = handle_opt {
                            handle.join().expect("handle_graceful_shutdown failed");
                        }
                        process::exit(0);
                    }
                    Message::Ping { sender } => {
                        debug!("Got pinged from Node #{}", sender.get_id());
                        Ok(())
                    }
                    Message::RequestMessage { sender, request } => {
                        debug!("[Node #{}] Got request from Node #{}: {:?}", node.get_id().clone(), sender.get_id(), request.clone());
                        let response_option = node.process_incoming_request(request);
                        let node_as_other_node = node.to_other_node();
                        drop(node);
                        if let Some(response) = response_option {
                            send_response(node_as_other_node, *sender.get_ip_addr(), response);
                        }
                        Ok(())
                    }
                    Message::ResponseMessage { sender, response } => {
                        debug!("[Node #{}] Got response from Node #{}: {:?}", node.get_id().clone(), sender.get_id(), response.clone());
                        node.process_incoming_response(response);
                        drop(node);
                        Ok(())
                    }
                }
            })
            .then(|_| Ok(())); // Just discard the socket and buffer

        // Spawn a new task that processes the socket:
        tokio::spawn(connection);

        Ok(())
    }).map_err(|e| println!("failed to accept socket; error = {:?}", e));
    tokio::run(server);
    Ok(())
}