use num_bigint::BigInt;

use super::node::OtherNode;

#[derive(Serialize, Deserialize)]
pub struct RequestMessage {
    pub sender: OtherNode,
    pub request: Request,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMessage {
    pub sender: OtherNode,
    pub response: Response,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Request {
    FindSuccessor {
        id: BigInt
    },
    GetPredecessor,
    Notify {
        node: OtherNode,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Response {
    FoundSuccessor {
        successor: OtherNode
    },
    AskFurther {
        next_node: OtherNode
    },
    GetPredecessorResponse {
        predecessor: Option<OtherNode>
    },
    NotifyResponse,
}
