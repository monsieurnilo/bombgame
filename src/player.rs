use crate::map::Map;

use crossterm::cursor;
use std::io::{self, Write};

pub struct Player {
    x: u16,
    y: u16,
}

impl Player {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    // Détermine la nouvelle position du joueur en fonction de la touche pressée
    pub fn calculate_new_position(&self, key: crossterm::event::KeyCode) -> (u16, u16) {
        match key {
            crossterm::event::KeyCode::Up => {
                if self.y > 0 {
                    (self.x, self.y - 1)
                } else {
                    (self.x, self.y)
                }
            }
            crossterm::event::KeyCode::Down => {
                if self.y < 10 {
                    (self.x, self.y + 1)
                } else {
                    (self.x, self.y)
                }
            }
            crossterm::event::KeyCode::Left => {
                if self.x > 0 {
                    (self.x - 1, self.y)
                } else {
                    (self.x, self.y)
                }
            }
            crossterm::event::KeyCode::Right => {
                if self.x < 10 {
                    (self.x + 1, self.y)
                } else {
                    (self.x, self.y)
                }
            }
            _ => (self.x, self.y),
        }
    }

    // Déplace le joueur en fonction de la touche pressée
    pub fn move_player(&mut self, key: crossterm::event::KeyCode, map: &Map) {
        match key {
            crossterm::event::KeyCode::Up => {
                if self.y > 0 {
                    self.y -= 1
                }
            }
            crossterm::event::KeyCode::Down => {
                if (self.y as usize) < map.height() - 1 {
                    self.y += 1
                }
            }
            crossterm::event::KeyCode::Left => {
                if self.x > 0 {
                    self.x -= 1
                }
            }
            crossterm::event::KeyCode::Right => {
                if (self.x as usize) < map.width() - 1 {
                    self.x += 1
                }
            }
            _ => {}
        }
    }

    // Dessine le joueur à l'écran
    pub fn draw<W: Write>(&self, stdout: &mut W) -> io::Result<()> {
        write!(stdout, "{}.", cursor::MoveTo(self.x, self.y))?; // Efface la position précédente avec un point
        write!(stdout, "{}@", cursor::MoveTo(self.x, self.y))?; // Affiche le joueur à la nouvelle position
        Ok(())
    }
}
