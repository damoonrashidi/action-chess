use std::net::SocketAddr;

use crate::world::World;
use network::unmarshal::Unmarshal;
use state::movegen::MoveGen;

use super::handler::Handler;

pub(crate) struct MoveHandler;

impl Handler for MoveHandler {
    fn handle(player: SocketAddr, msg: [u8; 4], world: &mut World) {
        let mv = Unmarshal::command(msg);
        println!("{player} is making move {mv}");
        if let (Ok(socket), Some(game)) = (
            world.socket.try_clone(),
            world.get_game_for_player_mut(&player),
        ) {
            if !game.is_valid_move(&mv) {
                println!("{player} tried to make illegal move {mv}");
                let gen = MoveGen::new(&game.board);
                MoveGen::render_movelist(&game.board, &gen.get_possible_moves());
                return;
            }
            game.make_move(&mv);
            game.get_players().for_each(|participant| {
                println!("sending {mv} to {participant}");
                let _ = socket.send_to(&msg, participant);
            });
        } else {
            println!("could not find an active game for {player}");
        }
    }
}
