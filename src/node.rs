use std::{str, thread, time};
use std::net::SocketAddr;

use num_bigint::{BigInt, ToBigInt};

use super::chord;
use super::finger;
use super::finger::FingerTable;
use super::network_util;
use super::protocols::*;
use super::storage::Storage;
use super::util::*;

/// Simple representation of an external node in the network
#[derive(Clone, Serialize, Deserialize, Debug)]
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
}

/// Complete representation of internal node
///
/// * `id`             - Identifier of node: Currently SHA1 hashed IP address
/// * `ip_addr`        - Ip address and port of the node
/// * `finger_table`   - Finger table of the node, which stores up to n other nodes
/// * `next_finger`    - Used to point on the entry of the finger table, we are currently processing
/// * `successor`      - Successor of the node //TODO can be found out by finger table, //TODO do we need var finger_entries (e.g. 32 or 8) -> Not really, finger_entries depend on bit_size of hashing!!
/// * `predecessor`    - [Optional] Predecessor of the node
/// * `storage`        - DHT storage inside the node
#[derive(Clone)]
pub struct Node {
    //TODO not pup
    pub id: BigInt,
    pub ip_addr: SocketAddr,
    pub finger_table: FingerTable,
    // next_finger: usize,
    pub predecessor: Option<OtherNode>,
    pub joined: bool,
    // storage: Storage,
}

/// `Node` implementation
impl Node {
    /// Creates new Node
    /// TODO fix comments
    /// if `predecessor` is None, the node will initialize a new chord ring
    /// if `predecessor` is Some(), the node will join an existing network and `predecessor` as its own predecessor
    ///
    /// * `ip_addr`     - Ip address and port of the node
    /// * `predecessor` - (Optional) Ip address and port of a known member of an existing network
    pub fn new(node_ip_addr: SocketAddr, entry_node_addr: SocketAddr) -> Node {
        //let next_finger = 0; // Always start at first entry of finger_table
        //let finger_table = FingerTable::new(successor.clone(), &id);
        //let storage = Storage::new();
        let id = create_node_id(node_ip_addr);
        let successor = OtherNode { id: create_node_id(entry_node_addr), ip_addr: entry_node_addr };
        Node {
            id: id.clone(),
            ip_addr: node_ip_addr,
            finger_table: FingerTable::new(successor, &id),
            predecessor: None,
            joined: false,
        }
    }

    pub fn new_first(node_ip_addr: SocketAddr) -> Node {
        let id = create_node_id(node_ip_addr);
        let successor = OtherNode { id: id.clone(), ip_addr: node_ip_addr.clone() };
        Node {
            id: id.clone(),
            ip_addr: node_ip_addr.clone(),
            finger_table: FingerTable::new(successor, &id),
            predecessor: Some(OtherNode { id: id, ip_addr: node_ip_addr }),
            joined: true,
        }
    }

    /// Converts internal representation of node to the simpler representation OtherNode
    pub fn to_other_node(&self) -> OtherNode {
        OtherNode {
            id: self.id.clone(),
            ip_addr: self.ip_addr,
        }
    }

    pub fn get_successor(&self) -> OtherNode {
        self.finger_table.get_successor()
    }

    pub fn set_successor(&mut self, successor: OtherNode) {
        self.finger_table.set_successor(successor);
    }

    pub fn print_current_state(&self) {
        let pre_string = if let Some(pre) = self.predecessor.clone() {
            pre.get_id().to_string()
        } else {
            "None".to_string()
        };
        let string_to_print =
            format!("\n\n{0: <18} | #{1: <6}\n{2: <18} | #{3: <6}\n{4: <18} | #{5: <6}\n",
                    "I am Node".to_string(), self.id,
                    "My Predecessor is".to_string(), pre_string,
                    "My Successor is".to_string(), self.get_successor().id
            );
        info!("{}", string_to_print);
    }

    fn closest_preceding_node(&self, id: BigInt) -> OtherNode {
        let mut min_abs: BigInt = 999999999.to_bigint().unwrap();
        let mut return_node: OtherNode = self.to_other_node();
        for i in 0..self.finger_table.length() {
            let entry = self.finger_table.get(i);
            let finger_abs = chord_abs(&entry.node.id, &id);
            if finger_abs < min_abs {
                min_abs = finger_abs;
                return_node = entry.node.clone()
            }
        }
        return_node
    }

    pub fn process_incoming_request(&mut self, request: Request) -> Response {
        match request {
            Request::FindSuccessor { id } => {
                debug!("[Node #{}] Request::FindSuccessor({})", self.clone().id, id.clone());
                self.handle_find_successor_request(id)
            }
            Request::GetPredecessor => {
                debug!("[Node #{}] Request::GetPredecessor", self.clone().id);
                self.handle_get_predecessor_request()
            }
            Request::Notify { node } => {
                debug!("[Node #{}] Request::Notify(node: {})", self.clone().id, node.id.clone());
                self.handle_notify_request(node)
            }
            Request::FindSuccessorFinger { index, finger_id } => {
                debug!("[Node #{}] Request::FindSuccessorFinger({})", self.clone().id, finger_id.clone());
                self.handle_find_successor_finger_request(index, finger_id)
            }
        }
    }

