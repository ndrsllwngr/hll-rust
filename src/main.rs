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
extern crate chrono;

use std::env;
use std::net::SocketAddr;

use getopts::Options;

mod input;
mod print;

mod chord;
mod node;
mod fingertable;
mod storage;

mod network;
mod protocols;


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
        let node_handle = chord::spawn_node(listen_ip, port, Some(join_ip.parse::<SocketAddr>().unwrap()));
        node_handle.join().expect("node_handle.join() failed");
    } else {
        //Create new ring
        let first_node_handle = chord::spawn_node(listen_ip, port, None);
        first_node_handle.join().expect("first_node_handle.join() failed");
    }
}