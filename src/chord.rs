use std::time;

/// Size of the hash digest which gets generated by the hashs functions.
/// `m` (e.g.) 20 is the digest size of sha1
// pub const HASH_DIGEST_LENGTH: usize = 20;

pub const CHORD_CIRCLE_BITS: usize = 16;

//Used for length reduction on id creation
//The nth root of the initially created id will be calculated in order to reduce size
pub const ID_ROOT: u32 = 12;

pub const FINGERTABLE_SIZE: usize = CHORD_CIRCLE_BITS;

pub const SUCCESSORLIST_SIZE: usize = CHORD_CIRCLE_BITS;

/// At most a number of `2^m` nodes are allowed in the Chord Circle (Bit Shift left)
pub const CHORD_RING_SIZE: usize = 1 << CHORD_CIRCLE_BITS;

pub const NODE_STABILIZE_INTERVAL: time::Duration = time::Duration::from_millis(5000);

pub const NODE_FIX_FINGERS_INTERVAL: time::Duration = time::Duration::from_millis(500);

pub const NODE_CHECK_PREDECESSOR_INTERVAL: time::Duration = time::Duration::from_millis(5000);

pub const NODE_INIT_SLEEP_INTERVAL: time::Duration = time::Duration::from_millis(2000);

pub const NODE_PRINT_INTERVAL: time::Duration = time::Duration::from_millis(2000);

pub const LISTENING_ADDRESS: &str = "0.0.0.0";