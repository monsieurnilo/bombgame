use serde::{Serialize, Deserialize};
use std::{collections::HashMap, io::Write};
use crossterm::cursor;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    x: u16,
    y: u16
}


impl Position {
    pub fn new(x: u16, y: u16) -> Position {
        Position { x, y }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionMessage {
    id: u64,
    pos: Position
}

impl PositionMessage {
    pub fn new(id: u64, pos: Position) -> PositionMessage {
        PositionMessage { id, pos }
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct GameState {
    players_positions: HashMap<u64, Position>,
}




impl GameState {

    pub fn new() -> GameState {
        GameState { players_positions: HashMap::new() }
    }

    pub fn update(&mut self, mut pos_message: PositionMessage) {
        let _ = self.players_positions.get_mut(&pos_message.id).insert(&mut pos_message.pos);
    }

    pub fn draw<W: Write>(&self, stdout: &mut W) -> io::Result<()> {

        for pos in self.players_positions.values() {
            write!(stdout, "{}.", cursor::MoveTo(pos.x, pos.y))?; // Efface la position précédente avec un point
            write!(stdout, "{}%", cursor::MoveTo(pos.x, pos.y))?;
        }

        Ok(())
    }


}