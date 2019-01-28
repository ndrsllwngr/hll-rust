extern crate crypto;
extern crate getopts;
extern crate num;
extern crate num_bigint;

extern crate tokio;
extern crate futures;

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate get_if_addrs;

use getopts::Options;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread::JoinHandle;
use std::{thread, time};

mod chord;
mod finger;
mod network_util;
mod node;
mod protocols;
mod storage;
mod util;
mod tokio_experiments;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    debug!("Booting...");


    // Command line options
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("i", "", "Use to specify local ip_address for nodes to bind to", "127.0.0.1");
    opts.optopt("n", "", "Use to specify number of nodes to spawn (standard = 1)", "1");
    opts.optopt(
        "j",
        "",
        "Use to join existing chord ring at a nodes ip_address:port",
        "127.0.0.1:5555",
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
    let node_number_option = matches.opt_str("n");
    let join_ip_option = matches.opt_str("j");

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

    let number_of_nodes = if let Some(number) = node_number_option {
        match number.parse::<i32>() {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        }
    } else {
        1
    };
    debug!("Spawning {} nodes.", number_of_nodes);

    // let join_ip =

    // TODO maybe instead ask to start program via input by user
    let millis2000 = time::Duration::from_millis(2000);
    //let now = time::Instant::now();
    //thread::sleep(millis2000);
    //assert!(now.elapsed() >= millis2000);

    let first_node_ip = format!("{}:{}", ip_address.clone(), 6666)
        .parse::<SocketAddr>()
        .unwrap();

    let first_node_handle = spawn_first_node("FIRST".to_string(), first_node_ip);
    thread::sleep(millis2000);
    let mut node_handles = spawn_chord_circle(ip_address, number_of_nodes, first_node_ip);
    node_handles.push(first_node_handle);
    // Don't forget to join handles in the end, otherwise program terminates instantly
    for handler in node_handles {
        if let Err(e) = handler.join() {
            error!("{:?}", e)
        }
    }


//    let builder = thread::Builder::new().name(format!("{}-Listen", "echo").to_string());
//    let handler = builder
//        .spawn(move || {
//            tokio_experiments::listen_and_answer();
//        })
//        .unwrap();
//    let millis2000 = time::Duration::from_millis(2000);
//    let now = time::Instant::now();
//    thread::sleep(millis2000);
//    tokio_experiments::write_to_stream_with_answer("127.0.0.1:12345".to_string(), "Hi wazzup".to_string());
//    handler.join();

}

fn spawn_node(name: String, node_ip_addr: SocketAddr, entry_node_addr: SocketAddr) -> JoinHandle<()> {
    let builder = thread::Builder::new().name(name.clone().to_string());
    builder
        .spawn(move || {
            let mut node = node::Node::new(name.clone(), node_ip_addr, entry_node_addr);
            let mut node_clone = node.clone();
            let builder = thread::Builder::new().name(format!("{}-Listening", name).to_string());
            let handler = builder
                .spawn(move || {
                    node.start_listening_on_socket();
                }).unwrap();
            node_clone.join();
            handler.join();
        })
        .unwrap()
}

fn spawn_first_node(name: String, node_ip_addr: SocketAddr) -> JoinHandle<()> {
    let builder = thread::Builder::new().name(name.clone().to_string());
    builder
        .spawn(move || {
            let mut node = node::Node::new_first(name.clone(), node_ip_addr);
            node.start_stabilisation();
            let builder = thread::Builder::new().name(format!("{}-Listening", name).to_string());
            crossbeam::scope(|scope| {
                scope.spawn(|_| {
                    node.start_listening_on_socket();
                });
            });
            println!("{}", 2345);
            //node.start_stabilisation();
            //handler.join();
        })
        .unwrap()
}

fn spawn_chord_circle(ip_address: String, number_of_nodes: i32, entry_node_addr: SocketAddr) -> Vec<JoinHandle<()>> {
    let mut node_handlers = Vec::new();
    let base_port: i32 = 10000;
    for x in 0..number_of_nodes {
        let node_port = base_port + x;
        let node_ip_addr = format!("{}:{}", ip_address.clone(), node_port)
            .parse::<SocketAddr>()
            .unwrap();
        node_handlers.push(
            spawn_node(format!("N{}", x), node_ip_addr, entry_node_addr,
            ))
    }
    node_handlers
}
