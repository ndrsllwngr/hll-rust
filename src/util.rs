use super::crypto::digest::Digest;
use super::crypto::sha1::Sha1;

pub fn create_hash(string: &str) -> String {


// create a Sha1 object
    let mut hasher = Sha1::new();

// write input message
    hasher.input_str(string);

// read hash digest
    let hex = hasher.result_str();
    return hex;
}