use std::collections::HashMap;


pub struct Storage {
    data: HashMap<String,  String>
}

impl Storage {
    pub fn new() -> Storage {
        return Storage { data: HashMap::new() };
    }

    pub fn put(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<&String> {
        return self.data.get(&key);
    }

    pub fn delete(&mut self, key: String){
        self.data.remove(&key);
    }
}