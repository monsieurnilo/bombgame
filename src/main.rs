mod game;
mod map;
mod player;

use crossterm::{terminal, ExecutableCommand};
use std::io::{self};

fn main() -> std::io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    game::run_game()?;

    terminal::disable_raw_mode()?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}
