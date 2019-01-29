extern crate crypto;
extern crate futures;
extern crate get_if_addrs;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate num;
extern crate num_bigint;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use std::{thread, time};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use getopts::Options;

mod chord;
mod chord_util;
mod finger;
mod network_util;
mod node;
mod protocols;
mod storage;
mod util;
mod tokio_experiments;


/*
    run this with or without -p flag to start a new chord circle

    run this with -p & -j to join an existing chord circle
    example: cargo run -- -j 210.0.0.41:6666 -p 10001
*/

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    debug!("Booting...");


    // Command line options
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("i", "", "Used to specify local ip_address for nodes to bind to", "127.0.0.1");
    opts.optopt("p", "", "Used to specify the port the node listens on", "10000");
    opts.optopt(
        "j",
        "",
        "Use to join existing chord ring at a nodes ip_address:port",
        "127.0.0.1:6666",
    );
    opts.optflag("h", "help", "Print help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print!("\n{}", opts.usage("Usage: cargo run -- [options]"));
        return;
    }

    let ip_address_option = matches.opt_str("i");
    let join_ip_option = matches.opt_str("j");
    let port_option = matches.opt_str("p");

    let ip_address = if let Some(ip_address) = ip_address_option {
        ip_address
    } else {
        let interfaces: Vec<get_if_addrs::Interface> = get_if_addrs::get_if_addrs().unwrap();
        let interface_option = interfaces
            .into_iter()
            .find(|i| i.name == "en0" && i.addr.ip().is_ipv4());
        if let Some(interface) = interface_option {
            interface.addr.ip().to_string()
        } else {
            "127.0.0.1".to_string()
        }
    };
    debug!("Using {} as ip address.", ip_address);

    let port = if let Some(number) = port_option {
        match number.parse::<i32>() {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        }
    } else {
        6666 //TODO maybe randomise
    };
    debug!("Port is {}.", port);

    let listen_ip = format!("{}:{}", ip_address.clone(), port)
        .parse::<SocketAddr>()
        .unwrap();


    if let Some(join_ip) = join_ip_option {
        //Join existing node
        let node_handle = spawn_node("Node".to_string(), listen_ip, join_ip.parse::<SocketAddr>().unwrap());
        node_handle.join();
    } else {
        //Create new ring
        let first_node_handle = spawn_first_node("FIRST".to_string(), listen_ip);
        first_node_handle.join();
    }
}

fn spawn_node(name: String, node_ip_addr: SocketAddr, entry_node_addr: SocketAddr) -> JoinHandle<()> {
    let builder = thread::Builder::new().name(name.clone().to_string());
    builder
        .spawn(move || {
            let mut node = node::Node::new(name.clone(), node_ip_addr.clone(), entry_node_addr.clone());
            let id = node.id.clone();
            let other_node = node.to_other_node();

            let arc = Arc::new(Mutex::new(node));
            let arc_clone = arc.clone();

            let id_clone = id.clone();
            let name_clone = name.clone();
            let builder = thread::Builder::new().name("Listen".to_string());
            let handle1 = builder
                .spawn(move || {
                    network_util::start_listening_on_socket(arc_clone, node_ip_addr, id_clone);
                }).unwrap();


            thread::sleep(chord::NODE_INIT_SLEEP_INTERVAL);
            chord_util::join(id.clone(),other_node,entry_node_addr,name_clone.clone());

            let arc_clone2 = arc.clone();
            let builder = thread::Builder::new().name("Stabilize".to_string());
            let handle2 =builder
                .spawn(move || {
                    chord_util::stabilize(arc_clone2);
                }).unwrap();

            handle1.join();
            handle2.join();
        })
        .unwrap()
}

fn spawn_first_node(name: String, node_ip_addr: SocketAddr) -> JoinHandle<()> {
    let builder = thread::Builder::new().name(name.clone().to_string());
    builder
        .spawn(move || {
            let mut node = node::Node::new_first(name.clone(), node_ip_addr.clone());
            let id = node.id.clone();
            let arc = Arc::new(Mutex::new(node));
            let arc_clone = arc.clone();

            let name_clone = name.clone();
            let builder = thread::Builder::new().name("Listen".to_string());
            let handle1 = builder
                .spawn(move || {
                    network_util::start_listening_on_socket(arc_clone, node_ip_addr, id);
                }).unwrap();

            let arc_clone2 = arc.clone();
            let builder = thread::Builder::new().name("Stabilize".to_string());
            let handle2 =builder
                .spawn(move || {
                    chord_util::stabilize(arc_clone2);
                }).unwrap();
            handle1.join();
            handle2.join();
        })
        .unwrap()
}

//fn spawn_chord_circle(ip_address: String, number_of_nodes: i32, entry_node_addr: SocketAddr) -> Vec<JoinHandle<()>> {
//    let mut node_handlers = Vec::new();
//    let base_port: i32 = 10000;
//    for x in 0..number_of_nodes {
//        let node_port = base_port + x;
//        let node_ip_addr = format!("{}:{}", ip_address.clone(), node_port)
//            .parse::<SocketAddr>()
//            .unwrap();
//        node_handlers.push(
//            spawn_node(format!("N{}", x), node_ip_addr, entry_node_addr,
//            ))
//    }
//    node_handlers
//}
