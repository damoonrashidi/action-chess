mod world;

use std::{
    io::Result,
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread,
};

use network::{game_command::GameCmd, unmarshal::Unmarshal};
use world::state::State;

fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    let world = Arc::new(Mutex::new(State::new()));

    let thread = thread::spawn(move || -> Result<()> {
        loop {
            let mut msg: [u8; 4] = [0, 0, 0, 0];

            let (_, addr) = socket.recv_from(&mut msg)?;
            if let 0..=3 = msg[0] {
                println!("{}", Unmarshal::command(msg));
            } else {
                let cmd: GameCmd = msg.into();
                match cmd {
                    GameCmd::Join(game_id) => {
                        let mut world = world.lock().unwrap();
                        if let Some(game) = world.get_game_mut(&game_id) {
                            game.add_player(addr);
                        } else {
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

    Ok(())
}
