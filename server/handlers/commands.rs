use super::handler::Handler;
use crate::world::state::State;
use network::{game_command::GameCmd, marshal::Marshal};
use std::{
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
};

pub(crate) struct CommandHandler;

impl Handler for CommandHandler {
    fn handle(player: SocketAddr, msg: [u8; 4], world: &Arc<Mutex<State>>, socket: &UdpSocket) {
        let cmd: GameCmd = msg.into();

        match cmd {
            GameCmd::Join(game_id) => {
                let mut world = world.lock().unwrap();
                if let Some(game) = world.get_game_mut(&game_id) {
                    game.made_moves.iter().for_each(|mv| {
                        let _ = socket.send_to(&Marshal::command(*mv), player);
                    });
                    println!("{player} joined {game_id}");
                    world.add_player(player, &game_id);
                } else {
                    println!("{player} created {game_id}");
                    world.create_game(&game_id);
                    world.add_player(player, &game_id);
                }
            }
            GameCmd::Leave => println!("{player} is leaving their game"),
            GameCmd::Resign => println!("{player} is resigning"),
        };
    }
}
