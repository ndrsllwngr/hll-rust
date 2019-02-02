use std::io::stdin;
use std::{error::Error, thread};
use std::process;

use super::network_util;
use super::protocols::*;
use super::util::*;
use super::node::OtherNode;
use super::storage::DHTEntry;


pub fn perform_user_interaction(node_as_other: OtherNode) -> Result<(), Box<Error>> {
    let info =
        "\n Hello there! What do you want to do? \n \n\
        1 - Store a key/value pair in the Chord network\n\
        2 - Find the value for a given key in the Chord network\n\
        3 - Delete a key/value pair from the Chord network\n\n\
        4 - Cancel interaction\n\
        5 - Terminate Node\n\n\
        Choose 1, 2, 3, 4 or 5 and press Enter!";
    info!("{}", info);

    loop {
        let buffer = &mut String::new();
        stdin().read_line(buffer).unwrap();
        match buffer.trim_right() {
            "1" => {
                store(node_as_other.clone());
                break;
            }
            "2" => {
                find(node_as_other.clone());
                break;
            }
            "3" => {
                delete(node_as_other.clone());
                break;
            }
            "4" => {
                break;
            }
            "5" => {
                process::exit(1);
            }
            _ => {
                println!("Please choose an valid option [1,2,3,4,5]");
            }
        };
    }

    Ok(())
}

fn store(node_as_other: OtherNode) -> Result<(), Box<Error>> {
    let mut key = "".to_owned();
    let mut value = "".to_owned();
    loop {
        println!("Enter the string that should be used as a KEY\n\
        (p.e.: A name):");
        let buffer = &mut String::new();
        stdin().read_line(buffer)?;
        match buffer.trim_right() {
            "" => {
                println!("Please Enter a valid Key name.");
            }
            k => {
                key = k.to_string();
                loop {
                    println!("Enter the string that should be stored as value for key {} \n\
                    (p.e.: A phone number)", key.clone());
                    let buffer2 = &mut String::new();
                    stdin().read_line(buffer2)?;
                    match buffer2.trim_right() {
                        "" => {
                            println!("Please Enter a valid value.");
                        }
                        v => {
                            value = v.to_owned();
                            break;
                        }
                    }
                }
                store_key_value(key, value, node_as_other);
                break;
            }
        }
    };
    Ok(())
}

fn find(node_as_other: OtherNode) -> Result<(), Box<Error>> {
    let mut key = "".to_owned();
    loop {
        println!("Enter a Key to look for in the network:");
        let buffer = &mut String::new();
        stdin().read_line(buffer)?;
        match buffer.trim_right() {
            "" => {
                println!("Please Enter a valid Key name.");
            }
            k => {
                key = k.to_string();
                find_key(key, node_as_other);
                break;
            }
        }
    };
    Ok(())
}

fn delete(node_as_other: OtherNode) -> Result<(), Box<Error>> {
    let mut key = "".to_owned();
    loop {
        println!("Enter a Key to look for in the network:");
        let buffer = &mut String::new();
        stdin().read_line(buffer)?;
        match buffer.trim_right() {
            "" => {
                println!("Please Enter a valid Key name.");
            }
            k => {
                key = k.to_string();
                delete_key(key, node_as_other);
                break;
            }
        }
    };
    Ok(())
}

fn store_key_value(key: String, value: String, node_as_other: OtherNode) {
    println!("store!");
    let key_id = create_id(&key);
    let req = Request::DHTStoreKey { data: (key_id, DHTEntry { key, value }) };
    info!("Trying to store data {:?}", req.clone());
    send_req(node_as_other, req);
}

fn find_key(key: String, node_as_other: OtherNode) {
    println!("find!");
    let key_id = create_id(&key);
    let req = Request::DHTFindKey { key_id };
    send_req(node_as_other, req);
}

fn delete_key(key: String, node_as_other: OtherNode) {
    println!("delete!");
    let key_id = create_id(&key);
    let req = Request::DHTDeleteKey { key_id };
    send_req(node_as_other, req);
}

fn send_req(node_as_other: OtherNode, req: Request) {
    let msg = Message::RequestMessage { sender: node_as_other.clone(), request: req };
    network_util::send_string_to_socket(node_as_other.get_ip_addr().clone(), serde_json::to_string(&msg).unwrap());
}
