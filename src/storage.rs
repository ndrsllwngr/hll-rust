use std::collections::HashMap;
use num::bigint::BigInt;

use super::util::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DHTEntry {
    pub key: String,
    pub value: String,
}

#[derive(Clone)]
pub struct Storage {
    pub data: HashMap<BigInt, DHTEntry>,
    logs: Vec<String>,
    
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            data: HashMap::new(),
            logs: Vec::new(),
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

    pub fn write_log_entry(&mut self, str: String){
        self.logs.push(str);
    }

    pub fn get_all_log_entries(&self) -> Vec<String> {
        self.logs.clone()
    }

    pub fn get_last_log_entry(&self) -> String {
        if self.logs.len() > 0 {
            self.logs[self.logs.len()-1].clone()
        } else {
            "No log entry found".to_string()
        }
    }
}

pub fn make_hashed_key_value_pair(key: String, value: String) -> (BigInt, DHTEntry) {
    let id = create_id(&key);
    (id, DHTEntry{key, value})
}
