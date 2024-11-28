use std::{
    collections::{hash_set::Iter, HashSet},
    net::SocketAddr,
};

use state::{board::Board, movegen::MoveGen, piece::Move};

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub move_history: Vec<Move>,
    pub last_processed_at: u64,

    players: HashSet<SocketAddr>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        Self {
            players: HashSet::new(),
            board: Board::standard(),
            move_history: vec![],
            last_processed_at: 0,
        }
    }

    pub fn add_player(&mut self, player: SocketAddr) {
        self.players.insert(player);
    }

    #[allow(unused)]
    pub fn remove_player(&mut self, player: SocketAddr) {
        self.players.remove(&player);
    }

    #[must_use]
    pub fn get_players(&self) -> Iter<'_, SocketAddr> {
        self.players.iter()
    }

    /**
    # Panics
    Can't really panic
    */
    pub fn make_move(&mut self, mv: &Move) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        let diff = now - std::time::Duration::from_micros(self.last_processed_at);

        for y in 0..8 {
            for x in 0..8 {
                if let Some(mut piece) = self.board.pieces[y][x] {
                    let new_cd = piece.get_cooldown().saturating_sub(diff);
                    piece.set_cooldown(new_cd);
                    self.board.pieces[y][x] = Some(piece);
                }
            }
        }

        self.move_history.push(*mv);
        self.board.process_move(*mv);
    }

    #[must_use]
    pub fn is_valid_move(&self, mv: &Move) -> bool {
        MoveGen::new(&self.board).get_possible_moves().contains(mv)
    }
}
