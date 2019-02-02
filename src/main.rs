extern crate crypto;
extern crate futures;
extern crate get_if_addrs;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate num;
extern crate num_bigint;
#[macro_use] extern crate prettytable;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate signal_hook;
extern crate tokio;

use std::env;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::{error::Error};
use signal_hook::{iterator::Signals, SIGINT};

use getopts::Options;

mod chord;
mod chord_util;
mod finger;
mod interaction;
mod network_util;
mod node;
mod node_util;
mod protocols;
mod storage;
mod util;

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
        // return;
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
        8080 //TODO maybe randomise
    };
    debug!("Port is {}.", port);

    let listen_ip = format!("{}:{}", ip_address.clone(), port)
        .parse::<SocketAddr>()
        .unwrap();

    if let Some(join_ip) = join_ip_option {
        //Join existing node
        let node_handle = spawn_node(listen_ip, Some(join_ip.parse::<SocketAddr>().unwrap()));
        node_handle.join().expect("node_handle.join() failed");
    } else {
        //Create new ring
        let first_node_handle = spawn_node(listen_ip, None);
        first_node_handle.join().expect("first_node_handle.join() failed");
    }
}

fn spawn_node(node_ip_addr: SocketAddr, entry_node_addr: Option<SocketAddr>) -> JoinHandle<()> {
    if let Some(entry_node_addr) = entry_node_addr {
        info!("Spawn node and join.");
    } else {
        info!("Spawn master node.");
    }
    let builder = thread::Builder::new().name("Node".to_string());
    builder
        .spawn(move || {
            let node = if let Some(entry_node_addr) = entry_node_addr {
                node::Node::new(node_ip_addr.clone())
            } else {
                node::Node::new_first(node_ip_addr.clone())
            };
            // let mut node = node::Node::new(node_ip_addr.clone());
            let id = node.id.clone();
            let id_clone = id.clone();

            let other_node = node.to_other_node();

            let arc = Arc::new(Mutex::new(node));
            let arc_clone = arc.clone();
            
            let handle1 = thread::Builder::new().name("Listen".to_string())
                .spawn(move || {
                    network_util::start_listening_on_socket(arc_clone, node_ip_addr, id_clone).expect("network_util::start_listening_on_socket failed");
                }).unwrap();

            if let Some(entry_node_addr) = entry_node_addr {
                thread::sleep(chord::NODE_INIT_SLEEP_INTERVAL);
                chord_util::join(id.clone(),other_node.clone(),entry_node_addr);
            }

            let arc_clone2 = arc.clone();
            let handle2 = thread::Builder::new().name("Stabilize".to_string())
                .spawn(move || {
                    chord_util::stabilize(arc_clone2);
                }).unwrap();

            let arc_clone3 = arc.clone();
            let handle3 = thread::Builder::new().name("Fix_Fingers".to_string())
                .spawn(move || {
                    chord_util::fix_fingers(arc_clone3);
                }).unwrap();

            let arc_clone4 = arc.clone();
            let handle4 = thread::Builder::new().name("Check_Predecessor".to_string())
                .spawn(move || {
                    chord_util::check_predecessor(arc_clone4);
                }).unwrap();

            let arc_clone5 = arc.clone();
            let handle5 = thread::Builder::new().name("Print_Interact".to_string())
                .spawn(move || {
                    chord_util::print_and_interact(arc_clone5);
                }).unwrap();

            handle1.join().expect("handle1 failed");
            handle2.join().expect("handle2 failed");
            handle3.join().expect("handle3 failed");
            handle4.join().expect("handle4 failed");
            handle5.join().expect("handle5 failed");


        })
        .unwrap()
}