use num_bigint::BigInt;

use tokio::io;
use tokio::net::{TcpStream, TcpListener};
use tokio::prelude::*;

use futures::{Future, Stream};

use std::net::SocketAddr;
use std::io::BufReader;

use std::{thread, time, str};

use super::finger::FingerTable;
use super::finger;
use super::network_util;
use super::protocols::*;
use super::storage::Storage;
use super::util::*;
use super::chord;

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

    pub fn print(&self, desc: &str) {
        info!("{}: id: {}, ip_addr: {}", desc, self.id, self.ip_addr);
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
    internal_name: String,
    id: BigInt,
    ip_addr: SocketAddr,
    // finger_table: FingerTable, TODO we will care about this later
    // next_finger: usize,
    successor: OtherNode,
    predecessor: Option<OtherNode>,
    joined: bool,
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
    pub fn new(internal_name: String, node_ip_addr: SocketAddr, entry_node_addr: SocketAddr) -> Node {
        //let next_finger = 0; // Always start at first entry of finger_table
        //let finger_table = FingerTable::new(successor.clone(), &id);
        //let storage = Storage::new();
        Node {
            internal_name: internal_name,
            id: create_node_id(node_ip_addr),
            ip_addr: node_ip_addr,
            successor: OtherNode { id: create_node_id(entry_node_addr), ip_addr: entry_node_addr },
            predecessor: None,
            joined: false,
        }
    }

    pub fn new_first(internal_name: String, node_ip_addr: SocketAddr) -> Node {
        let id = create_node_id(node_ip_addr);
        Node {
            internal_name: internal_name,
            id: id.clone(),
            ip_addr: node_ip_addr.clone(),
            successor: OtherNode { id: id.clone(), ip_addr: node_ip_addr.clone() },
            predecessor: Some(OtherNode { id: id, ip_addr: node_ip_addr }),
            joined: true,
        }
    }

    pub fn join(&mut self) {
        info!("Starting joining process");
        let req = Request::FindSuccessor { id: self.id.clone() };
        self.send_message_to_socket(self.successor.ip_addr, req);
    }

    pub fn start_stabilisation(&mut self) {
        let mut node_clone = self.clone();
        let builder = thread::Builder::new().name(format!("{}-Stabilize", node_clone.internal_name).to_string());
        let handler = builder
            .spawn(move || {
                info!("Starting stabilisation");
                loop {
                    let req = Request::GetPredecessor;
                    node_clone.send_message_to_socket(node_clone.successor.ip_addr, req);
                    thread::sleep(chord::NODE_STABILIZE_INTERVAL);
                }
            })
            .unwrap();
    }

    /// Converts internal representation of node to the simpler representation OtherNode
    fn to_other_node(&self) -> OtherNode {
        OtherNode {
            id: self.id.clone(),
            ip_addr: self.ip_addr,
        }
    }

    fn process_incoming_request(&mut self, request: Request) -> Response {
        match request {
            Request::FindSuccessor { id } => {
                self.handle_find_successor_request(id)
            }
            Request::GetPredecessor => {
                self.handle_get_predecessor_request()
            }
            Request::Notify { node } => {
                self.handle_notify_request(node)
            }
        }
    }

    fn handle_find_successor_request(&self, id: BigInt) -> Response {
        if is_in_half_range(&id, &self.id, self.successor.get_id()) {
            Response::FoundSuccessor { successor: self.successor.clone() }
        } else {
            Response::AskFurther { next_node: self.successor.clone() }
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
                info!("[Node #{}] Predecessor is now: {}",self.id , node.id);
                self.predecessor = Some(node)
            }
            Some(pre) => {
                println!("[{:p} - {}] Current pre id: {}, possible new pre id: {}", self, self.id, pre.id, node.id);
                if pre.id != node.id && is_in_range(node.get_id(), pre.get_id(), &self.id) {
                    info!("[Node #{}] Predecessor is now: {}",self.id , node.id);
                    self.predecessor = Some(node);
                    println!("Predecessor: {}", self.predecessor.clone().unwrap().id);
                }
            }
        }
        //TODO check if maybe a failure notification is necessary
        Response::NotifyResponse
    }

    fn process_incoming_response(&mut self, response: Response) {
        match response {
            Response::FoundSuccessor { successor } => {
                self.handle_found_successor_response(successor)
            }
            Response::AskFurther { next_node } => {
                self.handle_ask_further_response(next_node)
            }
            Response::GetPredecessorResponse { predecessor } => {
                self.handle_get_predecessor_response(predecessor)
            }
            Response::NotifyResponse => {
                self.handle_notify_response()
            }
        }
    }

    fn handle_found_successor_response(&mut self, successor: OtherNode) {
        info!("Found my new successor: node #{}", successor.id.clone());
        self.successor = successor;
        if !self.joined {
            self.start_stabilisation();
            self.joined = true;
        }
    }

    fn handle_ask_further_response(&mut self, next_node: OtherNode) {
        info!("Did not get successor yet, asking node #{} now...", next_node.id);
        let req = Request::FindSuccessor { id: self.id.clone() };
        self.send_message_to_socket(next_node.ip_addr, req);
    }

    fn handle_get_predecessor_response(&mut self, predecessor: Option<OtherNode>) {
        if let Some(predecessor) = predecessor {
            if is_in_range(predecessor.get_id(), &self.id, self.successor.get_id()) {
                info!("Successor was node #{}, but got node #{} as predecessor of successor, so it is  successor now...", self.successor.id.clone(), predecessor.id.clone());
                self.successor = predecessor;
            }
        }
        let req = Request::Notify { node: self.to_other_node() };
        self.send_message_to_socket(self.successor.ip_addr, req);
    }

    fn handle_notify_response(&self) {}