    pub fn process_incoming_response(&mut self, response: Response) {
        match response {
            Response::FoundSuccessor { successor } => {
                debug!("[Node #{}] Response::FoundSuccessor(successor: {})", self.clone().id, successor.id.clone());
                self.handle_found_successor_response(successor)
            }
            Response::AskFurther { next_node } => {
                debug!("[Node #{}] Response::AskFurther(next_node: {}", self.clone().id, next_node.id.clone());
                self.handle_ask_further_response(next_node)
            }
            Response::GetPredecessorResponse { predecessor } => {
                debug!("[Node #{}] Response::GetPredecessorResponse(predecessor: {:?})", self.clone().id, predecessor.clone());
                self.handle_get_predecessor_response(predecessor)
            }
            Response::NotifyResponse => {
                //debug!("Response::NotifyResponse");
                self.handle_notify_response()
            }
            Response::FoundSuccessorFinger { index, finger_id, successor } => {
                debug!("[Node #{}] Response::FoundSuccessorFinger(successor: {})", self.clone().id, successor.id.clone());
                self.handle_found_successor_finger_response(index, finger_id, successor)
            }
            Response::AskFurtherFinger { index, finger_id, next_node } => {
                debug!("[Node #{}] Response::AskFurtherFinger(next_node: {}", self.clone().id, next_node.id.clone());
                self.handle_ask_further_finger_response(index, finger_id, next_node)
            }
        }
    }

    fn handle_find_successor_request(&self, id: BigInt) -> Response {
        if is_in_interval(&self.id, self.get_successor().get_id(), &id) {
            Response::FoundSuccessor { successor: self.get_successor().clone() }
        } else {
            Response::AskFurther { next_node: self.closest_preceding_node(id) }
        }
    }

    fn handle_get_predecessor_request(&self) -> Response {
        Response::GetPredecessorResponse {
            predecessor: self.predecessor.clone()
        }
    }

    fn handle_notify_request(&mut self, node: OtherNode) -> Response {
        match &self.predecessor {
            None => {
                debug!("[Node #{}] Notify: Had no Pre. Pre is now: {}", self.id, node.id);
                self.predecessor = Some(node)
            }
            Some(pre) => {
                debug!("[Node #{}] Notify: Current Pre: {}, possible new Pre: {}. Successor is: {}", self.id, pre.id, node.id, self.get_successor().id);
                if pre.id != node.id && is_in_interval(pre.get_id(), &self.id, node.get_id()) {
                    self.predecessor = Some(node);
                    debug!("[Node #{}] Took new Pre: {}", self.id, self.predecessor.clone().unwrap().id);
                }
            }
        }
        //TODO check if maybe a failure notification is necessary
        Response::NotifyResponse
    }

    fn handle_find_successor_finger_request(&self, index: usize, finger_id: BigInt) -> Response {
        if is_in_interval(&self.id, self.get_successor().get_id(), &finger_id) {
            Response::FoundSuccessorFinger { index: index, finger_id: finger_id, successor: self.get_successor().clone() }
        } else {
            Response::AskFurtherFinger { index: index, finger_id: finger_id, next_node: self.get_successor() }
        }
    }

    fn handle_found_successor_response(&mut self, successor: OtherNode) {
        debug!("Found my new successor: node #{}", successor.id.clone());
        self.set_successor(successor);
        if !self.joined {
            debug!("Starting of stabilization not yet implemented");
            //TODO self.start_stabilisation();
            self.joined = true;
        }
    }

    fn handle_ask_further_response(&mut self, next_node: OtherNode) {
        debug!("Did not get successor yet, asking node #{} now...", next_node.id);
        let req = Request::FindSuccessor { id: self.id.clone() };

        let msg = Message::RequestMessage { sender: self.to_other_node(), request: req };
        network_util::send_string_to_socket(next_node.ip_addr, serde_json::to_string(&msg).unwrap());
    }

    fn handle_get_predecessor_response(&mut self, predecessor: Option<OtherNode>) {
        if let Some(predecessor) = predecessor {
            // maybe update my successor:
            if predecessor.get_id() != &self.id &&
                is_in_interval(&self.id, self.get_successor().get_id(), predecessor.get_id()) {
                debug!("[Node #{}] GetPreResp: Had succ #{}, got pre #{}, new succ: #{}", self.id.clone(), self.get_successor().id.clone(), predecessor.id.clone(), predecessor.id.clone());
                self.set_successor(predecessor);
            }
        }
        let req = Request::Notify { node: self.to_other_node() };

        let msg = Message::RequestMessage { sender: self.to_other_node(), request: req };
        network_util::send_string_to_socket(self.get_successor().ip_addr.clone(), serde_json::to_string(&msg).unwrap());
    }

    fn handle_notify_response(&self) {}

    fn handle_found_successor_finger_response(&mut self, index: usize, finger_id: BigInt, successor: OtherNode) {
        debug!("Found node for finger_id {}: node #{}", finger_id.clone(), successor.id.clone());

        self.finger_table.put(index, finger_id, successor);
        if index == chord::FINGERTABLE_SIZE - 1 {
            self.finger_table.print(self.id.clone());
        }
    }

    fn handle_ask_further_finger_response(&mut self, index: usize, finger_id: BigInt, next_node: OtherNode) {
        debug!("Did not get entry for finger {} (#{}) yet, asking node #{} now...", finger_id.clone(), index.clone(), next_node.id);
        let req = Request::FindSuccessorFinger { index: index, finger_id: finger_id };

        let msg = Message::RequestMessage { sender: self.to_other_node(), request: req };
        network_util::send_string_to_socket(next_node.ip_addr, serde_json::to_string(&msg).unwrap());
    }
}
