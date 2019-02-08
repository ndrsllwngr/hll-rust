use std::collections::HashMap;
use std::net::SocketAddr;
use std::str;

use num_bigint::{BigInt, Sign};

use super::chord;
use super::fingertable::FingerTable;
use super::network;
use super::protocols::*;
use super::storage::{DHTEntry, Storage};

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
    id: BigInt,
    ip_addr: SocketAddr,
    finger_table: FingerTable,
    predecessor: Option<OtherNode>,
    successor_list: Vec<OtherNode>,
    storage: Storage,
    joined: bool,
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
    pub fn new(node_ip_addr: SocketAddr) -> Node {
        let id = chord::create_node_id(node_ip_addr);
        Node {
            id: id.clone(),
            ip_addr: node_ip_addr,
            finger_table: FingerTable::new(id.clone()),
            predecessor: None,
            successor_list: Vec::with_capacity(chord::SUCCESSORLIST_SIZE),
            storage: Storage::new(),
            joined: false,
        }
    }

    pub fn new_first(node_ip_addr: SocketAddr) -> Node {
        let id = chord::create_node_id(node_ip_addr);
        let successor = OtherNode::new(id.clone(), node_ip_addr);
        Node {
            id: id.clone(),
            ip_addr: node_ip_addr,
            finger_table: FingerTable::new_first(id.clone(), successor.clone()),
            predecessor: Some(OtherNode::new(id, node_ip_addr)),
            successor_list: vec![successor],
            storage: Storage::new(),
            joined: true,
        }
    }

    pub fn get_id(&self) -> &BigInt {
        &self.id
    }
    pub fn get_ip_addr(&self) -> &SocketAddr {
        &self.ip_addr
    }

    pub fn get_finger_table(&self) -> &FingerTable {
        &self.finger_table
    }

    pub fn get_successor(&self) -> OtherNode {
        self.finger_table.get_successor()
    }

    pub fn get_predecessor(&self) -> &Option<OtherNode> {
        &self.predecessor
    }

    pub fn set_predecessor(&mut self, predecessor: Option<OtherNode>) {
        self.predecessor = predecessor.clone();

        // Redistribute keys, that I am not responsible for anymore
        if let Some(pre) = predecessor {
            self.check_redistribute_dht_keys(&pre.id)
        }
    }

    pub fn get_successor_list(&self) -> &Vec<OtherNode> {
        &self.successor_list
    }

    pub fn get_storage(&self) -> &Storage {
        &self.storage
    }

    pub fn is_joined(&self) -> bool {
        self.joined
    }

    /// Converts internal representation of node to the simpler representation OtherNode
    pub fn to_other_node(&self) -> OtherNode {
        OtherNode::new(self.id.clone(), self.ip_addr)
    }

    pub fn update_successor_and_successor_list(&mut self, successor: OtherNode) {
        //if self.finger_table.length() == 0  || &self.get_successor().id != &successor.id {
        self.finger_table.set_successor(successor.clone());
        let req = Request::GetSuccessorList;
        let msg = Message::RequestMessage { sender: self.to_other_node(), request: req };
        network::send_string_to_socket(*successor.get_ip_addr(), serde_json::to_string(&msg).unwrap());
        //}
    }

    pub fn move_all_keys_on_shutdown(&self) {
        if self.joined && self.storage.is_data_empty() {
            let req = Request::DHTTakeOverKeys { data: self.storage.get_data_as_vec().clone() };
            let msg = Message::RequestMessage { sender: self.to_other_node(), request: req };
            network::send_string_to_socket(self.get_successor().get_ip_addr().clone(), serde_json::to_string(&msg).unwrap());
        }
    }

    fn check_redistribute_dht_keys(&mut self, pre_id: &BigInt) {
        for (key, value) in self.storage.clone().get_data_as_iter() {
            if !chord::is_my_key(&self.id, pre_id, key) {
                let req = Request::DHTStoreKey { data: (key.clone(), value.clone()) };
                let msg = Message::RequestMessage { sender: self.to_other_node(), request: req };
                network::send_string_to_socket(self.ip_addr, serde_json::to_string(&msg).unwrap());
                self.storage.delete_key(key);
            }
        }
    }

    fn closest_preceding_node(&self, id: BigInt) -> OtherNode {
        let mut min_abs: BigInt = BigInt::new(Sign::Plus, vec![u32::max_value(); 5]);
        let mut return_node: OtherNode = self.to_other_node();
        for i in 0..self.finger_table.length() {
            let entry = self.finger_table.get(i);
            let finger_abs = chord::chord_abs(entry.get_node().get_id(), &id);
            if finger_abs < min_abs {
                min_abs = finger_abs;
                return_node = entry.get_node().clone()
            }
        }
        for i in 0..self.successor_list.len() {
            let entry = &self.successor_list[i];
            if entry.id == self.id {
                break;
            } else {
                let finger_abs = chord::chord_abs(entry.get_id(), &id);
                if finger_abs < min_abs {
                    min_abs = finger_abs;
                    return_node = entry.clone()
                }
            }
        }
        return_node
    }

    pub fn process_incoming_request(&mut self, request: Request) -> Option<Response> {
        match request {
            Request::FindSuccessor { id } => {
                debug!("[Node #{}] Request::FindSuccessor(id: {})", self.clone().id, id.clone());
                Some(self.handle_find_successor_request(id))
            }
            Request::GetPredecessor => {
                debug!("[Node #{}] Request::GetPredecessor", self.clone().id);
                Some(self.handle_get_predecessor_request())
            }
            Request::Notify { node } => {
                debug!("[Node #{}] Request::Notify(node: {})", self.clone().id, node.id.clone());
                Some(self.handle_notify_request(node))
            }
            Request::FindSuccessorFinger { index, finger_id } => {
                debug!("[Node #{}] Request::FindSuccessorFinger(index: {} finger_id: {})", self.clone().id, index, finger_id.clone());
                Some(self.handle_find_successor_finger_request(index, finger_id))
            }
            Request::GetSuccessorList => {
                debug!("[Node #{}] Request::GetSuccessorList", self.clone().id);
                Some(self.handle_get_successor_list_request())
            }
            Request::DHTStoreKey { data } => {
                info!("[Node #{}] Request::StoreKey(data: {:?})", self.clone().id, data.clone());
                Some(self.handle_dht_store_key_request(data))
            }
            Request::DHTFindKey { key_id } => {
                info!("[Node #{}] Request::FindKey(key_id: {})", self.clone().id, key_id.clone());
                Some(self.handle_dht_find_key_request(key_id))
            }
            Request::DHTDeleteKey { key_id } => {
                info!("[Node #{}] Request::DeleteKey(key_id {})", self.clone().id, key_id.clone());
                Some(self.handle_dht_delete_key_request(key_id))
            }
            Request::DHTTakeOverKeys { data } => {
                info!("[Node #{}] Request::DHTTakeOverKey(data {:?})", self.clone().id, data.clone());
                self.handle_dht_take_over_keys(data);
                None
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
                debug!("Response::NotifyResponse");
                self.handle_notify_response()
            }
            Response::FoundSuccessorFinger { index, finger_id, successor } => {
                debug!("[Node #{}] Response::FoundSuccessorFinger(index: {}, finger_id: {}, successor: {})",
                       self.clone().id, index, finger_id.clone(), successor.id.clone());
                self.handle_found_successor_finger_response(index, finger_id, successor)
            }
            Response::AskFurtherFinger { index, finger_id, next_node } => {
                debug!("[Node #{}] Response::AskFurtherFinger(next_node: {}", self.clone().id, next_node.id.clone());
                self.handle_ask_further_finger_response(index, finger_id, next_node)
            }
            Response::GetSuccessorListResponse { successor_list } => {
                debug!("[Node #{}] Response::GetSuccessorListResponse(successor_list: {:?}",
                       self.clone().id, successor_list.clone());
                self.handle_get_successor_list_response(successor_list)
            }
            Response::DHTStoredKey => {
                debug!("[Node #{}] Response::DHTStoredKey", self.clone().id);
                self.handle_dht_stored_key_response()
            }
            Response::DHTFoundKey { data } => {
                debug!("[Node #{}] Response::DHTFoundKey(data: {:?})", self.clone().id, data.clone());
                self.handle_dht_found_key_response(data)
            }
            Response::DHTDeletedKey { key_existed } => {
                debug!("[Node #{}] Response::DHTDeletedKey(key_existed: {})", self.clone().id, key_existed);
                self.handle_dht_deleted_key_response(key_existed)
            }
            Response::DHTAskFurtherStore { next_node, data } => {
                info!("[Node #{}] Response::DHTAskFurtherStore(next_node: {}, data: {:?})",
                      self.clone().id, next_node.get_id().clone(), data);
                self.handle_dht_ask_further_store_response(next_node, data)
            }
            Response::DHTAskFurtherFind { next_node, key_id } => {
                info!("[Node #{}] Response::DHTAskFurtherFind(next_node: {}, key_id: {})",
                      self.clone().id, next_node.get_id().clone(), key_id.clone());
                self.handle_dht_ask_further_find_response(next_node, key_id)
            }
            Response::DHTAskFurtherDelete { next_node, key_id } => {
                info!("[Node #{}] Response::DHTAskFurtherDelete(next_node: {}, key_id: {})",
                      self.clone().id, next_node.get_id().clone(), key_id.clone());
                self.handle_dht_ask_further_delete_response(next_node, key_id)
            }
        }
    }

    // REQUESTS

    fn handle_find_successor_request(&self, id: BigInt) -> Response {
        if chord::is_in_interval(&self.id, self.get_successor().get_id(), &id) {
            Response::FoundSuccessor { successor: self.get_successor().clone() }
        } else if let Some(pre) = self.predecessor.clone() {
            if chord::is_in_interval(pre.get_id(), &self.id, &id) {
                Response::FoundSuccessor { successor: self.to_other_node() }
            } else {
                Response::AskFurther { next_node: self.closest_preceding_node(id) }
            }
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
                self.set_predecessor(Some(node))
            }
            Some(pre) => {
                debug!("[Node #{}] Notify: Current Pre: {}, possible new Pre: {}. Successor is: {}", self.id, pre.id, node.id, self.get_successor().id);
                if pre.id != node.id && chord::is_in_interval(pre.get_id(), &self.id, node.get_id()) {
                    self.set_predecessor(Some(node));
                    debug!("[Node #{}] Took new Pre: {}", self.id, self.predecessor.clone().unwrap().id);
                }
            }
        }
        //TODO check if maybe a failure notification is necessary
        Response::NotifyResponse
    }

    fn handle_find_successor_finger_request(&self, index: usize, finger_id: BigInt) -> Response {
        if chord::is_in_interval(&self.id, self.get_successor().get_id(), &finger_id) {
            Response::FoundSuccessorFinger { index, finger_id, successor: self.get_successor().clone() }
        } else {
            Response::AskFurtherFinger { index, finger_id, next_node: self.get_successor() }
        }
    }

    fn handle_get_successor_list_request(&self) -> Response {
        Response::GetSuccessorListResponse { successor_list: self.successor_list.clone() }
    }

    fn handle_dht_store_key_request(&mut self,
                                    data: (BigInt, DHTEntry)) -> Response {
        if let Some(predecessor) = self.predecessor.clone() {
            // I am responsible for the key
            if chord::is_my_key(&self.id, predecessor.get_id(), &data.0) {
                self.storage.store_key(data);
                Response::DHTStoredKey
            } else {
                Response::DHTAskFurtherStore {
                    next_node: self.closest_preceding_node(data.0.clone()),
                    data,
                }
            }
        } else {
            Response::DHTAskFurtherStore {
                next_node: self.closest_preceding_node(data.0.clone()),
                data,
            }
        }
    }

    fn handle_dht_find_key_request(&self, key_id: BigInt) -> Response {
        if let Some(predecessor) = self.predecessor.clone() {
            // I am responsible for the key
            if chord::is_my_key(&self.id, predecessor.get_id(), &key_id) {
                let value_option = self.storage.get_key(&key_id);
                Response::DHTFoundKey { data: (key_id, value_option.cloned()) }
            } else {
                Response::DHTAskFurtherFind {
                    next_node: self.closest_preceding_node(key_id.clone()),
                    key_id,
                }
            }
        } else {
            Response::DHTAskFurtherFind {
                next_node: self.closest_preceding_node(key_id.clone()),
                key_id,
            }
        }
    }

    fn handle_dht_delete_key_request(&mut self, key_id: BigInt) -> Response {
        if let Some(predecessor) = self.predecessor.clone() {
            // I am responsible for the key
            if chord::is_my_key(&self.id, predecessor.get_id(), &key_id) {
                let key_existed = self.storage.delete_key(&key_id).is_some();
                Response::DHTDeletedKey { key_existed }
            } else {
                Response::DHTAskFurtherDelete {
                    next_node: self.closest_preceding_node(key_id.clone()),
                    key_id,
                }
            }
        } else {
            Response::DHTAskFurtherDelete {
                next_node: self.closest_preceding_node(key_id.clone()),
                key_id,
            }
        }
    }

    fn handle_dht_take_over_keys(&mut self, data: Vec<(BigInt, DHTEntry)>) {
        for entry in data {
            self.storage.store_key(entry);
        }
    }


    // RESPONSES

    fn handle_found_successor_response(&mut self, successor: OtherNode) {
        debug!("Found my new successor: node #{}", successor.id.clone());
        self.update_successor_and_successor_list(successor);
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
        network::send_string_to_socket(next_node.ip_addr, serde_json::to_string(&msg).unwrap());
    }

    fn handle_get_predecessor_response(&mut self, predecessor: Option<OtherNode>) {
        if let Some(predecessor) = predecessor {
            // maybe update my successor:
            if predecessor.get_id() != &self.id &&
                chord::is_in_interval(&self.id, self.get_successor().get_id(), predecessor.get_id()) {
                debug!("[Node #{}] GetPreResp: Had succ #{}, got pre #{}, new succ: #{}", self.id.clone(), self.get_successor().id.clone(), predecessor.id.clone(), predecessor.id.clone());
                self.update_successor_and_successor_list(predecessor);
            }
        }
        let req = Request::Notify { node: self.to_other_node() };

        let msg = Message::RequestMessage { sender: self.to_other_node(), request: req };
        network::send_string_to_socket(self.get_successor().ip_addr, serde_json::to_string(&msg).unwrap());
    }

    fn handle_notify_response(&self) {}

    fn handle_found_successor_finger_response(&mut self, index: usize, finger_id: BigInt, successor: OtherNode) {
        debug!("Found node for finger_id {}: node #{}", finger_id.clone(), successor.id.clone());

        self.finger_table.put(index, finger_id, successor);
        if index == chord::FINGERTABLE_SIZE - 1 {
            //self.finger_table.print(self.id.clone());
        }
    }

    fn handle_ask_further_finger_response(&mut self, index: usize, finger_id: BigInt, next_node: OtherNode) {
        debug!("Did not get entry for finger {} (#{}) yet, asking node #{} now...", finger_id.clone(), index, next_node.id);
        let req = Request::FindSuccessorFinger { index, finger_id };

        let msg = Message::RequestMessage { sender: self.to_other_node(), request: req };
        network::send_string_to_socket(next_node.ip_addr, serde_json::to_string(&msg).unwrap());
    }

    fn handle_get_successor_list_response(&mut self, successor_list: Vec<OtherNode>) {
        let mut new_successor_list = vec![self.get_successor().clone()];
        if successor_list.len() == chord::SUCCESSORLIST_SIZE {
            new_successor_list.append(&mut successor_list.clone()[..(successor_list.len() - 1)].to_owned())
        } else {
            new_successor_list.append(&mut successor_list.clone())
        };
        self.successor_list = new_successor_list;
    }

    fn handle_dht_stored_key_response(&mut self) {
        self.storage.write_log_entry("Key stored".to_string());
        debug!("Key stored");
    }

    fn handle_dht_found_key_response(&mut self, data: (BigInt, Option<DHTEntry>)) {
        if let Some(dht_entry) = data.1.clone() {
            self.storage.write_log_entry(format!("Value for key {} (id: {}) is {}", dht_entry.get_key(), data.0, dht_entry.get_value()));
            debug!("Value for key {} (id: {}) is {}", dht_entry.get_key(), data.0, dht_entry.get_value());
        } else {
            self.storage.write_log_entry(format!("No value for key_id {} found in the network", data.0));
            debug!("No value for key_id {} found in the network", data.0)
        }
    }

    fn handle_dht_deleted_key_response(&mut self, key_existed: bool) {
        if key_existed {
            self.storage.write_log_entry("Key deleted".to_string());
            info!("Key deleted");
        } else {
            self.storage.write_log_entry("Tried to delete key but the key was not present in the network".to_string());
            debug!("Tried to delete key but the key was not present in the network");
        }
    }

    fn handle_dht_ask_further_store_response(&self,
                                             next_node: OtherNode,
                                             data: (BigInt, DHTEntry)) {
        debug!("Did not store data {:?} yet, asking node #{} now...", data, next_node.id);
        let req = Request::DHTStoreKey { data };
        let msg = Message::RequestMessage { sender: self.to_other_node(), request: req };

        network::send_string_to_socket(next_node.ip_addr, serde_json::to_string(&msg).unwrap());
    }

    fn handle_dht_ask_further_find_response(&self,
                                            next_node: OtherNode,
                                            key_id: BigInt) {
        debug!("Did not find key {} yet, asking node #{} now...", key_id, next_node.id);
        let req = Request::DHTFindKey { key_id };
        let msg = Message::RequestMessage { sender: self.to_other_node(), request: req };

        network::send_string_to_socket(next_node.ip_addr, serde_json::to_string(&msg).unwrap());
    }

    fn handle_dht_ask_further_delete_response(&self,
                                              next_node: OtherNode,
                                              key_id: BigInt) {
        debug!("Did not find key {} yet, asking node #{} now...", key_id, next_node.id);
        let req = Request::DHTDeleteKey { key_id };
        let msg = Message::RequestMessage { sender: self.to_other_node(), request: req };

        network::send_string_to_socket(next_node.ip_addr, serde_json::to_string(&msg).unwrap());
    }
}
