use hecs::Bundle;

use super::board::Board;

#[derive(Debug, Clone, Bundle)]
pub struct GameState {
    pub board: Board,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: { Board::new() },
        }
    }
}
