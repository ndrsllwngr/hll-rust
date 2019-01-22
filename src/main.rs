extern crate crypto;
extern crate getopts;
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

use getopts::Options;
use std::env;
use std::thread::JoinHandle;
use std::{thread, time};

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

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("i", "", "Specify local ip address", "IPV4");
    opts.optopt("n", "", "Specify number of nodes to spawn", "NUMBER");
    opts.optopt(
        "j",
        "",
        "Specify known node ip address and port",
        "IPV4:PORT",
    );
    opts.optflag("h", "help", "Print help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let ip_address_optional = matches.opt_str("i");
    let node_number_optional = matches.opt_str("n");
    let join_ip_optional = matches.opt_str("j");

    let ip_address = if let Some(ip_address) = ip_address_optional {
        ip_address
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
    debug!("Using {} as ip address.", ip_address);

    let number_of_nodes = if let Some(number) = node_number_optional {
        match number.parse::<i32>() {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        }
    } else {
        1
    };
    debug!("Spawning {} nodes.", number_of_nodes);

    // TODO maybe instead ask to start program via input by user
    let millis2000 = time::Duration::from_millis(2000);
    let now = time::Instant::now();
    thread::sleep(millis2000);
    assert!(now.elapsed() >= millis2000);

    // Don't forget to join handles, otherwise program terminates instantely
    let threads_handles = spawn_chord_circle(ip_address, number_of_nodes);
    for handler in threads_handles {
        if let Err(e) = handler.join() {
            error!("{:?}", e)
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
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
    for x in 0..number_of_nodes {
        node_handlers.push(spawn_node(
            ip_addr.clone(),
            base_port + x,
            format!("N{}", x),
        ))
    }
    node_handlers
}
