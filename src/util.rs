use crypto::digest::Digest;
use crypto::sha1::Sha1;
use num::bigint::{BigInt, Sign, ToBigInt};
use num::traits::pow;
use std::net::SocketAddr;

use super::chord;

//TODO discuss if this is better placed here or in node.rs
//TODO write test for this function to verify correctness
pub fn create_node_id(ip_addr: SocketAddr) -> BigInt {
    let hash = create_hash(&ip_addr.to_string());
    let byte_vec = hash.as_bytes().to_vec();
    BigInt::modpow(&BigInt::from_bytes_be(Sign::Plus, &byte_vec),
                   &1.to_bigint().unwrap(),
                   &chord::CHORD_RING_SIZE.to_bigint().unwrap())
}

// TODO write test for this function to verify correctness
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
    if b < a {
        chord::CHORD_RING_SIZE - a + b
    } else {
        b - a
    }
}

//TODO write test for this function to verify correctness
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

// WE MOST LIKELY DONT NEED THOSE ANYMORE
//**
//* Testing if key ∈ (left, right)
//*/
////TODO check if left & right naming is reasonable
////TODO write test for this function to verify correctness
//pub fn is_in_range(key: &BigInt, left: &BigInt, right: &BigInt) -> bool {
//    if left < right {
//        (key > left && key < right) || (left == right && key != left)
//    } else {
//        (key > right && key < left) || (left == right && key != left)
//    }
//}
//
//**
//* Testing if id ∈ (n, successor]
//*/
////TODO write test for this function to verify correctness
//pub fn is_in_half_range(id: &BigInt, n: &BigInt, succ: &BigInt) -> bool {
//    if n < successor {
//        // In most cases this will match
//        (id > n && id <= successor)
//    } else if n > successor {
//        // But the circle is a ring! So at some point n will be bigger as its Successor
//        // Example: Chord ring with maximum 16 nodes. Successor(N16) = N1
//        (id > successor && id <= n)
//    } else {
//        // This is the case for a new Chord ring with just the master node in, so return true
//    }
//    true
//}