//    /// Gets closet preceding finger
//    pub fn closet_finger_preceding(&self, find_id: &BigInt) -> OtherNode {
//        // n.closest_preceding_node(id)
//        //   for i = m downto 1
//        //     if (finger[i]∈(n,id))
//        //       return finger[i];
//        //   return n;
//        for x in self.finger_table.length()..0 {
//            let finger_entry = self.finger_table.get(x);
//            if let Some(node) = finger_entry.node.clone() {
//                if is_in_range(node.get_id(), &self.id, &find_id) {
//                    return node;
//                }
//            }
//        }
//
//        if is_in_range(&self.successor.id, &self.id, &find_id) {
//            self.successor.clone()
//        } else {
//            self.to_other_node()
//        }
//    }

//    /// Entry point after creation of node
//    /// Loops periodically to update fingertable
//    /// Calls fix_fingers
//    /// Notifies successor that I am his predecessor by sending NOTIFY_PREDECESSOR
//    pub fn start_update_fingers(&mut self) {
//        loop {
//            debug!("start_update_fingers()");
//            self.fix_fingers();
//            let message = Message::new(NOTIFY_PREDECESSOR, None, None);
//            self.send_msg(self.successor.clone(), None, message);
//
//            let millis2000 = time::Duration::from_millis(2000);
//            let now = time::Instant::now();
//            thread::sleep(millis2000);
//            assert!(now.elapsed() >= millis2000);
//        }
//    }

//    /// Periodically find successor for all entries of our fingertable
//    /// Sending self a message which subsequently sends messages to others
//    /// by dispatching FIND_SUCCESSOR message to other nodes
//    fn fix_fingers(&mut self) {
//        let fix_finger_id: BigInt;
//        let mut next = self.next_finger;
//        if next >= self.finger_table.length() {
//            next = 0;
//        }
//        fix_finger_id = finger::get_finger_id(&self.id, next);
//        self.next_finger = next + 1;
//        // n.fix_fingers()
//        let message = Message::new(FIND_SUCCESSOR, Some(next), Some(fix_finger_id));
//        self.send_msg(self.to_other_node(), None, message);
//    }
//
//    /// Notifies other peer about joining the network
//    /// TODO set remote to our successor
//    pub fn join(&mut self, remote: OtherNode) -> bool {
//        let message = Message::new(NOTIFY_JOIN, None, None);
//        self.predecessor = None;
//        remote.print("Try to join");
//        self.send_msg(remote, None, message);
//        true
//    }

