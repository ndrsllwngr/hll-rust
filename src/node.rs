use std::net::IpAddr;

use super::finger::FingerTable;
use super::storage::Storage;

const NOTIFY_PREDECESSOR: i32 = 0;
const NOTIFY_SUCCESSOR: i32 = 1;
const NOTIFY_JOIN: i32 = 2;
const FIND_SUCCESSOR: i32 = 3;
const FOUND_SUCCESSOR: i32 = 4;
const MESSAGE: i32 = 5;

#[derive(Clone)]
pub struct Config {
    pub id: Vec<u8>,
    pub ip_addr: IpAddr,
}

pub struct Node<'a> {
    pub config: Config,
    //pub predecessor: &'a Node<'a>,
    // pub finger_table: FingerTable<'a>,
    pub storage: Storage<'a>,
}

pub fn dispatch(_from: i32, _message: i32) {
    let from = _from;
    let message = _message;

    match message {
        // Node notifies successor about predessor
        NOTIFY_PREDECESSOR =>
        /*
         *  predecessor is nil or n'∈(predecessor, n)
         */
        {
            println!("0-NOTIFY_PREDECESSOR")
        }

        // Stabilize
        NOTIFY_SUCCESSOR =>
        /*
         *  n.stabilize()
         *    x = successor.predecessor;
         *    if (x∈(n, successor))
         *      successor = x;
         *    successor.notify(n);
         */
        {
            println!("1-NOTIFY_SUCCESSOR")
        }
        //
        FIND_SUCCESSOR => println!("3-FIND_SUCCESSOR"),
        FOUND_SUCCESSOR => println!("4-FOUND_SUCCESSOR"),
        MESSAGE => println!("5-MESSAGE"),
        _ => println!("Unknown Chord message: {}", message),
    }
}
