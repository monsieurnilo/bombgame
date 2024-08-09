use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    stream.write_all(b"ping")?;

    let mut buffer = [0; 4];
    stream.read_exact(&mut buffer)?;

    if &buffer == b"pong" {
        println!("Received pong from server");
    }

    Ok(())
}