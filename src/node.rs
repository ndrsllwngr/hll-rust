use std::net::IpAddr;

use super::finger::FingerTable;
use super::storage::Storage;
use super::util::create_hash;

use num_bigint::{BigInt, Sign};


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
    successor:  OtherNode,
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
            successor: successor,
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