use crypto::digest::Digest;
use crypto::sha1::Sha1;
use num_bigint::{BigInt, Sign};
use std::net::SocketAddr;


//TODO discuss if this is better placed here or in node.rs
pub fn create_node_id(ip: SocketAddr) -> BigInt {
    let hash = create_hash(&ip.to_string());
    let byte_vec = hash.as_bytes().to_vec();
    return BigInt::from_bytes_be(Sign::Plus, &byte_vec);
}

pub fn create_hash(string: &str) -> String {

// create a Sha1 object
    let mut hasher = Sha1::new();

// write input message
    hasher.input_str(string);

// read hash digest
    let hex = hasher.result_str();
    return hex;
}

/**
* Testing if key ∈ (n, successor]
*/
pub fn is_in_half_range(key: BigInt, n: BigInt, successor: BigInt) -> bool {
    if n < successor {
        return (key > n && key <= successor) || (n == successor);
    } else {
        return (key > successor && key <= n) || (n == successor);
    }
}

/**
* Testing if key ∈ (left, right)
*/
//TODO check if left & right naming is reasonable
pub fn is_in_range(key: BigInt, left: BigInt, right: BigInt) -> bool {
    if left < right {
        return (key > left && key < right) || (left == right && key != left);
    } else {
        return (key > right && key < left) || (left == right && key != left);
    }
}