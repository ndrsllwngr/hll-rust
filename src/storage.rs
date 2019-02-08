use std::collections::hash_map::Iter;
use std::collections::HashMap;

use chrono::{DateTime, Local};
use colored::*;
use num::bigint::BigInt;

use super::chord;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DHTEntry {
    key: String,
    value: String,
}

impl DHTEntry {
    pub fn new(key: String, value: String) -> DHTEntry {
        DHTEntry { key, value }
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}

#[derive(Clone)]
pub struct Storage {
    data: HashMap<BigInt, DHTEntry>,
    logs: Vec<String>,

}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            data: HashMap::new(),
            logs: Vec::new(),
        }
    }

    pub fn get_data_as_vec(&self) -> Vec<(BigInt, DHTEntry)> {
        self.data.iter().map(|(id, entry)| (id.clone(), entry.clone())).collect()
    }
    pub fn get_data_as_iter(&self) -> Iter<BigInt, DHTEntry> {
        self.data.iter()
    }

    pub fn is_data_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn store_key(&mut self, data: (BigInt, DHTEntry)) {
        self.data.insert(data.0, data.1);
    }

    pub fn get_key(&self, key_id: &BigInt) -> Option<&DHTEntry> {
        self.data.get(key_id)
    }

    pub fn delete_key(&mut self, key_id: &BigInt) -> Option<DHTEntry> {
        self.data.remove(key_id)
    }

    pub fn write_log_entry(&mut self, str: String) {
        let local: DateTime<Local> = Local::now();
        self.logs.push(format!("{} {}", local.format("%H:%M:%S").to_string().yellow(), str));
    }

    pub fn get_last_three_log_entries(&self) -> Vec<String> {
        let mut last_three_entries = Vec::new();
        if self.logs.len() >= 3 {
            last_three_entries.push(self.logs[self.logs.len() - 3].clone());
            last_three_entries.push(self.logs[self.logs.len() - 2].clone());
            last_three_entries.push(self.logs[self.logs.len() - 1].clone());
        } else if self.logs.len() == 2 {
            last_three_entries.push(self.logs[self.logs.len() - 2].clone());
            last_three_entries.push(self.logs[self.logs.len() - 1].clone());
        } else if self.logs.len() == 1 {
            last_three_entries.push(self.logs[self.logs.len() - 1].clone());
        } else {
            last_three_entries.push("No log entry found".italic().yellow().to_string())
        }
        last_three_entries.clone()
    }
}

pub fn make_hashed_key_value_pair(key: String, value: String) -> (BigInt, DHTEntry) {
    let id = chord::create_id(&key);
    (id, DHTEntry::new(key, value))
}
