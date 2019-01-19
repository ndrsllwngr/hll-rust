use std::net::{TcpStream, SocketAddr};
use std::io::{BufWriter, Write};
use std::thread;

pub fn send_string_to_socket(addr: SocketAddr, msg: String) {
    thread::spawn(move || {
        match TcpStream::connect(addr) {
            Ok(stream) => {
                let mut writer = BufWriter::new(stream);
                writer.write_all(msg.as_bytes()).unwrap();
                info!("Sent msg: {}", msg);
            }
            Err(e) => {
                error!("Unable to send msg - Failed to connect: {}", e);
            }
        }
    });
}
