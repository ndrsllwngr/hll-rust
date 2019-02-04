use crypto::digest::Digest;
use crypto::sha1::Sha1;
use num::bigint::{BigInt, Sign, ToBigInt};
use std::net::SocketAddr;

use super::chord;

pub fn create_node_id(ip_addr: SocketAddr) -> BigInt {
    let hash = create_hash(&ip_addr.to_string());
    let byte_vec = hash.as_bytes().to_vec();
    let id = BigInt::from_bytes_be(Sign::Plus, &byte_vec);
    id.nth_root(chord::ID_ROOT)
}

pub fn create_id(string: &str) -> BigInt {
    let hash = create_hash(string);
    let byte_vec = hash.as_bytes().to_vec();
    let id = BigInt::from_bytes_be(Sign::Plus, &byte_vec);
    id.nth_root(chord::ID_ROOT)
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
    if b < a {
        chord::CHORD_RING_SIZE - a + b
    } else {
        b - a
    }
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
