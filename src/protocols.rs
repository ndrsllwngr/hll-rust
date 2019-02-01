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
    },
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
        finger_id: BigInt,
    },
    GetSuccessorList,
    DHTStoreKey {
        data: (BigInt, String)
    },
    DHTFindKey {
        key: BigInt
    },
    DHTDeleteKey {
        key: BigInt
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
    FoundSuccessorFinger {
        index: usize,
        finger_id: BigInt,
        successor: OtherNode,
    },
    AskFurtherFinger {
        index: usize,
        finger_id: BigInt,
        next_node: OtherNode,
    },
    GetSuccessorListResponse {
        successor_list: Vec<OtherNode>
    },
    DHTStoredKey,
    DHTFoundKey {
        data: (BigInt, Option<String>)
    },
    DHTDeletedKey {
        key_existed: bool
    },
    DHTAskFurtherStore {
        next_node: OtherNode,
        data: (BigInt, String),
    },
    DHTAskFurtherFind {
        next_node: OtherNode,
        key: BigInt,
    },
    DHTAskFurtherDelete {
        next_node: OtherNode,
        key: BigInt,
    },
}
