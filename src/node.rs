use std::net::IpAddr;

use super::finger::FingerTable;
use super::storage::Storage;
use super::util::create_hash;
use num_bigint::{BigInt, Sign};

const NOTIFY_PREDECESSOR: u8 = 0;
const NOTIFY_SUCCESSOR: u8 = 1;
const NOTIFY_JOIN: u8 = 2;
const FIND_SUCCESSOR: u8 = 3;
const FOUND_SUCCESSOR: u8 = 4;
const MESSAGE: u8 = 5;

pub struct OtherNode {
    id: BigInt,
    ip_addr: IpAddr,
}

impl OtherNode {
    pub fn new(id: BigInt, ip: IpAddr) -> OtherNode {
        return OtherNode { id, ip_addr: ip };
    }
}

pub struct Node {
    id: BigInt,
    ip_addr: IpAddr,
    //TODO check if better possibilities available
    predecessor: Option<OtherNode>,
    successor: OtherNode,
    //TODO can be found out by finger table
    finger_table: FingerTable,
    storage: Storage,
}

impl Node {
    //Constructor for initialisation of new Chord Ring, call new_existing_network if joining existing network
    pub fn new(ip: IpAddr) -> Node {
        let id = create_node_id(ip);
        let successor = OtherNode::new(id.clone(), ip);
        let finger_table = FingerTable::new();
        let storage = Storage::new();

        return Node {
            id,
            ip_addr: ip,
            predecessor: None,
            successor,
            finger_table,
            storage,
        };
    }

    /*TODO
    pub fn new_existing_network(own_ip: IpAddr, joining_node_ip: IpAddr) -> Node{

    }*/
}

fn create_node_id(ip: IpAddr) -> BigInt {
    let hash = create_hash(&ip.to_string());
    let byte_vec = hash.as_bytes().to_vec();
    return BigInt::from_bytes_be(Sign::Plus, &byte_vec);
}

pub fn dispatch(_from: i32, _message: u8) {
    let from = _from;
    let message = _message;

    match message {
        // Node notifies successor about predessor
        NOTIFY_PREDECESSOR =>
        /*
         *  predecessor is nil or n'∈(predecessor, n)
         */
        {
            info!("0-NOTIFY_PREDECESSOR")
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
            info!("1-NOTIFY_SUCCESSOR")
        }
        NOTIFY_JOIN => info!("Node joined: {}", from),
        FIND_SUCCESSOR => info!("3-FIND_SUCCESSOR"),
        FOUND_SUCCESSOR => 
        {
            // TODO this.send(this.successor, message, from);
            info!("4-FOUND_SUCCESSOR")
        }
        MESSAGE => info!("5-MESSAGE"),
        _ => info!("Unknown chord message: {}", message),
    }
}
