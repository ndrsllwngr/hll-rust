use num_bigint::BigInt;

use super::node::OtherNode;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message {
    Ping {
        sender: OtherNode
    },
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
    FindSuccessorFinger {
        index: usize,
        finger_id: BigInt
    },
    GetSuccessorList
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
    FoundSuccessorFinger {
        index: usize,
        finger_id: BigInt,
        successor: OtherNode
    },
    AskFurtherFinger {
        index: usize,
        finger_id: BigInt,
        next_node: OtherNode
    },
    GetSuccessorListResponse {
        successor_list: Vec<OtherNode>
    }

}
