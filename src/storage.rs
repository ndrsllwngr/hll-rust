use std::collections::HashMap;
use num::bigint::BigInt;

use super::util::*;

#[derive(Clone)]
pub struct Storage {
    pub data: HashMap<BigInt, DHTEntry>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DHTEntry {
    pub key: String,
    pub value: String,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            data: HashMap::new(),
        }
    }

    pub fn put(&mut self, data: (BigInt, DHTEntry)) {
        self.data.insert(data.0, data.1);
    }

    pub fn get(&self, key_id: &BigInt) -> Option<&DHTEntry> {
        self.data.get(key_id)
    }

    pub fn delete(&mut self, key_id: &BigInt) -> Option<DHTEntry> {
        self.data.remove(key_id)
    }
}

pub fn make_hashed_key_value_pair(key: String, value: String) -> (BigInt, DHTEntry) {
    let id = create_id(&key);
    (id, DHTEntry{key, value})
}
