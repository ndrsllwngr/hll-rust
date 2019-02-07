use std::io::stdin;
use std::net::SocketAddr;
use std::process;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use num::bigint::{BigInt, Sign, ToBigInt};

use super::chord;
use super::fingertable::*;
use super::input::*;
use super::network;
use super::node::*;
use super::print;
use super::protocols::*;

/// Size of the hash digest which gets generated by the hashs functions.
/// `m` (e.g.) 20 is the digest size of sha1
// pub const HASH_DIGEST_LENGTH: usize = 20;

//TODO discuss best size (fingertable and succ_list should depend on this)
pub const CHORD_CIRCLE_BITS: usize = 32;

//Used for length reduction on id creation
//The nth root of the initially created id will be calculated in order to reduce size

pub const FINGERTABLE_SIZE: usize = 16;

pub const SUCCESSORLIST_SIZE: usize = 16;

/// At most a number of `2^m` nodes are allowed in the Chord Circle (Bit Shift left)
pub const CHORD_RING_SIZE: usize = 1 << CHORD_CIRCLE_BITS;

pub const NODE_STABILIZE_INTERVAL: time::Duration = time::Duration::from_millis(4000);

pub const NODE_FIX_FINGERS_INTERVAL: time::Duration = time::Duration::from_millis(1000);

pub const NODE_CHECK_PREDECESSOR_INTERVAL: time::Duration = time::Duration::from_millis(4000);

pub const NODE_INIT_SLEEP_INTERVAL: time::Duration = time::Duration::from_millis(2000);

pub const NODE_PRINT_INTERVAL: time::Duration = time::Duration::from_millis(2000);

pub const LISTENING_ADDRESS: &str = "0.0.0.0";

pub fn join(id: BigInt, sender: OtherNode, join_ip: SocketAddr) {
    info!("Starting joining process");
    let req = Request::FindSuccessor { id };
    let msg = Message::RequestMessage { sender, request: req };
    network::send_string_to_socket(join_ip, serde_json::to_string(&msg).unwrap());
    //self.send_message_to_socket(self.successor.ip_addr, req);
}

//TODO pass internal name & othernode as parameters
pub fn stabilize(arc: Arc<Mutex<Node>>) {
    info!("Starting stabilisation...");
    loop {
        debug!("Stabilize.............");
        // make a copy of node and instantly drop it
        let node = arc.lock().unwrap();
        let node_clone = node.clone();
        drop(node);

        if node_clone.is_joined() {
            //print_current_node_state(&node_clone);

            let mut ring_is_alive = false;
            for succ in node_clone.get_successor_list().clone() {
                if network::check_alive(*succ.get_ip_addr(), node_clone.to_other_node().clone()) {
                    let req = Request::GetPredecessor;
                    let msg = Message::RequestMessage { sender: node_clone.to_other_node().clone(), request: req };
                    network::send_string_to_socket(*succ.get_ip_addr(), serde_json::to_string(&msg).unwrap());

                    // after async operation check alive lock again.
                    arc.lock().unwrap().update_successor_and_successor_list(succ);

                    ring_is_alive = true;
                    break;
                } else {
                    error!("Node is dead: {:?}", succ);
                }
            }
            if !ring_is_alive {
                error!("No functional successor found in successor list. RING IS DEAD. Initializing shutdown...");
                process::exit(1);
            }
        } else {
            info!("Not joined jet going to sleep again")
        }
        //this is super important, because otherwise the lock would persist endlessly due to the loop
        //node_clone.send_message_to_socket(node_clone.successor.ip_addr, req);
        thread::sleep(chord::NODE_STABILIZE_INTERVAL);
    }
}

