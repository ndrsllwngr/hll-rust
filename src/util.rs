use crypto::digest::Digest;
use crypto::sha1::Sha1;
use num_bigint::BigInt;


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
pub fn isInHalfRange(key: BigInt, n: BigInt, successor: BigInt) -> Bool {
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
pub fn isInRange(key: BigInt, left: BigInt, right: BigInt) -> Bool {
    if left < right {
        return (key > left && key < right) || (left == right && key != left);
    } else {
        return (key > right && key < left) || (left == right && key != left);
    }
}