//    pub fn send_msg(&mut self, mut label: OtherNode, to: Option<OtherNode>, mut msg: Message) {
//        // If no recipient is provided,
//        // the message is returned to the intial sender
//        // and labelled by `self` as `from`
//        let new_to = match to {
//            Some(to) => to,
//            None => {
//                let new_to = label.clone();
//                label = self.to_other_node();
//                new_to
//            }
//        };
//
//        // If the message id is undefined, it is set to `self``s ID
//        if msg.get_id().is_none() {
//            msg.set_id(Some(self.id.clone()))
//        }
//
//        let packet = Packet::new(label, msg);
//        let json_string = serde_json::to_string(&packet).unwrap();
//        // Send packet to recipient
//        self.send_message_to_socket(*new_to.get_ip_addr(), json_string);
//    }

    /* TODO fix
    fn handle_request(&mut self, stream: TcpStream, client_addr: SocketAddr) {
         let mut reader = BufReader::new(stream);

         loop {
             let mut buffer = String::new();
             match reader.read_line(&mut buffer) {
                 Ok(len) => {
                     // break when line is finished
                     if len == 0 {
                         break;
                     } else {
                         info!("New message from {}: {}", client_addr.to_string(), buffer);
                         let parsed_packet: Packet = serde_json::from_str(&buffer).unwrap();
                         let from = parsed_packet.get_from();
                         let message = parsed_packet.get_message();
                         self.process_incoming_msg(from.clone(), message.clone());
                     }
                 }
                 Err(e) => error!("Error reading message from {}: {}", client_addr, e),
             }
         }
     }*/

    // HINT: this can be tested by connecting via bash terminal (preinstalled on Mac/Linux) by executing:
    // nc 127.0.0.1 34254
    // afterwards every message will be echoed in the console by handle_request
    pub fn start_listening_on_socket(&mut self) -> Result<(), Box<std::error::Error>> {
        let mut node = self.clone();
        let listener = TcpListener::bind(&self.ip_addr).unwrap();

        //TODO figure out if extensive cloning is working
        info!("[Node #{}] Starting to listen on socket: {}", self.id, self.ip_addr);

        let server = listener.incoming().for_each(move |socket| {
            info!("[Node #{}] accepted socket; addr={:?}", node.id.clone(), socket.peer_addr()?);

            let buf = vec![];
            let buf_reader = BufReader::new(socket);
            let mut node_clone = node.clone();
            let connection = io::read_until(buf_reader, b'\n', buf)
                .and_then(move |(socket, buf)| {
                    let stream = socket.into_inner();

                    let msg_string = str::from_utf8(&buf).unwrap();

                    let message: RequestMessage = serde_json::from_str(msg_string).unwrap();
                    let request: Request = message.request;
                    info!("[Node #{}] Got request from Node #{}: {:?}", node_clone.id.clone(), message.sender.id, request.clone());
                    let response: Response = node_clone.process_incoming_request(request);
                    info!("[Node #{}] Sending response: {:?}", node_clone.id.clone(), response.clone());
                    let response_message = ResponseMessage { sender: node_clone.to_other_node(), response };
                    let response_string = format!("{}\n", serde_json::to_string(&response_message).unwrap());
                    io::write_all(stream, response_string)
                })
                .then(|_| Ok(())); // Just discard the socket and buffer

            // Spawn a new task that processes the socket:
            tokio::spawn(connection);

            Ok(())
        }).map_err(|e| println!("failed to accept socket; error = {:?}", e));
        tokio::run(server);
        Ok(())
    }

    pub fn send_message_to_socket(&mut self, addr: SocketAddr, request: Request) {
        let node = self.clone();
        let builder = thread::Builder::new().name(self.internal_name.clone().to_string());
        builder
            .spawn(move || {
                let mut node = node.clone();
                let request_message = RequestMessage { sender: node.to_other_node(), request };
                let request_message_string: String = format!("{}\n", serde_json::to_string(&request_message).unwrap());
                let client = TcpStream::connect(&addr).and_then(move |stream| {
                    io::write_all(stream, request_message_string).and_then(move |(stream, msg)| {
                        let sock = BufReader::new(stream);
                        io::read_until(sock, b'\n', vec![]).and_then(move |(stream, buf)| {
                            let response_string = str::from_utf8(&buf).unwrap();
                            let response_message: ResponseMessage = serde_json::from_str(response_string).unwrap();
                            let response: Response = response_message.response;
                            info!("[Node #{}] Got response from Node #{}: {:?}", node.id.clone(), response_message.sender.id, response.clone());
                            node.process_incoming_response(response);
                            Ok(())
                        })
                    })
                })
                    .map_err(|err| {
                        println!("connection error = {:?}", err);
                    });
                tokio::run(client);
                //Ok(())
            });
    }

