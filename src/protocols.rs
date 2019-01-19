use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use super::node::OtherNode;
// Protocol states
pub const NOTIFY_PREDECESSOR: u8 = 0;
pub const NOTIFY_SUCCESSOR: u8 = 1;
pub const NOTIFY_JOIN: u8 = 2;
pub const FIND_SUCCESSOR: u8 = 3;
pub const FOUND_SUCCESSOR: u8 = 4;
pub const MESSAGE: u8 = 5;

#[derive(Serialize, Deserialize)]
pub struct Message {
    message_type: u8,
    next_finger: Option<usize>,
    id: Option<BigInt>,
}

#[derive(Serialize, Deserialize)]
pub struct Packet {
    from: OtherNode,
    message: Message,
}

impl Packet {
    pub fn new(from: OtherNode, message: Message) -> Packet {
        Packet {
            from,
            message,
        }
    }

    pub fn get_from(self) -> OtherNode {
        self.from
    }

    pub fn get_message(self) -> Message {
        self.message
    }


}

impl Message {
    pub fn new(message_type: u8, next_finger: Option<usize>, id: Option<BigInt>) -> Message {
        Message {
            message_type,
            next_finger,
            id,
        }
    }

    pub fn get_message_type(&self) -> u8 {
        self.message_type
    }

    pub fn get_next_finger(&self) -> Option<usize> {
        self.next_finger
    }

    pub fn get_id(&self) -> Option<BigInt> {
        self.id.clone()
    }

    pub fn set_id(&mut self, id: Option<BigInt>) {
        self.id = id
    }

    pub fn set_message_type(&mut self, message_type: u8) {
        self.message_type = message_type
    }

    pub fn print(&self) {
        match (self.next_finger, self.id.clone()) {
            (Some(next_finger), Some(id)) => info!(
                "Message: state: {}, next_finger: {}, id: {}",
                self.message_type, next_finger, id
            ),
            (Some(next_finger), None) => info!(
                "Message: state: {}, next_finger: {}",
                self.message_type, next_finger
            ),
            (None, Some(id)) => info!("Message: state: {}, id: {}", self.message_type, id),
            _ => info!("Message: state:{}", self.message_type),
        }
    }
}
