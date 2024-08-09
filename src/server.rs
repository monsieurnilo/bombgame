use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let mut stream = stream?;

        let mut buffer = [0; 4];
        stream.read_exact(&mut buffer)?;

        if &buffer == b"ping" {
            stream.write_all(b"pong")?;
        }
    }

    Ok(())
}