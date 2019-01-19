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

use num_bigint::{BigInt, Sign, ToBigInt};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str;

mod finger;
mod network;
mod node;
mod protocols;
mod storage;
mod util;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    debug!("Booting...");
    /*
    //let id = "node_id".bytes();
    //let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let mut data = HashMap::new();
    data.insert("key", "value");
    let config = node::Config{id , ip_addr};
    let storage = storage::Storage{data};
    let node = node::Node{config, storage};

    let finger_table: finger::FingerTable = finger::new_finger_table(&node, 5);
    let mut bytes: Bytes = finger_table[0].id.clone();

    assert_eq!(Some(b'f'), bytes.next());
    assert_eq!(Some(b'i'), bytes.next());
    assert_eq!(Some(b'n'), bytes.next());


    test_endian("a94a8fe5ccb19ba61c4c0873d391e987982fbbd3");
    test_modulo_bigint();
    test_compare_bigint();

    let mut test_node = node::Node::new("127.0.0.1:34254".parse().unwrap());
    test_node.process_received_msg(
        node::OtherNode::new(
            BigInt::new(Sign::Minus, vec![1]),
            "127.0.0.1:34254".parse().unwrap(),
        ),
        protocols::Message::new(0, Some(0), None),
    );
    test_node.process_received_msg(
        node::OtherNode::new(
            BigInt::new(Sign::Minus, vec![1]),
            "127.0.0.1:34254".parse().unwrap(),
        ),
        protocols::Message::new(1, Some(0), None),
    );
    test_node.process_received_msg(
        node::OtherNode::new(
            BigInt::new(Sign::Minus, vec![1]),
            "127.0.0.1:34254".parse().unwrap(),
        ),
        protocols::Message::new(2, Some(0), None),
    );
    test_node.process_received_msg(
        node::OtherNode::new(
            BigInt::new(Sign::Minus, vec![1]),
            "127.0.0.1:34254".parse().unwrap(),
        ),
        protocols::Message::new(3, Some(0), Some(BigInt::new(Sign::Plus, vec![2]))),
    );
    test_node.process_received_msg(
        node::OtherNode::new(
            BigInt::new(Sign::Minus, vec![1]),
            "127.0.0.1:34254".parse().unwrap(),
        ),
        protocols::Message::new(4, Some(0), None),
    );
    test_node.process_received_msg(
        node::OtherNode::new(
            BigInt::new(Sign::Minus, vec![1]),
            "127.0.0.1:34254".parse().unwrap(),
        ),
        protocols::Message::new(5, Some(0), None),
    );
    // &test_node.start_network();
    test_node.start_update_fingers();

    ip_address_to_string_test();*/
    let node = node::Node::new("127.0.0.1:3000".parse::<SocketAddr>().unwrap());
    //let node2 = node::Node::new("127.0.0.1:3001".parse::<SocketAddr>().unwrap());
    //let node3 = node::Node::new("127.0.0.1:3002".parse::<SocketAddr>().unwrap());

    node.network.start_listening_on_socket();


    //let network1 = network::Network::new("127.0.0.1:34254".parse::<SocketAddr>().unwrap());
    //network1.start_listening_on_socket();

    //let network2 = network::Network::new("127.0.0.1:34255".parse::<SocketAddr>().unwrap());
    //network2.start_listening_on_socket();
    //network2.send_string_to_socket("127.0.0.1:34254".parse::<SocketAddr>().unwrap(),"bla".to_owned());
    //network2.send_string_to_socket("127.0.0.1:34254".parse::<SocketAddr>().unwrap(),"bli".to_owned());
}

fn test_endian(str: &str) {
    let byte_vec = str.as_bytes().to_vec();

    // 3 and 5 work!
    let big_int_no_b = BigInt::from_bytes_be(Sign::NoSign, &byte_vec);
    let big_int_no_l = BigInt::from_bytes_le(Sign::NoSign, &byte_vec);
    let big_int_plus_b = BigInt::from_bytes_be(Sign::Plus, &byte_vec);
    let big_int_plus_l = BigInt::from_bytes_le(Sign::Plus, &byte_vec);
    let big_int_minus_b = BigInt::from_bytes_be(Sign::Minus, &byte_vec);
    let big_int_minus_l = BigInt::from_bytes_le(Sign::Minus, &byte_vec);

    info!("{}", big_int_plus_b);

    let byte_vec_no_b = big_int_no_b.to_bytes_be();
    let byte_vec_no_l = big_int_no_l.to_bytes_be();
    let byte_vec_plus_b = big_int_plus_b.to_bytes_be();
    let byte_vec_plus_l = big_int_plus_l.to_bytes_le();
    let byte_vec_minus_b = big_int_minus_b.to_bytes_be();
    let byte_vec_minus_l = big_int_minus_l.to_bytes_le();

    let str_byte_vec_no_b = std::str::from_utf8(&byte_vec_no_b.1);
    let str_byte_vec_no_l = std::str::from_utf8(&byte_vec_no_l.1);
    let str_byte_vec_plus_b = std::str::from_utf8(&byte_vec_plus_b.1);
    let str_byte_vec_plus_l = std::str::from_utf8(&byte_vec_plus_l.1);
    let str_byte_vec_minus_b = std::str::from_utf8(&byte_vec_minus_b.1);
    let str_byte_vec_minus_l = std::str::from_utf8(&byte_vec_minus_l.1);

    custom_print(str_byte_vec_no_b);
    custom_print(str_byte_vec_no_l);
    custom_print(str_byte_vec_plus_b);
    custom_print(str_byte_vec_plus_l);
    custom_print(str_byte_vec_minus_b);
    custom_print(str_byte_vec_minus_l);
}

fn test_modulo_bigint() {
    let should_be_two = BigInt::modpow(
        &12.to_bigint().unwrap(),
        &1.to_bigint().unwrap(),
        &10.to_bigint().unwrap(),
    );
    info!("{}", should_be_two)
}

fn test_compare_bigint() {
    let one = &1.to_bigint().unwrap();
    let two = &2.to_bigint().unwrap();
    let two_again = &2.to_bigint().unwrap();
    let three = &3.to_bigint().unwrap();

    info!("{}", two == two_again);
    info!("{}", two < three);
    info!("{}", two > one);
}

fn custom_print(result: Result<&str, std::str::Utf8Error>) {
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
