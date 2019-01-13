use tokio::io::copy;
use tokio::net::TcpListener;
use tokio::prelude::*;

pub fn start_server() {
    info!("Bind the server socket.");
    // Bind the server's socket.
    let addr = "127.0.0.1:12345".parse().unwrap();
    info!("http://{}", addr);
    let listener = TcpListener::bind(&addr).expect("unable to bind TCP listener");

    // Pull out a stream of sockets for incoming connections
    let server = listener
        .incoming()
        .map_err(|e| error!("accept failed = {:?}", e))
        .for_each(|sock| {
            // Split up the reading and writing parts of the
            // socket.
            let (reader, writer) = sock.split();

            // A future that echos the data and returns how
            // many bytes were copied...
            let bytes_copied = copy(reader, writer);

            // ... after which we'll print what happened.
            let handle_conn = bytes_copied
                .map(|amt| info!("wrote {:?} bytes", amt))
                .map_err(|err| error!("IO error {:?}", err));

            // Spawn the future as a concurrent task.
            tokio::spawn(handle_conn)
        });

    // Start the Tokio runtime
    info!("Server is running.");
    tokio::run(server);
}
