use std::net::{TcpStream, SocketAddr};
use std::io::{BufWriter, Write};
use std::thread;


pub fn send_string_to_socket(addr: SocketAddr, msg: String, sending_node_name: String) {
    let builder = thread::Builder::new().name("Send".to_string());
    let handle = builder.spawn(move || {
        match TcpStream::connect(addr.clone()) {
            Ok(stream) => {
                let mut writer = BufWriter::new(stream);
                writer.write_all(msg.as_bytes()).unwrap();
                debug!("Sent msg: {}", msg);
            }
            Err(e) => {
                error!("Unable to send msg to {} - Failed to connect: {}",addr, e);
            }
        }
    }).unwrap();
    if let Err(e) = handle.join() {
        error!("{:?}", e)
    }
}
