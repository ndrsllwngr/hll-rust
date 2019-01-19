use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::net::SocketAddr;
use std::{thread, time};

use super::finger::FingerTable;
use super::network::Network;
use super::protocols::*;
use super::storage::Storage;
use super::util::*;

/// Simple representation of an external node in the network
#[derive(Clone, Serialize, Deserialize)]
pub struct OtherNode {
    id: BigInt,
    ip_addr: SocketAddr,
}

impl OtherNode {
    pub fn new(id: BigInt, ip: SocketAddr) -> OtherNode {
        OtherNode { id, ip_addr: ip }
    }

    pub fn get_id(&self) -> &BigInt {
        &self.id
    }

    pub fn get_ip_addr(&self) -> &SocketAddr {
        &self.ip_addr
    }

    pub fn print(&self, desc: &str) {
        info!("{}: id: {}, ip_addr: {}", desc, self.id, self.ip_addr);
    }
}

/// Complete representation of internal node
///
/// * `id`           - Identifier of node: Currently SHA1 hashed IP address
/// * `ip_addr`      - Ip address and port of the node
/// * `network`      - The network parameters of the node. Each node stores it's own network
/// * `finger_table` - Finger table of the node, which stores up to n other nodes
/// * `next_finger`  - Used to point on the entry of the finger table, we are currently processing
/// * `successor`    - Successor of the node //TODO can be found out by finger table, //TODO do we need var finger_entries (e.g. 32 or 8)
/// * `predecessor`  - [Optional] Predecessor of the node
/// * `storage`      - DHT storage inside the node
pub struct Node {
    id: BigInt,
    ip_addr: SocketAddr,
    pub network: Network,
    finger_table: FingerTable,
    next_finger: usize,
    successor: OtherNode,
    predecessor: Option<OtherNode>,
    storage: Storage,
}

/// `Nodè` implementation
impl Node {
    /// Creates new Node
    /// if `predecessor` is None, the node will initialize a new chord ring
    /// if `predecessor` is Some(), the node will join an existing network and `predecessor` as its own predecessor
    ///
    /// * `ip_addr`     - Ip address and port of the node
    /// * `predecessor` - (Optional) Ip address and port of a known member of an existing network
    // TODO implement predecessor: Option<SocketAddr>
    pub fn new(ip_addr: SocketAddr, predecessor: Option<SocketAddr>) -> Node {
        let id = create_node_id(ip_addr);
        let network = Network::new(ip_addr);
        let finger_table = FingerTable::new();
        /// Always start at first entry of finger_table
        let next_finger = 0;
        let successor = OtherNode::new(id.clone(), ip_addr);

        let storage = Storage::new();
        // TODO In addition to that we need to check how network can call methods on node, particularly: process_received_msg
        // Solution: pass reference of node to network
        Node {
            id,
            ip_addr,
            network,
            finger_table,
            next_finger,
            successor,
            predecessor: None,
            storage,
        }
    }

    /// Converts internal representation of node to the simpler representation OtherNode
    fn to_other_node(&self) -> OtherNode {
        OtherNode {
            id: self.id.clone(),
            ip_addr: self.ip_addr,
        }
    }

    /// Gets closet preceding
    pub fn closet_finger_preceding(&self, find_id: &BigInt) -> OtherNode {
        /// ```
        /// n.closest_preceding_node(id)
        ///   for i = m downto 1
        ///     if (finger[i]∈(n,id))
        ///       return finger[i];
        ///   return n;
        /// ```
        for x in self.finger_table.length()..0 {
            let finger_entry = self.finger_table.get(x);
            if let Some(finger_entry) = finger_entry {
                if is_in_range(finger_entry.node.get_id(), &self.id, &find_id) {
                    return finger_entry.node.clone();
                }
            }
        }

        if is_in_range(&self.successor.id, &self.id, &find_id) {
            self.successor.clone()
        } else {
            self.to_other_node()
        }
    }

    /// Entry point after creation of node
    /// Loops periodically to update fingertable
    /// Calls fix_fingers
    /// Notifies successor that I am his predecessor by sending NOTIFY_PREDECESSOR
    pub fn start_update_fingers(&mut self) {
        loop {
            self.fix_fingers();
            let message = Message::new(NOTIFY_PREDECESSOR, None, None);
            self.send_msg(self.successor.clone(), None, message);
            info!("start_update_fingers");
            thread::sleep(time::Duration::from_millis(2000));
        }
    }

