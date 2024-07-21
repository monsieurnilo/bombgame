use crossterm::event::{self, Event, KeyCode};
use crossterm::{cursor, ExecutableCommand};
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

use crate::map::Map;
use crate::player::Player;

pub fn run_game() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut map = Map::load("assets/maps/beach_map.txt")?;
    map.spawn_bombs(50);
    let random_position = map.random_position();
    let mut player = Player::new(random_position.0, random_position.1);
    let mut key_pressed = false; // Initialisation de la variable pour suivre l'état de la touche
    let sleep_duration = Duration::from_millis(10);

    loop {
        stdout.execute(cursor::Hide)?;
        map.draw(&mut stdout)?;
        player.draw(&mut stdout)?;
        stdout.flush()?;

        if event::poll(Duration::ZERO)? {
            if let Event::Key(event) = event::read()? {
                if !key_pressed {
                    match event.code {
                        KeyCode::Char('q') => break,
                        _ => {
                            let (potential_new_x, potential_new_y) =
                                player.calculate_new_position(event.code);

                            if !map.is_wall(potential_new_x.into(), potential_new_y.into()) {
                                player.move_player(event.code, &map);
                                key_pressed = true; // Marquer la touche comme pressée
                            }

                            let new_x = player.get_x();
                            let new_y = player.get_y();
                            if map.is_bomb(new_x.into(), new_y.into()) {
                                let random_position = map.random_position();
                                player = Player::new(random_position.0, random_position.1);
                            } else if map.is_door(new_x.into(), new_y.into()) {
                                break;
                            }
                        }
                    }
                }
            }
        } else {
            key_pressed = false; // Aucun événement, marquer la touche comme non pressée
        }

        stdout.execute(cursor::Show)?;
        sleep(sleep_duration);
        stdout.flush()?;
    }

    Ok(())
}
