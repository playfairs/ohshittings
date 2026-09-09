use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use ohshit::{Diagnostic, Location, Logger, Record};

struct RunningServer {
    handle: JoinHandle<std::io::Result<usize>>,
}

fn start_server(address: &str, logger: Logger) -> std::io::Result<RunningServer> {
    let listener = TcpListener::bind(address)?;
    listener.set_nonblocking(true)?;
    ohshit::info!(target: "network", address = address; "server bound to {}", address);

    let handle = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(3);
        let mut connections = 0;
        while Instant::now() < deadline {
            match listener.accept() {
                Ok((mut stream, peer)) => {
                    connections += 1;
                    let mut request = [0; 2048];
                    let bytes = stream.read(&mut request)?;
                    let response =
                        b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nOK";
                    stream.write_all(response)?;
                    logger.log(
                        Record::new(ohshit::Level::Info, "request handled")
                            .with_target("network")
                            .with_context("peer", peer.to_string())
                            .with_context("request_bytes", bytes)
                            .with_context("connection_state", "closed"),
                    );
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => return Err(error),
            }
        }
        Ok(connections)
    });
    Ok(RunningServer { handle })
}

pub fn exercise_networking(logger: &Logger) -> Result<(), Box<dyn Error>> {
    let address = "127.0.0.1:8000";
    let server = start_server(address, logger.clone())?;

    match TcpListener::bind(address) {
        Ok(_) => return Err("the occupied-port bind unexpectedly succeeded".into()),
        Err(error) => {
            let diagnostic =
                Diagnostic::new(format!("Could not bind a second listener to {address}"))
                    .with_cause(error.kind().to_string())
                    .with_reason(error.to_string())
                    .with_action("use the existing listener or choose another port")
                    .with_location(Location::new(file!(), line!()));
            ohshit::ohshit!(target: "network", diagnostic);
        }
    }

    let mut client = TcpStream::connect(address)?;
    client.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")?;
    let mut response = String::new();
    client.read_to_string(&mut response)?;
    if !response.contains("200 OK") || !response.ends_with("OK") {
        return Err(format!("unexpected TCP response: {response:?}").into());
    }

    let connections = server
        .handle
        .join()
        .map_err(|_| "server thread panicked")??;
    if connections != 1 {
        return Err(format!("expected one accepted connection, got {connections}").into());
    }
    Ok(())
}
