use std::{
    collections::{hash_set::Iter, HashSet},
    net::SocketAddr,
};

use state::{board::Board, movegen::MoveGen, piece::Move};

#[derive(Debug)]
pub(crate) struct Game {
    pub(crate) board: Board,
    pub(crate) move_history: Vec<Move>,

    players: HashSet<SocketAddr>,
}

impl Game {
    pub(crate) fn new() -> Self {
        Self {
            players: HashSet::new(),
            board: Board::standard(),
            move_history: vec![],
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

    pub(crate) fn make_move(&mut self, mv: &Move) {
        self.move_history.push(*mv);
        self.board.process_move(*mv);
    }

    pub(crate) fn is_valid_move(&self, mv: &Move) -> bool {
        let gen = MoveGen::new(&self.board);
        gen.get_possible_moves().contains(mv)
    }

    pub(crate) fn tick(&mut self) {
        self.board.tick();
    }
}
