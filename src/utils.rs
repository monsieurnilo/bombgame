use crossterm::cursor;
use serde::{Deserialize, Serialize};
use std::io;
use std::{collections::HashMap, io::Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    // Constructeur pour créer une nouvelle position
    pub fn new(x: u16, y: u16) -> Position {
        Position { x, y }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionMessage {
    id: u64,
    pos: Position,
}

impl PositionMessage {
    // Constructeur pour créer un nouveau message de position
    pub fn new(id: u64, pos: Position) -> PositionMessage {
        PositionMessage { id, pos }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    players_positions: HashMap<u64, Position>,
}

impl GameState {
    // Constructeur pour initialiser un nouvel état de jeu
    pub fn new() -> GameState {
        GameState {
            players_positions: HashMap::new(),
        }
    }

    // Met à jour la position d'un joueur dans l'état du jeu
    pub fn update(&mut self, pos_message: PositionMessage) {
        let _ = self
            .players_positions
            .insert(pos_message.id, pos_message.pos);
    }

    // Dessine les positions des joueurs à l'écran
    pub fn draw<W: Write>(&self, stdout: &mut W) -> io::Result<()> {
        for pos in self.players_positions.values() {
            write!(stdout, "{}.", cursor::MoveTo(pos.x, pos.y))?; // Efface la position précédente avec un point
            write!(stdout, "{}%", cursor::MoveTo(pos.x, pos.y))?;
        }

        Ok(())
    }
}
