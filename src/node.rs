use std::net::SocketAddr;

use super::finger::FingerTable;
use super::storage::Storage;
use super::network::Network;
use super::util::create_node_id;
use num_bigint::{BigInt, Sign};

const NOTIFY_PREDECESSOR: u8 = 0;
const NOTIFY_SUCCESSOR: u8 = 1;
const NOTIFY_JOIN: u8 = 2;
const FIND_SUCCESSOR: u8 = 3;
const FOUND_SUCCESSOR: u8 = 4;
const MESSAGE: u8 = 5;

pub struct OtherNode {
    id: BigInt,
    ip_addr: SocketAddr,
}

impl OtherNode {
    pub fn new(id: BigInt, ip: SocketAddr) -> OtherNode {
        return OtherNode { id, ip_addr: ip };
    }
}

pub struct Node {
    id: BigInt,
    ip_addr: SocketAddr,
    //TODO check if better possibilities available
    predecessor: Option<OtherNode>,
    successor: OtherNode,
    //TODO can be found out by finger table
    finger_table: FingerTable,
    storage: Storage,
    network: Network,
}

impl Node {
    //Constructor for initialisation of new Chord Ring, call new_existing_network if joining existing network
    pub fn new(ip: SocketAddr) -> Node {
        let id = create_node_id(ip);
        let successor = OtherNode::new(id.clone(), ip);
        let finger_table = FingerTable::new();
        let storage = Storage::new();
        /*  TODO fix when new is implemented
            TODO In addition to that we need to check how network cann call methods on node, particularly: process_received_msg
        */
        let network = Network::new(1234);

        return Node {
            id,
            ip_addr: ip,
            predecessor: None,
            successor,
            finger_table,
            storage,
            network,
        };
    }

    //TODO check if needs to be pulic method, assumption: No ;)
    pub fn to_other_node(self) -> OtherNode {
        return OtherNode {
            id: self.id,
            ip_addr: self.ip_addr,
        };
    }

    pub fn send_msg(self, from: OtherNode, to: OtherNode, message: String) {
        //TODO build JSON Object, and send it as message

        self.network.send(message, to.ip_addr);
    }

    // @andreasellw das is deine dispatch methode, ich glaub das macht hier und mit dem namen mehr sinn,
    // vll hab ich sie aber auch falsch verstanden ;)
    pub fn process_received_msg(_from: i32, _message: u8) {
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
}

