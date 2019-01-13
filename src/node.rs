use std::net::IpAddr;

use super::finger::FingerTable;
use super::storage::Storage;

const CHORD_STATES_NOTIFY_PREDECESSOR: i32 = 0;
const CHORD_STATES_NOTIFY_SUCCESSOR: i32 = 1;
const CHORD_STATES_NOTIFY_JOIN: i32 = 2;
const CHORD_STATES_FIND_SUCCESSOR: i32 = 3;
const CHORD_STATES_FOUND_SUCCESSOR: i32 = 4;
const CHORD_STATES_MESSAGE: i32 = 5;

#[derive(Clone)]
pub struct Config {
    pub id: Vec<u8>,
    pub ip_addr: IpAddr
}

pub struct Node<'a> {
    pub config: Config,
    //pub predecessor: &'a Node<'a>,
    // pub finger_table: FingerTable<'a>,
    pub storage: Storage<'a>
}

fn dispatch (_from: i32, _message: i32) {

    let from = _from;
    let message = _message;

    match message {
        CHORD_STATES_NOTIFY_PREDECESSOR => println!("0"),
        CHORD_STATES_NOTIFY_SUCCESSOR => println!("1"),
        CHORD_STATES_NOTIFY_JOIN => println!("2"),
        CHORD_STATES_FIND_SUCCESSOR => println!("3"),
        CHORD_STATES_FOUND_SUCCESSOR => println!("4"),
        CHORD_STATES_MESSAGE => println!("5"),
        _ => println!("NO MATCH!"),
    }
}