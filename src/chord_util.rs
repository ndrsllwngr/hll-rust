use std::net::SocketAddr;
use std::thread::JoinHandle;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use num_bigint::BigInt;

use super::network_util;
use super::protocols::*;
use super::chord;
use super::node::*;
use super::finger::*;


pub fn join(id: BigInt, sender: OtherNode, join_ip: SocketAddr) {
    info!("Starting joining process");
    let req = Request::FindSuccessor { id: id };
    let msg = Message::RequestMessage { sender: sender, request: req };
    network_util::send_string_to_socket(join_ip, serde_json::to_string(&msg).unwrap());
    //self.send_message_to_socket(self.successor.ip_addr, req);
}

//TODO pass internal name & othernode as parameters
pub fn stabilize(arc: Arc<Mutex<Node>>) {
    info!("Starting stabilisation...");
    loop {
        info!("Stabilize.............");
        let node = arc.lock().unwrap();

        if node.joined {
            let req = Request::GetPredecessor;
            let msg = Message::RequestMessage { sender: node.to_other_node(), request: req };
            network_util::send_string_to_socket(node.get_successor().get_ip_addr().clone(), serde_json::to_string(&msg).unwrap());
        } else { info!("Not joined jet going to sleep again") }
        node.print_current_state();
        //this is super important, because otherwise the lock would persist endlessly due to the loop
        drop(node);
        //node_clone.send_message_to_socket(node_clone.successor.ip_addr, req);
        thread::sleep(chord::NODE_STABILIZE_INTERVAL);
    }
}

pub fn fix_fingers(arc: Arc<Mutex<Node>>) {
    info!("Starting fix_fingers...");
    let mut next = 1;
    loop {
        let node = arc.lock().unwrap();
        if node.joined {
            let finger_id = get_finger_id(&node.id, next);

            let req = Request::FindSuccessorFinger { index: next, finger_id: finger_id };
            let msg = Message::RequestMessage { sender: node.to_other_node(), request: req };
            network_util::send_string_to_socket(node.get_successor().get_ip_addr().clone(), serde_json::to_string(&msg).unwrap());

            next = if next < chord::FINGERTABLE_SIZE - 1 {
                next + 1
            } else {
                1
            };

        } else { info!("Not joined jet going to sleep again") }
        //this is super important, because otherwise the lock would persist endlessly due to the loop
        drop(node);
        //node_clone.send_message_to_socket(node_clone.successor.ip_addr, req);
        thread::sleep(chord::NODE_FIX_FINGERS_INTERVAL);
    }
}

pub fn check_predecessor(arc: Arc<Mutex<Node>>) {
    info!("Starting check_predecessor...");
    loop {
        let mut node = arc.lock().unwrap();
        if node.joined {
            if let Some(predecessor) = node.predecessor.clone() {
                if !network_util::check_alive(predecessor.get_ip_addr().clone(), node.to_other_node()) {
                    node.predecessor = None;
                    info!("Node #{} is dead", predecessor.get_id());
                } else {
                    info!("Node #{} is alive", predecessor.get_id());
                }
            }
        } else { info!("Not joined jet going to sleep again") }
        //this is super important, because otherwise the lock would persist endlessly due to the loop
        drop(node);
        //node_clone.send_message_to_socket(node_clone.successor.ip_addr, req);
        thread::sleep(chord::NODE_CHECK_PREDECESSOR_INTERVAL);
    }

}