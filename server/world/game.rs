use std::{
    collections::{hash_set::Iter, HashSet},
    net::SocketAddr,
};

use state::board::Board;

pub struct Game {
    players: HashSet<SocketAddr>,
    pub board: Board,
}

impl Game {
    pub fn add_player(&mut self, player: SocketAddr) {
        println!("adding {player} to game");
        self.players.insert(player);
    }

    #[must_use]
    #[allow(unused)]
    pub fn get_players(&self) -> Iter<'_, SocketAddr> {
        self.players.iter()
    }
}
