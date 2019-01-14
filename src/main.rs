extern crate crypto;
extern crate num_bigint;
extern crate num;

#[macro_use]
extern crate log;
extern crate log4rs;

use std::net::{IpAddr, Ipv4Addr};
use std::collections::HashMap;
use num_bigint::{BigInt, Sign, ToBigInt};
use std::str;

mod node;
mod storage;
mod finger;
mod util;
mod network;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("booting up");

    //let id = "node_id".bytes();
    //let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
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


    test_endian("a94a8fe5ccb19ba61c4c0873d391e987982fbbd3");
    test_modulo_bigint();
    test_compare_bigint();

    node::dispatch(1, 3);
    network::start_server();
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

    info!("{}",big_int_plus_b);


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

    custom_print(str_byte_vec_no_b);
    custom_print(str_byte_vec_no_l);
    custom_print(str_byte_vec_plus_b);
    custom_print(str_byte_vec_plus_l);
    custom_print(str_byte_vec_minus_b);
    custom_print(str_byte_vec_minus_l);
}

fn test_modulo_bigint(){


    let should_be_two = BigInt::modpow(&12.to_bigint().unwrap(),&1.to_bigint().unwrap(), &10.to_bigint().unwrap());
    info!("{}",should_be_two)
}

fn test_compare_bigint(){
    let one = &1.to_bigint().unwrap();
    let two = &2.to_bigint().unwrap();
    let two_again = &2.to_bigint().unwrap();
    let three = &3.to_bigint().unwrap();

    info!("{}",two == two_again);
    info!("{}",two < three);
    info!("{}",two > one);
}

fn custom_print(result: Result<&str, std::str::Utf8Error>) {
    match result {
        Ok(n)  => info!("{}", n),
        Err(e) => error!("Error: {}", e),
    }
}




