use num::bigint::{BigInt, Sign, ToBigInt};
use num::traits::pow;
use std::net::SocketAddr;

use super::chord::FINGERTABLE_SIZE;
use super::node::OtherNode;
use super::util;

// Represents a single finger table entry
#[derive(Clone)]
pub struct FingerEntry {
    pub id: BigInt,
    // ID hash of (n + 2^i) mod (2^m)
    pub node: OtherNode,
}

#[derive(Clone)]
pub struct FingerTable {
    entries: Vec<FingerEntry>,
}

impl FingerTable {
    pub fn new(successor: OtherNode) -> FingerTable {
        let mut entries: Vec<FingerEntry> = Vec::new();
        entries.push(
            FingerEntry{
                // TODO maybe use hashing function with smaller bit range for testing. (Bitsize = entries in finger_table)
                id: get_finger_id(successor.get_id(), 0_usize),
                node: successor
            });
        FingerTable{entries}
    }

    pub fn put(&mut self, index: usize, id: BigInt, node: OtherNode) {
        let entry = FingerEntry { id, node };
        if index >= self.length() {
            self.entries.push(entry)
        } else {
            self.entries[index] = entry;

        }
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
        let mut finger_table_string: String =
            format!("\n{0: <2} | {1: <97} | {2: <20}\n\
        ----------------------------------------------------------------------------------------------------------------------------\n",
                    "#", "Start: node_id + 2^i", "Node_IP");
        for i in 0..self.entries.len() {
            let borrowed_str: &str = &format!(
                "{0: <2} | {1: <97} | {2: <20}\n",
                i,
                self.entries[i].id,
                self.entries[i].node.get_ip_addr()
            );

            finger_table_string.push_str(borrowed_str);
        }
        info!("{}", finger_table_string);
    }
}


pub fn get_finger_id(key: &BigInt, exponent: usize) -> BigInt {
    // Get the offset
    let two: BigInt = 2.to_bigint().unwrap();
    let offset: BigInt = pow(two.clone(), exponent);

    // Sum
    key + offset
}

// Not in use right now TODO use this fn instead of get_finger_id to generate indices of fingertable?
fn get_finger_id_with_modulo(n: &[u8], i: usize, m: usize) -> Vec<u8> {
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
