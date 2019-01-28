use num_bigint::BigInt;

use super::node::OtherNode;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message {
    RequestMessage {
        sender: OtherNode,
        request: Request,
    },
    ResponseMessage {
        sender: OtherNode,
        response: Response,
    }
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
