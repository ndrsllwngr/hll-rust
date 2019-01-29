use std::net::SocketAddr;
use std::thread::JoinHandle;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use num_bigint::BigInt;

use super::network_util;
use super::protocols::*;
use super::chord;
use super::node::*;


pub fn join(id: BigInt, sender: OtherNode, join_ip: SocketAddr) {
    info!("Starting joining process");
    let req = Request::FindSuccessor { id };
    let msg = Message::RequestMessage { sender: sender, request: req };
    network_util::send_string_to_socket(join_ip, serde_json::to_string(&msg).unwrap());
    //self.send_message_to_socket(self.successor.ip_addr, req);
}

//TODO pass internal name & othernode as parameters
pub fn stabilize(arc: Arc<Mutex<Node>>) {
    info!("Starting stabilisation...");
    loop {
        info!("Stabilize.............");
        let node = arc.try_lock().unwrap();

        if node.joined {
            let req = Request::GetPredecessor;
            let msg = Message::RequestMessage { sender: node.to_other_node(), request: req };
            network_util::send_string_to_socket(node.successor.get_ip_addr().clone(), serde_json::to_string(&msg).unwrap());
        } else { info!("Not joined jet going to sleep again") }

        //this is super important, because otherwise the lock would persist endlessly due to the loop
        drop(node);
        //node_clone.send_message_to_socket(node_clone.successor.ip_addr, req);
        thread::sleep(chord::NODE_STABILIZE_INTERVAL);
    }
}
