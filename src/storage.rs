use std::collections::HashMap;


pub struct Storage<'a> {
        pub data: HashMap<&'a str, &'a str>
}