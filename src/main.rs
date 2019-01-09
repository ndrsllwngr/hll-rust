extern crate crypto;
extern crate num_bigint;
extern crate num;

use std::str::Bytes;
use std::net::{IpAddr, Ipv4Addr};
use std::collections::HashMap;
use num_bigint::{BigInt, Sign};
use std::str;



mod node;
mod storage;
mod finger;
mod constants;
mod util;

fn main() {
    println!("Hello, world!");

    let id = "node_id".bytes();
    let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let mut data = HashMap::new();
    data.insert("key", "value");
    /* let config = node::Config{id , ip_addr};
    let storage = storage::Storage{data};
    let node = node::Node{config, storage};

    let finger_table: finger::FingerTable = finger::new_finger_table(&node, 5);
    let mut bytes: Bytes = finger_table[0].id.clone();

    assert_eq!(Some(b'f'), bytes.next());
    assert_eq!(Some(b'i'), bytes.next());
    assert_eq!(Some(b'n'), bytes.next());
    */

    test_endian("test");
}

fn test_endian(str: &str) {
    let byte_vec = str.as_bytes().to_vec();

    // 3 and 5 work!
    let big_int_no_b = BigInt::from_bytes_be(Sign::NoSign, &byte_vec);
    let big_int_no_l = BigInt::from_bytes_le(Sign::NoSign, &byte_vec);
    let big_int_plus_b = BigInt::from_bytes_be(Sign::Plus, &byte_vec);
    let big_int_plus_l = BigInt::from_bytes_le(Sign::Plus, &byte_vec);
    let big_int_minus_b = BigInt::from_bytes_be(Sign::Minus, &byte_vec);
    let big_int_minus_l = BigInt::from_bytes_le(Sign::Minus, &byte_vec);



    let byte_vec_no_b = big_int_no_b.to_bytes_be();
    let byte_vec_no_l = big_int_no_l.to_bytes_be();
    let byte_vec_plus_b = big_int_plus_b.to_bytes_be();
    let byte_vec_plus_l = big_int_plus_l.to_bytes_le();
    let byte_vec_minus_b = big_int_minus_b.to_bytes_be();
    let byte_vec_minus_l = big_int_minus_l.to_bytes_le();


    let str_byte_vec_no_b = std::str::from_utf8(&byte_vec_no_b.1);
    let str_byte_vec_no_l = std::str::from_utf8(&byte_vec_no_l.1);
    let str_byte_vec_plus_b = std::str::from_utf8(&byte_vec_plus_b.1);
    let str_byte_vec_plus_l = std::str::from_utf8(&byte_vec_plus_l.1);
    let str_byte_vec_minus_b = std::str::from_utf8(&byte_vec_minus_b.1);
    let str_byte_vec_minus_l = std::str::from_utf8(&byte_vec_minus_l.1);

    print(str_byte_vec_no_b);
    print(str_byte_vec_no_l);
    print(str_byte_vec_plus_b);
    print(str_byte_vec_plus_l);
    print(str_byte_vec_minus_b);
    print(str_byte_vec_minus_l);
}

fn print(result: Result<&str, std::str::Utf8Error>) {
    match result {
        Ok(n)  => println!("{}", n),
        Err(e) => println!("Error: {}", e),
    }
}




