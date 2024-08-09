use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let mut stream_clone = stream.try_clone()?;

    thread::spawn(move || {
        let mut buffer = [0; 4];

        loop {
            if let Err(e) = stream_clone.write_all(b"ping") {
                eprintln!("Error writing to socket: {}", e);
                break;
            }

            thread::sleep(Duration::from_secs(1));
        }
    });

    let mut buffer = [0; 4];

    loop {
        let n = match stream.read(&mut buffer) {
            Ok(n) if n == 0 => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                break;
            }
        };

        if &buffer[..n] == b"pong" {
            println!("Received pong from server");
        }
    }

    Ok(())
}
