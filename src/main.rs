extern crate crypto;
extern crate num;
extern crate num_bigint;

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate get_if_addrs;

use std::env;
use std::thread;
use std::thread::JoinHandle;

mod chord;
mod finger;
mod network_util;
mod node;
mod protocols;
mod storage;
mod util;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    debug!("Booting...");

    // get public ip address from command line arguments
    let args: Vec<_> = env::args().collect();
    let ip_address = if args.len() > 1 {
        log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
        args[1].clone()
    } else {
        let interfaces: Vec<get_if_addrs::Interface> = get_if_addrs::get_if_addrs().unwrap();
        let iface = interfaces
            .into_iter()
            .find(|interface| interface.name == "en0" && interface.addr.ip().is_ipv4());
        if let Some(iface) = iface {
            iface.addr.ip().to_string()
        } else {
            "127.0.0.1".to_string()
        }
    };

    info!("USING IP_ADDRESS: {}", ip_address);
    // Don't forget to join handles, otherwise program terminates instantely
    let threads_handles = spawn_chord_circle(ip_address, 10);
    for handler in threads_handles {
        if let Err(e) = handler.join() {
            error!("{:?}", e)
        }
    }
}

fn spawn_node(ip_addr: String, port: i32, name: String) -> JoinHandle<()> {
    let builder = thread::Builder::new().name("N1".to_string());
    builder
        .spawn(move || {
            let mut node = node::Node::new(ip_addr, port, None);
            let mut node_clone = node.clone();
            let builder = thread::Builder::new().name(format!("{}-Listen", name).to_string());
            let handler = builder
                .spawn(move || {
                    node.start_listening_on_socket();
                })
                .unwrap();
            let builder2 = thread::Builder::new().name(format!("{}-Update", name).to_string());
            let handler2 = builder2
                .spawn(move || {
                    node_clone.start_update_fingers();
                })
                .unwrap();
            if let Err(e) = handler.join() {
                error!("{:?}", e)
            }
            if let Err(e) = handler2.join() {
                error!("{:?}", e)
            }
        })
        .unwrap()
}

fn spawn_chord_circle(ip_addr: String, number_of_nodes: i32) -> Vec<JoinHandle<()>> {
    let mut node_handlers = Vec::new();
    let base_port: i32 = 10000;
    for x in 1..number_of_nodes {
        node_handlers.push(spawn_node(
            ip_addr.clone(),
            base_port + x,
            format!("N{}", x),
        ))
    }
    node_handlers
}
