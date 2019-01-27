use num_bigint::BigInt;

use super::node::OtherNode;

// Protocol states
pub const NOTIFY_PREDECESSOR: i8 = 0;

pub const NOTIFY_SUCCESSOR: i8 = 1;

pub const NOTIFY_JOIN: i8 = 2;

pub const FIND_SUCCESSOR: i8 = 3;

pub const FOUND_SUCCESSOR: i8 = 4;

pub const MESSAGE: i8 = 5;

#[derive(Serialize, Deserialize)]
pub struct Packet {
    from: OtherNode,
    message: Message,
}

impl Packet {
    pub fn new(from: OtherNode, message: Message) -> Packet {
        Packet { from, message }
    }

    pub fn get_from(&self) -> &OtherNode {
        &self.from
    }

    pub fn get_message(&self) -> &Message {
        &self.message
    }
}

/// * `message_type`= Message type (e.g. `MSG_TYPE_FIND_SUCCESSOR`)
/// * `next_finger` = FingerTable index, we need this information because of `if let Some(msg_id) = msg.get_id() {`
/// * `id` = key of FingerEntry
#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
    message_type: i8,
    next_finger: Option<usize>,
    id: Option<BigInt>,
}

impl Message {
    pub fn new(message_type: i8, next_finger: Option<usize>, id: Option<BigInt>) -> Message {
        Message {
            message_type,
            next_finger,
            id,
        }
    }

    pub fn get_message_type(&self) -> i8 {
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

    pub fn set_message_type(&mut self, message_type: i8) {
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

#[derive(Serialize, Deserialize)]
pub enum Request {
    FindSuccessor {
        id: BigInt
    },
    GetPredecessor,
    Notify {
        node: OtherNode,
    }
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    FoundSuccessor {
        successor: OtherNode
    },
    AskFurther{
        next_node: OtherNode
    },
    GetPredecessorResponse {
        predecessor: Option<OtherNode>
    },
    NotifyResponse

}
