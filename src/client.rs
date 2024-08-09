use std::io::{stdin, BufRead, Read, Write}; // Import the stdin function, the BufRead trait, and the Read and Write traits from the std::io module
use std::net::TcpStream; // Import the TcpStream type from the std::net module
use std::thread; // Import the thread module

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?; // Create a TCP stream that connects to the server on localhost port 8080

    let mut stream_clone = stream.try_clone()?; // Clone the stream to be used in the new thread

    // Spawn a new thread to handle user input
    thread::spawn(move || {
        let stdin = stdin(); // Get a handle to the standard input stream
        let mut buffer = String::new(); // Create a buffer to store the user input

        loop {
            buffer.clear(); // Clear the buffer
            stdin.lock().read_line(&mut buffer).unwrap(); // Read a line of input from the user

            // If the user entered the string "ping", write the string "ping" to the socket
            if buffer.trim() == "ping" {
                if let Err(e) = stream_clone.write_all(b"ping") {
                    eprintln!("Error writing to socket: {}", e); // Print an error message if an error occurs
                    break; // Break out of the loop
                }
            }
        }

        Ok::<(), std::io::Error>(()) // Return Ok(()) to indicate that the thread has completed successfully
    });

    let mut buffer = [0; 4]; // Create a buffer to store the data read from the socket

    // Loop to read data from the socket
    loop {
        let n = match stream.read(&mut buffer) {
            Ok(n) if n == 0 => break, // If the connection is closed, break out of the loop
            Ok(n) => n, // Otherwise, store the number of bytes read into the n variable
            Err(e) => {
                eprintln!("Error reading from socket: {}", e); // Print an error message if an error occurs
                break; // Break out of the loop
            }
        };

        // If the data read from the socket is the string "pong", print a message to the console
        if &buffer[..n] == b"pong" {
            println!("Received pong from server");
        }
    }

    Ok(()) // Return Ok(()) to indicate that the program has completed successfully
}
