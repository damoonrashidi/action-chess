use std::{
    collections::{hash_set::Iter, HashSet},
    net::SocketAddr,
};

use state::board::Board;

#[derive(Debug)]
pub struct Game {
    players: HashSet<SocketAddr>,
    pub board: Board,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: HashSet::new(),
            board: Board::new(),
        }
    }

    pub fn add_player(&mut self, player: SocketAddr) {
        self.players.insert(player);
    }

    #[must_use]
    #[allow(unused)]
    pub fn get_players(&self) -> Iter<'_, SocketAddr> {
        self.players.iter()
    }
}
