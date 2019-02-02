use std::io::stdin;
use signal_hook::{iterator::Signals, SIGSTOP};
use std::{error::Error, thread};
use std::process;

use super::network_util;
use super::protocols::*;
use super::util::*;
use super::node::OtherNode;
use super::storage::DHTEntry;


pub fn user_input(node_as_other: OtherNode) -> Result<(), Box<Error>> {
    loop {
        println!("Hello, there!  Which Storage Interaction do you want to perform?");
        println!("1 - Store a Key, Value Pair in the Chord Network");
        println!("2 - Find the Value for a given Key in the Chord Network");
        println!("3 - Delete a Key in the Chord Network ");
        println!("4 - Terminate program\n");


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
                process::exit(1);
            }
            _ => {
                println!("Please Enter an valid Option");
            }
        };
    }

    Ok(())
}

fn store(node_as_other: OtherNode) -> Result<(), Box<Error>> {
    let mut key = "".to_owned();
    let mut value = "".to_owned();
    loop {
        println!("Enter a Key (for example your name):");
        let buffer = &mut String::new();
        stdin().read_line(buffer)?;
        match buffer.trim_right() {
            "" => {
                println!("Please Enter a valid Key name.");
            }
            k => {
                key = k.to_string();
                loop {
                    println!("Enter a value to store (for example your phone number)");
                    let buffer2 = &mut String::new();
                    stdin().read_line(buffer2)?;
                    match buffer2.trim_right() {
                        "" => {
                            println!("Please Enter a valid value name.");
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
