use num_bigint::{BigInt, Sign};

// Protocol states
pub const NOTIFY_PREDECESSOR: u8 = 0;
pub const NOTIFY_SUCCESSOR: u8 = 1;
pub const NOTIFY_JOIN: u8 = 2;
pub const FIND_SUCCESSOR: u8 = 3;
pub const FOUND_SUCCESSOR: u8 = 4;
pub const MESSAGE: u8 = 5;

pub struct Message {
    state: u8,
    next_finger: Option<usize>,
    id: Option<BigInt>,
}

impl Message {
    pub fn new(state: u8, next_finger: Option<usize>, id: Option<BigInt>) -> Message {
        return Message {
            state,
            next_finger,
            id,
        };
    }

    pub fn get_state(&self) -> u8 {
        self.state
    }

    pub fn get_next_finger(&self) -> Option<usize> {
        self.next_finger
    }

    pub fn get_id(&self) -> Option<BigInt> {
        self.id.clone()
    }

    pub fn print(&self) {
        match (self.next_finger.clone(), self.id.clone()) {
            (Some(next_finger), Some(id)) => info!(
                "Message: state: {}, next_finger: {}, id: {}",
                self.state, next_finger, id
            ),
            (Some(next_finger), None) => info!(
                "Message: state: {}, next_finger: {}",
                self.state, next_finger
            ),
            (None, Some(id)) => info!("Message: state: {}, id: {}", self.state, id),
            _ => info!("Message: state:{}", self.state),
        }
    }
}
