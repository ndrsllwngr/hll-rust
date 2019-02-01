use std::collections::HashMap;
use num::bigint::{BigInt, Sign, ToBigInt};

#[derive(Clone)]
pub struct Storage {
    data: HashMap<BigInt, String>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            data: HashMap::new(),
        }
    }

    pub fn put(&mut self, data: (BigInt, String)) {
        self.data.insert(data.0, data.1);
    }

    pub fn get(&self, key: &BigInt) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &BigInt) -> Option<String> {
        self.data.remove(key)
    }
}
