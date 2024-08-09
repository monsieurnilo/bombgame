use std::io::{Read, Write}; // Import the Read and Write traits from the std::io module
use std::net::{TcpListener, TcpStream}; // Import the TcpListener and TcpStream types from the std::net module
use std::thread; // Import the thread module

// Define a function to handle each client connection
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 4]; // Create a buffer to store the data read from the socket

    loop {
        // Read data from the socket into the buffer
        let n = match stream.read(&mut buffer) {
            Ok(n) if n == 0 => return, // If the connection is closed, return from the function
            Ok(n) => n, // Otherwise, store the number of bytes read into the n variable
            Err(e) => {
                eprintln!("Error reading from socket: {}", e); // Print an error message if an error occurs
                return; // Return from the function
            }
        };

        // If the data read from the socket is the string "ping", write the string "pong" to the socket
        if &buffer[..n] == b"ping" {
            if let Err(e) = stream.write_all(b"pong") {
                eprintln!("Error writing to socket: {}", e); // Print an error message if an error occurs
                return; // Return from the function
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?; // Create a TCP listener that listens on localhost port 8080

    // Loop over incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the connection
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e); // Print an error message if an error occurs
            }
        }
    }

    Ok(()) // Return Ok(()) to indicate that the program has completed successfully
}



#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::{TcpListener, TcpStream};

    #[tokio::test]
    async fn test_server_client() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = [0; 4];
            socket.read_exact(&mut buf).await.unwrap();
            assert_eq!(&buf, b"ping");
            socket.write_all(b"pong").await.unwrap();
        });

        let mut socket = TcpStream::connect(addr).await.unwrap();
        socket.write_all(b"ping").await.unwrap();
        let mut buf = [0; 4];
        socket.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"pong");
    }
}
