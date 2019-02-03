use num::bigint::{BigInt, ToBigInt};
use num::traits::pow;

use super::chord;
use super::node::OtherNode;

// Represents a single finger table entry
#[derive(Clone)]
pub struct FingerEntry {
    pub id: BigInt,
    // ID hash of (n + 2^i) mod (2^m)
    pub node: OtherNode,
}

#[derive(Clone)]
pub struct FingerTable {
    parent_node_id: BigInt,
    pub entries: Vec<FingerEntry>,
}

impl FingerTable {
    pub fn new(parent_node_id: BigInt) -> FingerTable {
        FingerTable { parent_node_id, entries: Vec::with_capacity(chord::FINGERTABLE_SIZE) }
    }
    // TODO maybe use hashing function with smaller bit range for testing. (Bitsize = entries in finger_table)
    pub fn new_first(parent_node_id: BigInt, successor: OtherNode) -> FingerTable {
        let mut entries: Vec<FingerEntry> = Vec::with_capacity(chord::FINGERTABLE_SIZE);
        entries.push(FingerEntry {
            id: get_finger_id(&parent_node_id, 0),
            node: successor,
        });
        FingerTable { parent_node_id, entries }
    }

    pub fn put(&mut self, index: usize, finger_id: BigInt, node: OtherNode) {
        let finger_entry = FingerEntry {id: finger_id, node};
        if self.entries.len() > index {
            self.entries[index] = finger_entry;
        } else {
            self.entries.push(finger_entry);
        }
    }

    pub fn get_successor(&self) -> OtherNode {
        self.entries[0].node.clone()
    }

    pub fn set_successor(&mut self, successor: OtherNode) {
        if self.entries.is_empty() {
            self.entries.push(FingerEntry{id: get_finger_id(&self.parent_node_id, 0), node: successor});
        } else {
            self.entries[0].node = successor;
        }
    }

    pub fn get(&self, index: usize) -> &FingerEntry {
        &self.entries[index]
    }

    pub fn length(&self) -> usize {
        self.entries.len()
    }

}

pub fn get_finger_id(n: &BigInt, exponent: usize) -> BigInt {
    // Get the offset
    let two: BigInt = 2.to_bigint().unwrap();
    let offset: BigInt = pow(two.clone(), exponent);

    // Sum
    n + offset
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
