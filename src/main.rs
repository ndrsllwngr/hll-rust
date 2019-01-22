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

use num_bigint::ToBigInt;
use std::net::SocketAddr;
use std::str;
use std::thread;
use std::env;

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
        let iface = interfaces.into_iter()
            .find(|interface| interface.name == "en0" && interface.addr.ip().is_ipv4());
        if let Some(iface) = iface {
            iface.addr.ip().to_string()
        } else {
            "127.0.0.1".to_string()
        }
    };


    info!("USING IP_ADDRESS: {}", ip_address);

    let ip_address_clone_1 = ip_address.clone();
    let builder1 = thread::Builder::new().name("N1".to_string());
    let handler1 = builder1
        .spawn(|| {
            let mut node = node::Node::new(
                ip_address_clone_1,
                11111,
                None,
            );
            let mut node_clone = node.clone();
            let builder = thread::Builder::new().name("N1-Listen".to_string());
            let handler = builder
                .spawn(move || {
                    node.start_listening_on_socket();
                })
                .unwrap();
            let builder2 = thread::Builder::new().name("N1-Update".to_string());
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
        .unwrap();

    let ip_address_clone_2 = ip_address.clone();
    let builder2 = thread::Builder::new().name("N2".to_string());
    let handler2 = builder2
        .spawn(|| {
            let mut node = node::Node::new(
                ip_address_clone_2,
                22221,
                None,
            );
            if node.join(node::OtherNode::new(
                -10000.to_bigint().unwrap(),
                "127.0.0.1:11111".parse().unwrap(),
            )) {
                info!("Node2join");
            }
            let mut node_clone = node.clone();
            let builder = thread::Builder::new().name("N2-Listen".to_string());
            let handler = builder
                .spawn(move || {
                    node.start_listening_on_socket();
                })
                .unwrap();
            let builder2 = thread::Builder::new().name("N2-Update".to_string());
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
        .unwrap();

    let ip_address_clone_3 = ip_address.clone();
    let builder3 = thread::Builder::new().name("N3".to_string());
    let handler3 = builder3
        .spawn(|| {
            let mut node = node::Node::new(
                ip_address_clone_3,
                33331,
                None,
            );
            if node.join(node::OtherNode::new(
                -20000.to_bigint().unwrap(),
                "127.0.0.1:22221".parse().unwrap(),
            )) {
                info!("Node3join");
            }
            let mut node_clone = node.clone();
            let builder = thread::Builder::new().name("N3-Listen".to_string());
            let handler = builder
                .spawn(move || {
                    node.start_listening_on_socket();
                })
                .unwrap();
            let builder2 = thread::Builder::new().name("N3-Update".to_string());
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
        .unwrap();

    if let Err(e) = handler1.join() {
        error!("{:?}", e)
    }
    if let Err(e) = handler2.join() {
        error!("{:?}", e)
    }
    if let Err(e) = handler3.join() {
        error!("{:?}", e)
    }
}

//TODO check if solution exists
//fn start_node(addr: SocketAddr, predecessor: Option<SocketAddr>) {
//    let thread = thread::spawn(move || {
//        let node = node::Node::new("127.0.0.1:34254".parse::<SocketAddr>().unwrap(), None);
//        node.start_network();
//    });
//    thread.join();
//}

// fn test_endian(str: &str) {
//     let byte_vec = str.as_bytes().to_vec();

//     // 3 and 5 work!
//     let big_int_no_b = BigInt::from_bytes_be(Sign::NoSign, &byte_vec);
//     let big_int_no_l = BigInt::from_bytes_le(Sign::NoSign, &byte_vec);
//     let big_int_plus_b = BigInt::from_bytes_be(Sign::Plus, &byte_vec);
//     let big_int_plus_l = BigInt::from_bytes_le(Sign::Plus, &byte_vec);
//     let big_int_minus_b = BigInt::from_bytes_be(Sign::Minus, &byte_vec);
//     let big_int_minus_l = BigInt::from_bytes_le(Sign::Minus, &byte_vec);

//     info!("{}", big_int_plus_b);

//     let byte_vec_no_b = big_int_no_b.to_bytes_be();
//     let byte_vec_no_l = big_int_no_l.to_bytes_be();
//     let byte_vec_plus_b = big_int_plus_b.to_bytes_be();
//     let byte_vec_plus_l = big_int_plus_l.to_bytes_le();
//     let byte_vec_minus_b = big_int_minus_b.to_bytes_be();
//     let byte_vec_minus_l = big_int_minus_l.to_bytes_le();

//     let str_byte_vec_no_b = std::str::from_utf8(&byte_vec_no_b.1);
//     let str_byte_vec_no_l = std::str::from_utf8(&byte_vec_no_l.1);
//     let str_byte_vec_plus_b = std::str::from_utf8(&byte_vec_plus_b.1);
//     let str_byte_vec_plus_l = std::str::from_utf8(&byte_vec_plus_l.1);
//     let str_byte_vec_minus_b = std::str::from_utf8(&byte_vec_minus_b.1);
//     let str_byte_vec_minus_l = std::str::from_utf8(&byte_vec_minus_l.1);

//     custom_print(str_byte_vec_no_b);
//     custom_print(str_byte_vec_no_l);
//     custom_print(str_byte_vec_plus_b);
//     custom_print(str_byte_vec_plus_l);
//     custom_print(str_byte_vec_minus_b);
//     custom_print(str_byte_vec_minus_l);
// }

// fn test_modulo_bigint() {
//     let should_be_two = BigInt::modpow(
//         &12.to_bigint().unwrap(),
//         &1.to_bigint().unwrap(),
//         &10.to_bigint().unwrap(),
//     );
//     info!("{}", should_be_two)
// }

// fn test_compare_bigint() {
//     let one = &1.to_bigint().unwrap();
//     let two = &2.to_bigint().unwrap();
//     let two_again = &2.to_bigint().unwrap();
//     let three = &3.to_bigint().unwrap();

//     info!("{}", two == two_again);
//     info!("{}", two < three);
//     info!("{}", two > one);
// }

fn custom_print(result: core::result::Result<&str, std::str::Utf8Error>) {
    match result {
        Ok(n) => info!("{}", n),
        Err(e) => error!("Error: {}", e),
    }
}

fn ip_address_to_string_test() {
    let addr = "127.0.0.1:34254".parse::<SocketAddr>().unwrap();
    info!("{}", addr.to_string());
    info!("{}", "blöasdsa");
}
