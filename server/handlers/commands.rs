use super::handler::Handler;
use crate::world::World;
use network::{game_command::GameCmd, marshal::Marshal};
use std::net::SocketAddr;

pub(crate) struct CommandHandler;

impl Handler for CommandHandler {
    fn handle(player: SocketAddr, msg: [u8; 4], world: &mut World) {
        let cmd: GameCmd = msg.into();

        match cmd {
            GameCmd::Join(game_id) => {
                if let Some(game) = world.get_game(&game_id) {
                    for mv in &game.move_history {
                        let _ = world.socket.send_to(&Marshal::command(*mv), player);
                    }
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
