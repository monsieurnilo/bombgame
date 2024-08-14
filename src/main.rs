mod game;
mod map;
mod player;
mod server;
mod utils;


use crossterm::{terminal, ExecutableCommand};
use std::io::{self};
use std::env;


fn main() -> std::io::Result<()> {

    if env::args().len() == 1 {
        println!("Choisir mode client ou serveur");
        return Ok(())
    }

    if env::args().nth(1).unwrap() == "server" {
        let mut server1 = server::Server::new("0.0.0.0:8864".to_string()).unwrap();
        server1.listen();
        return Ok(())
    } else {
        let mut stdout = io::stdout();
        stdout.execute(terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;


        
    
        match game::run_game(dbg!(env::args().nth(1).unwrap().to_string())) {
            Ok(()) => (),
            Err(e) => print!("Mon erreur est {}", e),
        }
    
        terminal::disable_raw_mode()?;
        //stdout.execute(terminal::LeaveAlternateScreen)?;
    }


    // Ok(())

    Ok(())
}
