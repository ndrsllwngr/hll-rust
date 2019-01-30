use std::time;

/// Size of the hash digest which gets generated by the hashs functions.
/// `m` (e.g.) 20 is the digest size of sha1
pub const HASH_DIGEST_LENGTH: usize = 20;

pub const CHORD_CIRCLE_BITS: usize = 16;

pub const FINGERTABLE_SIZE: usize = CHORD_CIRCLE_BITS;

pub const SUCCESSORLIST_SIZE: usize = CHORD_CIRCLE_BITS;

/// At most a number of `2^m` nodes are allowed in the Chord Circle (Bit Shift left)
pub const CHORD_RING_SIZE: usize = 1 << CHORD_CIRCLE_BITS;

pub const NODE_STABILIZE_INTERVAL: time::Duration = time::Duration::from_millis(2000);

pub const NODE_FIX_FINGERS_INTERVAL: time::Duration = time::Duration::from_millis(200);

pub const NODE_CHECK_PREDECESSOR_INTERVAL: time::Duration = time::Duration::from_millis(2000);

pub const NODE_INIT_SLEEP_INTERVAL: time::Duration = time::Duration::from_millis(2000);

// pub const CHORD_CHANGE_INTERVALL: usize = 5;

// /// Error return type for failed requests
// pub const MSG_TYPE_CHORD_ERR: i8 = -1;

// pub const MSG_TYPE_NULL: i8 = 0;

// /// Find successor for given ID
// pub const MSG_TYPE_FIND_SUCCESSOR: i8 = 1;

// /// Response to `MSG_TYPE_FIND_SUCCESSOR`
// pub const MSG_TYPE_FIND_SUCCESSOR_RESPONSE: i8 = 2;

// /// Next noode we need to ask
// pub const MSG_TYPE_FIND_SUCCESSOR_RESP_NEXT: i8 = 3;

// /// Get predrecessor of the target node
// pub const MSG_TYPE_GET_PREDECESSOR: i8 = 4;

// /// Response to `MSG_TYPE_GET_PREDECESSOR`
// pub const MSG_TYPE_GET_PREDECESSOR_RESP: i8 = 5;

// /// Response `Nil` to MSG_TYPE_GET_PREDECESSOR
// pub const MSG_TYPE_GET_PREDECESSOR_RESP_NIL: i8 = 6;

// /// Find successor for given ID
// pub const MSG_TYPE_GET_SUCCESSOR: i8 = 7;

// /// Response to `MSG_TYPE_GET_SUCCESSOR`
// pub const MSG_TYPE_GET_SUCCESSOR_RESP: i8 = 8;

// /// Check if node is alive
// pub const MSG_TYPE_PING: i8 = 9;

// /// Response to `MSG_TYPE_PING`
// pub const MSG_TYPE_PONG: i8 = 10;

// /// Notify successor that we may be predecessor
// pub const MSG_TYPE_NOTIFY: i8 = 11;

// /// Dummy Type
// pub const MSG_TYPE_NO_WAIT: i8 = 12;

// /// Request to copy successorlist
// pub const MSG_TYPE_COPY_SUCCESSORLIST: i8 = 13;

// /// Response to `MSG_TYPE_COPY_SUCCESSORLIST` with successor list
// pub const MSG_TYPE_COPY_SUCCESSORLIST_RESP: i8 = 14;

// pub const MSG_TYPE_EXIT: i8 = 15;
// pub const MSG_TYPE_EXIT_ACK: i8 = 16;
// pub const MSG_TYPE_GET: i8 = 17;
// pub const MSG_TYPE_PUT: i8 = 18;
// pub const MSG_TYPE_PUT_ACK: i8 = 19;
// pub const MSG_TYPE_GET_RESP: i8 = 20;
// pub const MSG_TYPE_FIND_SUCCESSOR_LINEAR: i8 = 21;
// pub const MSG_TYPE_REGISTER_CHILD: i8 = 22;
// pub const MSG_TYPE_REGISTER_CHILD_OK: i8 = 23;
// pub const MSG_TYPE_REGISTER_CHILD_EFULL: i8 = 24;
// pub const MSG_TYPE_REGISTER_CHILD_EWRONG: i8 = 25;
// pub const MSG_TYPE_REGISTER_CHILD_REDIRECT: i8 = 26;
// pub const MSG_TYPE_REFRESH_CHILD: i8 = 27;
// pub const MSG_TYPE_REFRESH_CHILD_OK: i8 = 28;
// pub const MSG_TYPE_REFRESH_CHILD_REDIRECT: i8 = 29;
