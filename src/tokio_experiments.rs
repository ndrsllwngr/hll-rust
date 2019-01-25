use tokio::io;
use tokio::net::{TcpStream,TcpListener};
use tokio::prelude::*;

use futures::{Future, Stream};

use std::env;
use std::net::SocketAddr;
use std::str;
use std::io::BufReader;



pub fn listen_and_answer()-> Result<(), Box<std::error::Error>> {

    let addr = env::args().nth(1).unwrap_or("127.0.0.1:12345".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(|socket| {
        println!("accepted socket; addr={:?}", socket.peer_addr()?);

        let buf = vec![];
        let buf_reader = BufReader::new(socket);

        let connection = io::read_until(buf_reader, b'\n', buf)
            .and_then(|(socket, buf)| {
                io::write_all(socket.into_inner(), buf)
            })
            .then(|_| Ok(())); // Just discard the socket and buffer

        // Spawn a new task that processes the socket:
        tokio::spawn(connection);

        Ok(())
    }).map_err(|e| println!("failed to accept socket; error = {:?}", e));
    tokio::run(server);
    Ok(())
}


pub fn write_to_stream_with_answer(addr_str: String) -> Result<(), Box<std::error::Error>> {
    let addr = addr_str.parse()?;
    let client = TcpStream::connect(&addr).and_then(|stream| {
        println!("created stream");
        io::write_all(stream, "hello world\n").and_then(|(stream, msg)| {
            let sock = BufReader::new(stream);
            io::read_until(sock, b'\n', vec![]).and_then(|(stream, buf)| {
                let msg = str::from_utf8(&buf).unwrap();
                println!("Got reply: {}",msg);
                Ok(())
            })
        })
    })
        .map_err(|err| {
            println!("connection error = {:?}", err);
        });
    tokio::run(client);
    Ok(())
}