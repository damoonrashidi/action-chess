use std::{
    collections::{hash_set::Iter, HashSet},
    net::SocketAddr,
};

use state::board::Board;

#[derive(Debug)]
pub(crate) struct Game {
    players: HashSet<SocketAddr>,
    #[allow(unused)]
    board: Board,
}

impl Game {
    pub(crate) fn new() -> Self {
        Self {
            players: HashSet::new(),
            board: Board::new(),
        }
    }

    pub(crate) fn add_player(&mut self, player: SocketAddr) {
        self.players.insert(player);
    }

    #[must_use]
    #[allow(unused)]
    pub(crate) fn get_players(&self) -> Iter<'_, SocketAddr> {
        self.players.iter()
    }
}
