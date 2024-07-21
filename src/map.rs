use crossterm::cursor;
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub struct Map {
    layout: Vec<String>,
}

impl Map {
    pub fn height(&self) -> usize {
        self.layout.len()
    }

    pub fn width(&self) -> usize {
        self.layout[0].len()
    }

    pub fn load(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let layout = reader.lines().collect::<Result<Vec<_>, _>>()?;
        Ok(Self { layout })
    }

    pub fn draw<W: Write>(&self, stdout: &mut W) -> io::Result<()> {
        for (y, line) in self.layout.iter().enumerate() {
            write!(stdout, "{}{}", cursor::MoveTo(0, y as u16), line)?;
        }
        Ok(())
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        self.layout[y].chars().nth(x).unwrap() == '─'
            || self.layout[y].chars().nth(x).unwrap() == '┌'
            || self.layout[y].chars().nth(x).unwrap() == '│'
            || self.layout[y].chars().nth(x).unwrap() == '┐'
            || self.layout[y].chars().nth(x).unwrap() == '└'
            || self.layout[y].chars().nth(x).unwrap() == '┘'
    }

    pub fn is_door(&self, x: usize, y: usize) -> bool {
        self.layout[y].chars().nth(x).unwrap() == '█'
    }

    pub fn is_bomb(&self, x: usize, y: usize) -> bool {
        self.layout[y].chars().nth(x).unwrap() == 'B'
    }

    pub fn spawn_bombs(&mut self, num_bombs: usize) {
        // Modifier `&self` en `&mut self` pour permettre la modification
        let mut rng = rand::thread_rng();
        let mut count = 0;

        while count < num_bombs {
            let x: usize = rng.gen_range(0..self.width());
            let y: usize = rng.gen_range(0..self.height());

            if self.layout[y].chars().nth(x) == Some('░') {
                // Correction: Utiliser une approche qui modifie effectivement la chaîne
                let mut chars: Vec<char> = self.layout[y].chars().collect();
                chars[x] = 'B';
                self.layout[y] = chars.into_iter().collect();
                count += 1;
            }
        }
    }

    pub fn random_position(&self) -> (u16, u16) {
        let mut rng = rand::thread_rng();
        let mut x: u16 = rng.gen_range(0..self.width() as u16);
        let mut y: u16 = rng.gen_range(0..self.height() as u16);
        while self
            .layout
            .get(y as usize)
            .and_then(|line| line.chars().nth(x as usize))
            != Some('▓')
        {
            x = rng.gen_range(0..self.width() as u16);
            y = rng.gen_range(0..self.height() as u16);
        }
        (x, y)
    }
}
