use std::collections::HashMap;


pub struct Storage<'a> {
    data: HashMap<&'a str, &'a str>
}

impl Storage {
    pub fn new() -> Storage {
        return Storage { data: HashMap::new() };
    }

    fn put(key: str, value: str) {
        data.insert(key, value);
    }

    fn get(key: str) -> Option<&str> {
        return data.get(key);
    }

    fn delete(key: str){
        remove(key);
    }
}