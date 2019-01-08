use std::net::IpAddr;
use std::str::Bytes;

use super::finger::FingerTable;
use super::storage::Storage;

pub struct Config<'a> {
    pub id: Bytes<'a>,
    pub ip_addr: IpAddr
}

pub struct Node<'a> {
    pub config: Config<'a>,
    //pub predecessor: &'a Node<'a>,
    // pub finger_table: FingerTable<'a>,
    pub storage: Storage<'a>
}