pub fn fix_fingers(arc: Arc<Mutex<Node>>) {
    debug!("Starting fix_fingers...");
    let mut next = 1;
    loop {
        let node = arc.lock().unwrap();
        if node.is_joined() {
            let finger_id = get_finger_id(node.get_id(), next);

            let req = Request::FindSuccessorFinger { index: next, finger_id };
            let msg = Message::RequestMessage { sender: node.to_other_node(), request: req };
            network::send_string_to_socket(*node.get_successor().get_ip_addr(), serde_json::to_string(&msg).unwrap());

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
    debug!("Starting check_predecessor...");
    loop {
        // make a copy of node and instantly drop it
        let node = arc.lock().unwrap();
        let node_clone = node.clone();
        drop(node);

        if node_clone.is_joined() {
            if let Some(predecessor) = node_clone.get_predecessor().clone() {
                if !network::check_alive(*predecessor.get_ip_addr(), node_clone.to_other_node().clone()) {
                    debug!("Node #{} is dead", predecessor.get_id());

                    // after async operation check alive lock again.
                    arc.lock().unwrap().set_predecessor(None);
                } else {
                    debug!("Node #{} is alive", predecessor.get_id());
                }
            }
        } else { info!("Not joined jet going to sleep again") }
        //this is super important, because otherwise the lock would persist endlessly due to the loop
        //node_clone.send_message_to_socket(node_clone.successor.ip_addr, req);
        thread::sleep(chord::NODE_CHECK_PREDECESSOR_INTERVAL);
    }
}

pub fn print_and_interact(arc: Arc<Mutex<Node>>) {
    let interaction_in_progress = Arc::new(AtomicBool::new(false));
    let i_clone = interaction_in_progress.clone();

    let node = arc.lock().unwrap();
    let other_node = node.to_other_node().clone();
    drop(node);

    let _handle = thread::Builder::new().name("Interaction".to_string()).spawn(move || {
        loop {
            let buffer = &mut String::new();
            stdin().read_line(buffer).unwrap();
            if let "m" = buffer.trim_right() {
                i_clone.store(true, Ordering::SeqCst);
                perform_user_interaction(other_node.clone()).expect("perform_user_interaction failed");
                i_clone.store(false, Ordering::SeqCst);
            };
        }
    }).unwrap();

    loop {
        let node = arc.lock().unwrap();
        let node_clone = node.clone();
        drop(node);
        if node_clone.is_joined() && !interaction_in_progress.load(Ordering::SeqCst) {
            print::print_current_node_state(&node_clone)
        }
        thread::sleep(chord::NODE_PRINT_INTERVAL);
    }
}

pub fn create_node_id(ip_addr: SocketAddr) -> BigInt {
    let hash = create_hash(&ip_addr.to_string());
    let byte_vec = hash.as_bytes().to_vec();
    let id = BigInt::from_bytes_be(Sign::Plus, &byte_vec);
    x_modulo_ring_size(id)
}

pub fn create_id(string: &str) -> BigInt {
    let hash = create_hash(string);
    let byte_vec = hash.as_bytes().to_vec();
    let id = BigInt::from_bytes_be(Sign::Plus, &byte_vec);
    x_modulo_ring_size(id)
}

/**
 * Test if id ∈ (first, second)
 */
pub fn is_in_interval(first: &BigInt, second: &BigInt, id: &BigInt) -> bool {
    if first == second {
        true
    } else {
        x_modulo_ring_size(id - first) < x_modulo_ring_size(second - first)
    }
}

pub fn chord_abs(a: &BigInt, b: &BigInt) -> BigInt {
    if b > a {
        chord::CHORD_RING_SIZE - b + a
    } else {
        a - b
    }
}

pub fn is_my_key(self_id: &BigInt, pre_id: &BigInt, key_id: &BigInt) -> bool {
    self_id == key_id || (key_id != pre_id && is_in_interval(pre_id, self_id, key_id))
}

fn create_hash(string: &str) -> String {
    // create a Sha1 object
    let mut hasher = Sha1::new();

    // write input message
    hasher.input_str(string);

    // read hash digest
    hasher.result_str()
}

fn x_modulo_ring_size(x: BigInt) -> BigInt {
    let one: &BigInt = &1.to_bigint().unwrap();
    let chord_ring_size: &BigInt = &chord::CHORD_RING_SIZE.to_bigint().unwrap();
    BigInt::modpow(&x, one, chord_ring_size)
}

pub fn spawn_node(node_ip_addr: SocketAddr, port: i32, entry_node_addr: Option<SocketAddr>) -> JoinHandle<()> {
    if entry_node_addr.is_some() {
        info!("Spawn node and join.");
    } else {
        info!("Spawn master node.");
    }
    let builder = thread::Builder::new().name("Node".to_string());
    builder
        .spawn(move || {
            let node = if entry_node_addr.is_some() {
                Node::new(node_ip_addr)
            } else {
                Node::new_first(node_ip_addr)
            };
            // let mut node = node::Node::new(node_ip_addr.clone());
            let id = node.get_id().clone();
            let id_clone = id.clone();

            let other_node = node.to_other_node();

            let arc = Arc::new(Mutex::new(node));

            let arc_clone1 = arc.clone();
            let handle1 = thread::Builder::new().name("Listen".to_string())
                .spawn(move || {
                    network::start_listening_on_socket(arc_clone1, port, id_clone).expect("network_util::start_listening_on_socket failed");
                }).unwrap();

            if let Some(entry_node_addr) = entry_node_addr {
                thread::sleep(chord::NODE_INIT_SLEEP_INTERVAL);
                chord::join(id.clone(), other_node.clone(), entry_node_addr);
            }

            let arc_clone2 = arc.clone();
            let handle2 = thread::Builder::new().name("Stabilize".to_string())
                .spawn(move || {
                    chord::stabilize(arc_clone2);
                }).unwrap();

            let arc_clone3 = arc.clone();
            let handle3 = thread::Builder::new().name("Fix_Fingers".to_string())
                .spawn(move || {
                    chord::fix_fingers(arc_clone3);
                }).unwrap();

            let arc_clone4 = arc.clone();
            let handle4 = thread::Builder::new().name("Check_Predecessor".to_string())
                .spawn(move || {
                    chord::check_predecessor(arc_clone4);
                }).unwrap();

            let arc_clone5 = arc.clone();
            let handle5 = thread::Builder::new().name("Print_Interact".to_string())
                .spawn(move || {
                    chord::print_and_interact(arc_clone5);//.expect("print_and_interact failed");
                }).unwrap();

            handle1.join().expect("handle1 failed");
            handle2.join().expect("handle2 failed");
            handle3.join().expect("handle3 failed");
            handle4.join().expect("handle4 failed");
            handle5.join().expect("handle5 failed");
        })
        .unwrap()
}
