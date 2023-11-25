mod world;
use std::{
    io::Result,
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use network::{game_command::GameCmd, marshal::Marshal, unmarshal::Unmarshal};

use world::state::State;

fn main() -> anyhow::Result<()> {
    let world = Arc::new(Mutex::new(State::new()));

    let tick_handle = tick_world(Arc::clone(&world));

    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    let command_handle = handle_commands(Arc::clone(&world), socket);

    let _ = command_handle.join();
    let _ = tick_handle.join();

    Ok(())
}

fn tick_world(world: Arc<Mutex<State>>) -> JoinHandle<()> {
    thread::spawn(move || loop {
        if let Ok(mut world) = world.lock() {
            for game in world.games_mut() {
                game.tick();
            }
        }
        thread::sleep(Duration::from_millis(16));
    })
}

fn handle_commands(world: Arc<Mutex<State>>, socket: UdpSocket) -> JoinHandle<Result<()>> {
    thread::spawn(move || -> Result<()> {
        loop {
            let mut msg: [u8; 4] = [0, 0, 0, 0];

            let (_, addr) = socket.recv_from(&mut msg)?;

            if let 0..=3 = msg[0] {
                let mut world = world.lock().unwrap();
                let mv = Unmarshal::command(msg);
                println!("{addr} is making move {mv}");
                if let Some(game) = world.get_game_for_player_mut(&addr) {
                    if !game.is_valid_move(&mv) {
                        println!("{addr} tried to make illegal move {mv}");
                        return Ok(());
                    }
                    game.make_move(&mv);
                    game.get_players().for_each(|player| {
                        println!("sending {mv} to {player}");
                        let _ = socket.send_to(&msg, player);
                    });
                }
            } else {
                let cmd: GameCmd = msg.into();
                match cmd {
                    GameCmd::Join(game_id) => {
                        let mut world = world.lock().unwrap();
                        if let Some(game) = world.get_game_mut(&game_id) {
                            game.made_moves.iter().for_each(|mv| {
                                let _ = socket.send_to(&Marshal::command(*mv), addr);
                            });
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
    })
}