    /// Periodically find successor for all entries of our fingertable
    /// Sending self a message which subsequently sends messages to others
    /// by dispatching FIND_SUCCESSOR message to other nodes
    fn fix_fingers(&mut self) {
        let fix_finger_id: BigInt;
        let mut next = self.next_finger;
        if next >= self.finger_table.length() {
            next = 0;
        }
        fix_finger_id = get_fix_finger_id(&self.id, next);
        self.next_finger = next + 1;
        // n.fix_fingers()
        let message = Message::new(FIND_SUCCESSOR, Some(next), Some(fix_finger_id));
        self.send_msg(self.to_other_node(), None, message);
    }

    /// TODO WTH
    /// Trys to join existing chord network by notifing
    pub fn join(&mut self, remote: OtherNode) -> bool {
        let message = Message::new(NOTIFY_JOIN, None, None);
        self.predecessor = None;
        remote.print("Try to join");
        self.send_msg(remote, None, message);
        true
    }

    pub fn send_msg(&self, _from: OtherNode, _to: Option<OtherNode>, _message: Message) {
        let from = _from;

        let to = match _to {
            Some(to) => to,
            None => from.clone(),
        };

        let mut message = _message;
        if message.get_id().is_none() {
            message.set_id(Some(self.id.clone()))
        }

        let json_string = serde_json::to_string(&message).unwrap();
        // let json_string_other_node = serde_json::to_string(&from).unwrap();
        // let parsed_node: OtherNode = serde_json::from_str(&json_string_other_node).unwrap();
        // let parsed_message: Message = serde_json::from_str(custom_json).unwrap();

        self.network.send(&from, &to, &message);
    }

    //TODO find better name
    pub fn start_network(self) {
        self.network.start_listening_on_socket();
    }

    pub fn process_received_msg(&mut self, _from: OtherNode, _message: Message) {
        let from = _from;
        let mut message = _message;

        match message.get_message_type() {
            // Notifies successor about myself that I am the predecessor
            NOTIFY_PREDECESSOR =>
            /*
             *  predecessor is nil or n'∈(predecessor, n)
             */
            {
                info!("0-NOTIFY_PREDECESSOR");
                message.print();

                let predecessor_to_notify = match self.predecessor.clone() {
                    Some(predecessor) => {
                        if is_in_range(&from.id, &predecessor.id, &self.id) {
                            from.print("New predecessor ist now");
                            self.predecessor = Some(from.clone());
                            from.clone()
                        } else {
                            predecessor
                        }
                    }
                    None => {
                        from.print("New predecessor ist now");
                        self.predecessor = Some(from.clone());
                        from.clone()
                    }
                };
                self.send_msg(predecessor_to_notify, Some(from), message);
                self.finger_table.print()
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
                info!("1-NOTIFY_SUCCESSOR");
                message.print();

                if is_in_range(&from.id, &self.id, &self.successor.id) {
                    self.successor = from;
                    self.successor.print("New succesor is now");
                }
            }
            NOTIFY_JOIN => {
                info!("2-NOTIFY_JOIN");
                message.print();
                from.print("Node joined");
            }
            FIND_SUCCESSOR => {
                info!("3-FIND_SUCCESSOR");
                message.print();
                if let Some(id) = message.get_id() {
                    if is_in_half_range(&id, &self.id, &self.successor.id) {
                        self.successor.print("FIND_SUCCESSOR");
                        message.set_message_type(FOUND_SUCCESSOR);
                        self.send_msg(self.successor.clone(), Some(from), message);
                    } else {
                        let node_0 = self.closet_finger_preceding(&id);
                        self.successor
                            .print("FIND_SUCCESSOR = closet_finger_preceding");
                        message.set_message_type(FOUND_SUCCESSOR);
                        self.send_msg(node_0, Some(from), message);
                    }
                };
            }
            FOUND_SUCCESSOR => {
                info!("4-FOUND_SUCCESSOR");
                message.print();

                match (message.get_next_finger(), message.get_id()) {
                    (Some(next_finger), Some(id)) => {
                        self.finger_table.put(next_finger, id, from);
                        info!("FingerTable fixed");
                    }
                    _ => {
                        self.successor = from;
                        self.successor.print("New successor is now");
                    }
                }
            }
            MESSAGE => {
                info!("5-MESSAGE");
                self.send_msg(self.successor.clone(), Some(from), message);
            }
            _ => {
                warn!("Unknown chord message!");
                message.print();
            }
        }
    }
}