//    pub fn process_incoming_msg(&mut self, from: OtherNode, msg: Message) {
//        match msg.get_message_type() {
//            NOTIFY_PREDECESSOR => self.update_predecessor(from, msg),
//            NOTIFY_SUCCESSOR => self.update_successor(from, msg),
//            NOTIFY_JOIN => self.notify_join(from, msg),
//            FIND_SUCCESSOR => self.find_successor(from, msg),
//            FOUND_SUCCESSOR => self.found_successor(from, msg),
//            MESSAGE => self.message(from, msg),
//            _ => {
//                warn!("Unknown chord message!");
//                msg.print();
//            }
//        }
//    }
//
//    /// A node `from` claims, that it is self's _new_ predecessor
//    /// ```rust
//    /// n.notify(n')
//    ///   if ( predecessor is nil or n' ∈ (predecessor, n) )
//    ///     predecessor = n';
//    /// ```
//    fn update_predecessor(&mut self, from: OtherNode, mut msg: Message) {
//        info!("MSG_TYPE_NOTIFY_PREDECESSOR = 0");
//
//        // Copy current self.predecessor value
//        let current_predecessor = self.predecessor.clone();
//        // Reassign self.predecessor
//        let new_predecessor = match current_predecessor {
//            // If `self.current_predecessor` is not empty verify
//            // if `from` is in range
//            // else keep `current_predecessor`
//            Some(self_predecessor) => {
//                if is_in_range(&from.id, &self_predecessor.id, &self.id) {
//                    from.print("Predecessor reassigned to");
//                    self.predecessor = Some(from.clone());
//                    from.clone()
//                } else {
//                    info!("Predecessor remains the same.");
//                    self_predecessor
//                }
//            }
//            // If `self.predecessor` is nil, assign `from` as new predecessor
//            None => {
//                from.print("Predecessor assigned to");
//                self.predecessor = Some(from.clone());
//                from.clone()
//            }
//        };
//        msg.set_message_type(NOTIFY_SUCCESSOR);
//        // TODO WHAT THE FUCK why is this msg labelled by the new_predecessor?
//        self.send_msg(new_predecessor, Some(from), msg);
//        self.finger_table.print()
//    }
//
//    /// ```rust
//    /// n.stabilize()
//    ///   x = successor.predecessor;
//    ///   if( x ∈ (n, successor) )
//    ///     successor = x;
//    ///   successor.notify(n);
//    /// ```
//    fn update_successor(&mut self, from: OtherNode, _msg: Message) {
//        info!("MSG_TYPE_NOTIFY_SUCCESSOR = 1");
//
//        // TODO maybe delete successor field in node struct
//        // TODO and instead use first finger entry in fingertable
//        // TODO but we have to verify if we don't produce any unforseen changes in the implementation
//        if is_in_range(&from.id, &self.id, &self.successor.id) {
//            self.successor = from;
//            self.successor.print("Successor reassigned");
//        }
//    }
//
//    fn notify_join(&mut self, from: OtherNode, _msg: Message) {
//        info!("MSG_TYPE_NOTIFY_JOIN = 2");
//        from.print("Node joined");
//    }
//
//    /// ```rust
//    /// n.find_successor(id)
//    ///  if ( id ∈ (n, successor] )
//    ///    return successor;
//    ///  else
//    ///    return successor.find_successor(id);
//    /// ```
//    fn find_successor(&mut self, from: OtherNode, mut msg: Message) {
//        info!("MSG_TYPE_FIND_SUCCESSOR = 3");
//
//        if let Some(msg_id) = msg.get_id() {
//            if is_in_half_range(&msg_id, &self.id, &self.successor.id) {
//                self.successor.print("FIND_SUCCESSOR");
//                msg.set_message_type(FOUND_SUCCESSOR);
//                self.send_msg(self.successor.clone(), Some(from), msg);
//            } else {
//                // Fix fingertable and forward the query
//                let node_0 = self.closet_finger_preceding(&msg_id);
//                node_0.print("FIND_SUCCESSOR = closet_finger_preceding");
//                msg.set_message_type(FOUND_SUCCESSOR);
//                self.send_msg(node_0, Some(from), msg);
//            }
//        };
//    }
//
//    /// ```rust
//    /// n.fix_fingers()
//    ///   for i = 1 to m
//    ///     finger[i].Knoten = find_successor(finger[i].Start);
//    /// ```
//    fn found_successor(&mut self, from: OtherNode, msg: Message) {
//        info!("MSG_TYPE_FOUND_SUCCESSOR = 4");
//
//        if let Some(next_finger_index) = msg.get_next_finger() {
//            self.finger_table.put(next_finger_index, from);
//            info!("FingerTable fixed.");
//            self.finger_table.print();
//        } else {
//            self.successor = from;
//            self.successor.print("New successor is now");
//        }
//    }
//
//    fn message(&mut self, from: OtherNode, msg: Message) {
//        info!("MSG_TYPE_MESSAGE = 5");
//
//        self.send_msg(self.successor.clone(), Some(from), msg);
//    }
}
