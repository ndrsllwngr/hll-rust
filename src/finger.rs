use num::bigint::{BigInt, Sign, ToBigInt};
use num::traits::{pow};

use super::node::Node;
use super::util::create_hash;


// Represents a single finger table entry
pub struct FingerEntry<'a> {
    pub id: Vec<u8>, // ID hash of (n + 2^i) mod (2^m)
    pub node: &'a Node<'a>
}



fn new_finger_entry<'a>(id: Vec<u8>, node: &'a Node) -> FingerEntry<'a> {
    return FingerEntry{id, node};
}


pub type FingerTable<'a> = Vec<FingerEntry<'a>>;



fn finger_id<'a>(n: &Vec<u8>, i: usize, m: usize) -> Vec<u8>  {

    let id_int  = BigInt::from_bytes_be(Sign::NoSign, n);

    // Get the offset
    let test = vec![2];
    let two : BigInt= 2.to_bigint().unwrap();
    let offset : BigInt = pow(two.clone(), i);

    // Sum
    let sum = id_int + offset;

    // Get the ceiling
    let ceil : BigInt = pow(two.clone(), m);

    // Modulo
    let modulo = BigInt::modpow(&sum,&1.to_bigint().unwrap(), &ceil);

    return modulo.to_bytes_be().1;
}

// m = size of finger table
pub fn new_finger_table<'a>(node: &'a Node, m: usize) -> FingerTable<'a>  {
    let mut ft: Vec<FingerEntry> = Vec::new();
    for i in 0..m {
        let id = finger_id(&node.config.id, i as usize, m).clone();
        ft.push(new_finger_entry(id , node));
    }
    return ft;
}