use std::io::{prelude::*, Error};
use std::net::{TcpListener, TcpStream};



use std::cell::RefCell;

use bincode;

use crate::utils::{self, GameState, Position, PositionMessage};


use std::fs::File;


pub struct Server {
    state: RefCell<GameState>,
    listener: TcpListener,
}


impl Server {
    pub fn new(bind_socket: String) -> Result<Server, Error> {
        let listener = TcpListener::bind(bind_socket)?;
        //println!("Je suis dans le server");

        Ok(Server { state: RefCell::new(GameState::new()), listener })
    }

    pub fn listen(&mut self) {
        loop {
            for stream in self.listener.incoming() {
                if let Ok(stream) = stream {
                    match self.handle_stream(&stream) {
                        Ok(()) => (),
                        Err(e) => (), //println!("{}", e),
                    }
                }
            }
        }
    }

    fn handle_stream(&self, mut stream: &TcpStream) -> Result<(), std::io::Error> {

        let mut buffer: Vec<u8> = vec![0; 1024];

        // match stream.read(&mut buffer) {
        //     Ok(size) => println!("Readed size : {}", size),
        //     Err(e) => println!("Cannot read data : {}", e),
        // }

        let mut logs_file = File::create("server_logs.txt")?;

        while match stream.read(&mut buffer) {
            Ok(0) => false, // End the connection
            Ok(size) => {
                ////println!("Size is : {}", size);
                let bindata = buffer[..size].as_ref();
                // Echo the message back to the client
                let new_pos: utils::PositionMessage = match bincode::deserialize(bindata) {
                    Ok(pos) => pos,
                    Err(e) => {
                        ////println!("Erreur de lecture : {}", e);
                        PositionMessage::new(0, Position::new(0, 0))
                    }
                };

                self.state.borrow_mut().update(dbg!(new_pos));

                let encoded: Vec<u8> = bincode::serialize(&self.state).unwrap();

                stream.write(&encoded)?;


                logs_file.write(format!("{:?}", self.state).as_bytes())?;


                true
            }
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                false
            }
        } {}

        Ok(())
    }
}

