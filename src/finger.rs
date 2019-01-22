use num::bigint::{BigInt, Sign, ToBigInt};
use num::traits::pow;
use std::net::SocketAddr;

use super::chord::FINGERTABLE_SIZE;
use super::node::OtherNode;
use super::util::create_node_id;

// Represents a single finger table entry
#[derive(Clone)]
pub struct FingerEntry {
    pub id: BigInt, // ID hash of (n + 2^i) mod (2^m)
    pub node: OtherNode,
}

#[derive(Clone)]
pub struct FingerTable {
    entries: Vec<FingerEntry>,
}

impl FingerTable {
    pub fn new() -> FingerTable {
        // TODO HAHA SO GEHT DAS NICHT, das ist ja ultra hacky, müssen uns überlegen wie das besser geht gn8
        let id = create_node_id("127.0.0.1:22222".parse::<SocketAddr>().unwrap());
        let node = OtherNode::new(
            0.to_bigint().unwrap(),
            "127.0.0.1:22222".parse::<SocketAddr>().unwrap(),
        );
        FingerTable {
            entries: vec![FingerEntry { id, node }; FINGERTABLE_SIZE],
        }
    }

    pub fn put(&mut self, index: usize, id: BigInt, node: OtherNode) {
        let entry = FingerEntry { id, node };
        self.entries[index] = entry;
    }

    pub fn get(&self, index: usize) -> Option<&FingerEntry> {
        if self.length() < index {
            Some(&self.entries[index])
        } else {
            None
        }
    }

    pub fn length(&self) -> usize {
        self.entries.len()
    }

    pub fn print(&self) {
        // info!("{0: <2} | {1: <97} | {2: <16}", "i", "id", "node");
        let mut finger_table_string: String = "\n".to_string();
        for i in 0..self.entries.len() {
            let borrowed_str: &str = &format!(
                "{0: <2} | {1: <97} | {2: <16}\n",
                i,
                self.entries[i].id,
                self.entries[i].node.get_ip_addr()
            );

            finger_table_string.push_str(borrowed_str);
        }
        info!("{}", finger_table_string);
    }
}

// TODO use this fn to generate indices of fingertable?
fn finger_id(n: &[u8], i: usize, m: usize) -> Vec<u8> {
    let id_int = BigInt::from_bytes_be(Sign::NoSign, n);

    // Get the offset
    let two: BigInt = 2.to_bigint().unwrap();
    let offset: BigInt = pow(two.clone(), i);

    // Sum
    let sum = id_int + offset;

    // Get the ceiling
    let ceil: BigInt = pow(two.clone(), m);

    // Modulo
    let modulo = BigInt::modpow(&sum, &1.to_bigint().unwrap(), &ceil);

    modulo.to_bytes_be().1
}

/*
// m = size of finger table
pub fn new_finger_table<'a>(node: &'a OtherNode, m: usize) -> FingerTable<'a>  {
    let mut ft: Vec<FingerEntry> = Vec::new();
    for i in 0..m {
        let id = finger_id(&node.config.id, i as usize, m).clone();
        ft.push(new_finger_entry(id , node));
    }
    return ft;
}
*/
