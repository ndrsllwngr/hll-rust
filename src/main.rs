extern crate chrono;
extern crate clap;
extern crate crypto;
extern crate futures;
extern crate get_if_addrs;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate num;
extern crate num_bigint;
#[macro_use]
extern crate prettytable;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate signal_hook;
extern crate tokio;

use std::net::{Ipv4Addr, SocketAddr};

use clap::{App, Arg};

mod input;
mod print;

mod chord;
mod fingertable;
mod node;
mod storage;

mod network;
mod protocols;

fn main() {
    // Init logger
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    debug!("Booting...");

    // Find 'en0' interface on macOS, extract Ip4Addr and add it CLI help
    let interfaces: Vec<get_if_addrs::Interface> = get_if_addrs::get_if_addrs().unwrap();
    let interface_option = interfaces
        .into_iter()
        .find(|i| i.name == "en0" && i.addr.ip().is_ipv4());
    let local_ip4addr = if let Some(interface) = interface_option {
        interface.addr.ip().to_string()
    } else {
        "<en0 not found>".to_string()
    };
    let ip4addr_help = format!("Sets the ip address to use (e.g. {})", local_ip4addr);

    // CLI requires static string
    let ip4addr_help_slice = &ip4addr_help[..];
    debug!("en0 interface IP4ADDR is: {}", local_ip4addr);

    // CLI incl. required arguments
    let matches = App::new("hll_rust_chord")
        .version("1.0")
        .author("Andreas Ellwanger, Timo Erdelt and Andreas Griesbeck")
        .about("High level languages: Rust - Group project (2018/2019)")
        .arg(
            Arg::with_name("ip4_addr")
                .short("i")
                .long("ipaddr")
                .value_name("IP4ADDR")
                .help(ip4addr_help_slice)
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Sets the port to use")
                .takes_value(true)
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("entry_point")
                .short("j")
                .long("join")
                .value_name("IP4ADDR:PORT")
                .help("Sets the node (entry point to an existing chord ring) to join")
                .takes_value(true)
                .required(false)
                .index(3),
        )
        .get_matches();

    // Validate, parse CLI arguments
    let ip4_addr = match matches.value_of("ip4_addr").unwrap().parse::<Ipv4Addr>() {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    debug!("ip4_addr: {}", ip4_addr);
    let port = match matches.value_of("port").unwrap().parse::<i32>() {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    debug!("port: {}", port);
    let listening_ip = match format!("{}:{}", ip4_addr, port).parse::<SocketAddr>() {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    debug!("listening_ip: {}", listening_ip);

    // Join existing chord ring, or create new chord ring as first node
    if matches.is_present("entry_point") {
        let entry_point = match matches
            .value_of("entry_point")
            .unwrap()
            .parse::<SocketAddr>()
        {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        };
        debug!("entry_point: {}", entry_point);
        if listening_ip != entry_point {
            let node_handle = chord::spawn_node(listening_ip, port, Some(entry_point));
            node_handle.join().expect("node_handle.join() failed");
        } else {
            panic!(
                "listening_ip != entry_point = {}",
                listening_ip != entry_point
            );
        }
    } else {
        let first_node_handle = chord::spawn_node(listening_ip, port, None);
        first_node_handle
            .join()
            .expect("first_node_handle.join() failed");
    }
}
