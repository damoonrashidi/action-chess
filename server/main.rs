mod world;
use std::{
    io::Result,
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use network::{game_command::GameCmd, unmarshal::Unmarshal};

use world::state::State;

fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    let world = Arc::new(Mutex::new(State::new()));

    let tick_world = Arc::clone(&world);
    let tick_thread = thread::spawn(move || loop {
        if let Ok(mut world) = tick_world.lock() {
            for game in world.games_mut() {
                game.tick();
            }
        }
        sleep(Duration::from_millis(16));
    });

    let thread = thread::spawn(move || -> Result<()> {
        loop {
            let mut msg: [u8; 4] = [0, 0, 0, 0];

            let (_, addr) = socket.recv_from(&mut msg)?;

            if let 0..=3 = msg[0] {
                let mut world = world.lock().unwrap();
                let mv = Unmarshal::command(msg);
                println!("{addr} is making move {mv}");
                if let Some(game) = world.get_game_for_player_mut(&addr) {
                    if game.is_valid_move(&mv) {
                        game.make_move(&mv);
                        game.get_players().for_each(|player| {
                            println!("sending {mv} to {player}");
                            let _ = socket.send_to(&msg, player);
                        });
                    } else {
                        println!("{addr} tried to make illegal move {mv}");
                    }
                }
            } else {
                let cmd: GameCmd = msg.into();
                match cmd {
                    GameCmd::Join(game_id) => {
                        let mut world = world.lock().unwrap();
                        if world.get_game_mut(&game_id).is_some() {
                            println!("{addr} joined {game_id}");
                            world.add_player(addr, &game_id);
                        } else {
                            println!("{addr} created {game_id}");
                            world.create_game(&game_id);
                            world.add_player(addr, &game_id);
                        }
                    }
                    GameCmd::Leave => println!("{addr} is leaving their game"),
                    GameCmd::Resign => println!("{addr} is resigning"),
                };
            }
        }
    });

    let _ = thread.join();
    let _ = tick_thread.join();

    Ok(())
}
