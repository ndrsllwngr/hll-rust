use std::str::Bytes;

use super::node::Node;


// Represents a single finger table entry
pub struct FingerEntry<'a> {
    pub id: Bytes<'a>, // ID hash of (n + 2^i) mod (2^m)
    pub node: &'a Node<'a>
}

pub type FingerTable<'a> = Vec<FingerEntry<'a>>;

fn new_finger_entry<'a>(id: Bytes<'a>, node: &'a Node) -> FingerEntry<'a> {
    return FingerEntry{id, node};
}

// TODO
fn finger_id<'a>(n: &Bytes, _i: i64, _m: i64) -> Bytes<'a>  {
    let test: Bytes<'a> = "finger_entry_id".bytes();
    return test
}

// m = size of finger table
pub fn new_finger_table<'a>(node: &'a Node, m: i64) -> FingerTable<'a>  {
    let mut ft: Vec<FingerEntry> = Vec::new();
    for i in 0..m {
        ft.push(new_finger_entry(finger_id(&node.config.id, i, m), node))
    }
    return ft;
}