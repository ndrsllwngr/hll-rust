use std::net::IpAddr;

use super::finger::FingerTable;
use super::storage::Storage;

#[derive(Clone)]
pub struct Config {
    pub id: Vec<u8>,
    pub ip_addr: IpAddr
}

pub struct Node<'a> {
    pub config: Config,
    //pub predecessor: &'a Node<'a>,
    // pub finger_table: FingerTable<'a>,
    pub storage: Storage<'a>
}
