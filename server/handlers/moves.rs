use std::{
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
};

use crate::world::state::State;
use network::unmarshal::Unmarshal;

use super::handler::Handler;

pub(crate) struct MoveHandler;

impl Handler for MoveHandler {
    fn handle(player: SocketAddr, msg: [u8; 4], world: &Arc<Mutex<State>>, socket: &UdpSocket) {
        let mut world = world.lock().unwrap();
        let mv = Unmarshal::command(msg);
        println!("{player} is making move {mv}");
        if let Some(game) = world.get_game_for_player_mut(&player) {
            if !game.is_valid_move(&mv) {
                println!("{player} tried to make illegal move {mv}");
            }
            game.make_move(&mv);
            game.get_players().for_each(|player| {
                println!("sending {mv} to {player}");
                let _ = socket.send_to(&msg, player);
            });
        }
    }
}
