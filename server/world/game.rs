use std::{
    collections::{hash_set::Iter, HashSet},
    net::SocketAddr,
};

use state::{board::Board, piece::Move};

#[derive(Debug)]
pub(crate) struct Game {
    players: HashSet<SocketAddr>,
    board: Board,
}

impl Game {
    pub(crate) fn new() -> Self {
        Self {
            players: HashSet::new(),
            board: Board::standard(),
        }
    }

    pub(crate) fn add_player(&mut self, player: SocketAddr) {
        self.players.insert(player);
    }

    #[allow(unused)]
    pub(crate) fn remove_player(&mut self, player: SocketAddr) {
        self.players.remove(&player);
    }

    pub(crate) fn get_players(&self) -> Iter<'_, SocketAddr> {
        self.players.iter()
    }

    pub(crate) fn make_move(&mut self, mv: Move) {
        self.board.process_move(mv);
    }
}
