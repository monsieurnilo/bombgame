mod game;
mod map;
mod player;
mod server;
mod utils;

use crossterm::{terminal, ExecutableCommand};
use std::env;
use std::io::{self};

pub fn run_server() -> std::io::Result<()> {
    let mut server1 = server::Server::new("0.0.0.0:8864".to_string()).unwrap();
    server1.listen();
    Ok(())
}

pub fn run_client(mode: String) -> std::io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    match game::run_game(mode) {
        Ok(()) => (),
        Err(e) => print!("Mon erreur est {}", e),
    }

    terminal::disable_raw_mode()?;
    Ok(())
}

pub fn main() -> std::io::Result<()> {
    // Vérifier si l'utilisateur a fourni un argument pour choisir le mode client ou serveur
    if env::args().len() == 1 {
        println!("Choisir mode client ou serveur");
        return Ok(());
    }

    if env::args().nth(1).unwrap() == "server" {
        run_server()
    } else {
        run_client(env::args().nth(1).unwrap().to_string())
    }
}
