use crossterm::event::{self, Event, KeyCode};
use crossterm::{cursor, ExecutableCommand};
use rand::{thread_rng, Rng};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

use crate::map::Map;
use crate::player::Player;
use crate::utils::{self, GameState, Position};

use std::fs::File;

use bincode;

/// Fonction principale pour exécuter le jeu.
///
/// Arguments
///
/// * `bind_socket` - Une chaîne de caractères représentant l'adresse du socket auquel se connecter.
///
/// Retourne
///
/// * `io::Result<()>` - Un résultat IO indiquant le succès ou l'échec de l'exécution.
pub fn run_game(bind_socket: String) -> io::Result<()> {
    // Chargement de la carte et initialisation du joueur
    let mut stdout = io::stdout();
    let mut map = Map::load("assets/maps/beach_map.txt")?;
    map.spawn_bombs(50);
    let random_position = map.random_position();
    let mut player = Player::new(random_position.0, random_position.1);
    let mut key_pressed = false; // Initialisation de la variable pour suivre l'état de la touche
    let sleep_duration = Duration::from_millis(10);

    let mut log_file = File::create("logs.txt")?;

    let id = thread_rng().gen_range(0..std::u64::MAX);

    let mut state = GameState::new();

    loop {
        stdout.execute(cursor::Hide)?;
        map.draw(&mut stdout)?;
        state.draw(&mut stdout)?;
        player.draw(&mut stdout)?;
        stdout.flush()?;

        // Vérifier les événements clavier
        if event::poll(Duration::ZERO)? {
            if let Event::Key(event) = event::read()? {
                if !key_pressed {
                    match event.code {
                        KeyCode::Char('q') => break,
                        _ => {
                            // Calculer la nouvelle position potentielle du joueur selon la map et les éléments de gameplay
                            let (potential_new_x, potential_new_y) =
                                player.calculate_new_position(event.code);

                            if !map.is_wall(potential_new_x.into(), potential_new_y.into()) {
                                player.move_player(event.code, &map);
                                key_pressed = true;
                            }

                            let new_x = player.get_x();
                            let new_y = player.get_y();
                            if map.is_bomb(new_x.into(), new_y.into()) {
                                let random_position = map.random_position();
                                player = Player::new(random_position.0, random_position.1);
                            } else if map.is_door(new_x.into(), new_y.into()) {
                                break;
                            }

                            // Mise à jour de la position du joueur et envoi au serveur
                            let pos = Position::new(player.get_x(), player.get_y());
                            let pos_update = utils::PositionMessage::new(id, pos);
                            let encoded = bincode::serialize(&pos_update).unwrap();

                            let mut stream = match TcpStream::connect(bind_socket.clone()) {
                                Ok(stream) => stream,
                                Err(e) => {
                                    println!("Failed to connect to server: {}", e);
                                    return Err(e);
                                }
                            };

                            match stream.write(&encoded) {
                                Ok(_) => (),
                                Err(e) => println!("Error sending data : {}", e),
                            }

                            let mut buffer: Vec<u8> = vec![0; 1024];

                            let Ok(n) = stream.read(&mut buffer) else {
                                println!("Error reading data");
                                continue;
                            };
                            let bindata = buffer[..n].as_ref();

                            // Mise à jour de l'état du jeu avec les données reçues du serveur
                            if let Ok(new_state) = bincode::deserialize(&bindata) {
                                state = new_state;
                                log_file.write(format!("{:?}", state).as_bytes())?;
                            } else {
                                println!("Impossible de récuperér le nouveau state.");
                            }
                        }
                    }
                }
            }
        } else {
            key_pressed = false;
        }

        stdout.execute(cursor::Show)?;
        sleep(sleep_duration);
        stdout.flush()?;
    }

    Ok(())
}
