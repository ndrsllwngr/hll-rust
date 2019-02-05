use std::collections::HashMap;
use num::bigint::BigInt;
use chrono::{DateTime, Local};
use colored::*;

use super::chord;

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
        let local: DateTime<Local> = Local::now();
        self.logs.push(format!("{} {}",local.format("%H:%M:%S").to_string().yellow(), str));
    }

    pub fn get_last_three_log_entries(&self) -> Vec<String> {
        let mut last_three_entries = Vec::new();
        if self.logs.len() >= 3 {
            last_three_entries.push(self.logs[self.logs.len()-3].clone());
            last_three_entries.push(self.logs[self.logs.len()-2].clone());
            last_three_entries.push(self.logs[self.logs.len()-1].clone());
        } else if self.logs.len() == 2 {
            last_three_entries.push(self.logs[self.logs.len()-2].clone());
            last_three_entries.push(self.logs[self.logs.len()-1].clone());
        } else if self.logs.len() == 1 {
            last_three_entries.push(self.logs[self.logs.len()-1].clone());
        } else {
            last_three_entries.push("No log entry found".italic().yellow().to_string())
        }
        last_three_entries.clone()
    }
}

pub fn make_hashed_key_value_pair(key: String, value: String) -> (BigInt, DHTEntry) {
    let id = chord::create_id(&key);
    (id, DHTEntry{key, value})
}
