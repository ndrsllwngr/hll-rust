use std::io::stdin;
use std::{error::Error};
use std::process;
use std::net::SocketAddr;

use super::network_util;
use super::protocols::*;
use super::util::*;
use super::node::OtherNode;
use super::storage;


pub fn perform_user_interaction(node_as_other: OtherNode) -> Result<(), Box<Error>> {
    let info =
        "\n Hello there! What do you want to do? \n \n\
        1 - Store a key/value pair in the Chord network\n\
        2 - Find the value for a given key in the Chord network\n\
        3 - Delete a key/value pair from the Chord network\n\n\
        4 - Kill a Chord network peer\n\n\
        5 - Cancel interaction\n\
        6 - Terminate Node\n\n\
        Choose 1, 2, 3, 4, 5 or 6 and press Enter!";
    info!("{}", info);

    loop {
        let buffer = &mut String::new();
        stdin().read_line(buffer).unwrap();
        match buffer.trim_right() {
            "1" => {
                store(node_as_other.clone()).expect("store failed");
                break;
            }
            "2" => {
                find(node_as_other.clone()).expect("find failed");
                break;
            }
            "3" => {
                delete(node_as_other.clone()).expect("delete failed");
                break;
            }
            "4" => {
                kill().expect("kill failed");
                break;
            }
            "5" => {
                break;
            }
            "6" => {
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
    let mut key;
    let mut value;
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
    let mut key;
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
    let mut key;
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

fn kill() -> Result<(), Box<Error>> {
    let mut key;
    loop {
        println!("Enter <IP>:<Port> (i.e. 127.0.0.1:10000) of a to be killed chord network peer:");
        let buffer = &mut String::new();
        stdin().read_line(buffer)?;
        match buffer.trim_right() {
            "" => {
                println!("Please enter a valid SocketAddr.");
            }
            k => {
                key = k.to_string();
                kill_node(key);
                break;
            }
        }
    };
    Ok(())
}

fn kill_node(key: String){
    let target_ip = key.parse::<SocketAddr>().unwrap();
    let msg = Message::Kill;
    network_util::send_string_to_socket(target_ip, serde_json::to_string(&msg).unwrap());
}

fn store_key_value(key: String, value: String, node_as_other: OtherNode) {
    let req = Request::DHTStoreKey { data:  storage::make_hashed_key_value_pair(key, value)};
    info!("Trying to store data {:?}", req.clone());
    send_req(node_as_other, req);
}

fn find_key(key: String, node_as_other: OtherNode) {
    let key_id = create_id(&key);
    let req = Request::DHTFindKey { key_id };
    send_req(node_as_other, req);
}

fn delete_key(key: String, node_as_other: OtherNode) {
    let key_id = create_id(&key);
    let req = Request::DHTDeleteKey { key_id };
    send_req(node_as_other, req);
}

fn send_req(node_as_other: OtherNode, req: Request) {
    let msg = Message::RequestMessage { sender: node_as_other.clone(), request: req };
    network_util::send_string_to_socket(*node_as_other.get_ip_addr(), serde_json::to_string(&msg).unwrap());
}
