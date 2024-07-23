use core::panic;
use std::collections::HashMap;
use std::io::{prelude::*, Error};
use std::net::{TcpListener, TcpStream};

use crate::map::Map;


use std::cell::RefCell;

use bincode;

use crate::utils::{self, GameState, Position, PositionMessage};


pub struct Server {
    state: RefCell<GameState>,
    listener: TcpListener,
}


impl Server {
    pub fn new(bind_socket: String) -> Result<Server, Error> {
        let listener = TcpListener::bind(bind_socket)?;

        Ok(Server { state: RefCell::new(GameState::new()), listener })
    }

    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                self.handle_stream(&stream).expect("Impossible de transmettre les informations au clients");
            }
        }
    }

    fn handle_stream(&self, mut stream: &TcpStream) -> Result<(), std::io::Error> {

        let mut buffer: Vec<u8> = vec![];

        stream.read(&mut buffer).expect("Impossible de repondre au client");

        let new_pos: utils::PositionMessage = match bincode::deserialize(&buffer) {
            Ok(pos) => pos,
            Err(e) => {
                println!("{}", e);
                PositionMessage::new(0, Position::new(0, 0))
            }
        };




        self.state.borrow_mut().update(new_pos);

        let encoded = bincode::serialize(&self.state).unwrap();
        stream.write(&encoded)?;

        Ok(())
    }
}

