use std::str::Bytes;
use std::net::{IpAddr, Ipv4Addr};
use std::collections::HashMap;

mod node;
mod storage;
mod finger;

fn main() {
    println!("Hello, world!");

    let id = "node_id".bytes();
    let ip_addr = IpAddr::V4(Ipv4Addr::new(127,0,0,1));
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
}
